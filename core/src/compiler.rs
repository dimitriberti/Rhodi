use crate::crypto::KeyPair;
use crate::error::{Result, RhodiError, SecurityError};
use crate::markdown::{parse_tmd_sections, Section};
use crate::models::{DocStatus, TraceBlock, TracedDocument};
use crate::resolver::SourceResolver;
use ed25519_dalek::VerifyingKey;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::path::PathBuf;

pub const MAX_INCLUDE_DEPTH: usize = 5;

pub struct Compiler<'a, R: SourceResolver> {
    resolver: &'a R,
}

#[derive(Debug, Default)]
pub struct CompilationReport {
    pub errors: Vec<RhodiError>,
    pub warnings: Vec<String>,
}

#[derive(Deserialize)]
struct IncludeBlock {
    path: String,
    #[allow(dead_code)]
    integrity: Option<String>,
}

impl<'a, R: SourceResolver> Compiler<'a, R> {
    pub fn new(resolver: &'a R) -> Self {
        Self { resolver }
    }

    pub fn create(&self, title: &str, content: &str) -> TracedDocument {
        TracedDocument::new(title, content)
    }

    pub fn update(
        &self,
        mut doc: TracedDocument,
        _keypair: Option<&KeyPair>,
    ) -> Result<TracedDocument> {
        doc = doc.update_modified_time();
        if doc.frontmatter.doc_status == DocStatus::Published
            || doc.frontmatter.doc_status == DocStatus::Revoked
        {
            doc.frontmatter.doc_status = DocStatus::Draft;
            doc.frontmatter.signature = None;
            doc.frontmatter.version_hash = None;
        }
        Ok(doc)
    }

    pub fn publish(&self, doc: TracedDocument, keypair: &KeyPair) -> Result<TracedDocument> {
        let report = self.verify(&doc)?;
        if !report.errors.is_empty() {
            return Err(RhodiError::Verification(format!(
                "Cannot publish due to verification errors: {:?}",
                report.errors
            )));
        }
        Ok(doc.seal(keypair))
    }

    pub fn revoke(&self, mut doc: TracedDocument, keypair: &KeyPair) -> Result<TracedDocument> {
        doc.frontmatter.doc_status = DocStatus::Revoked;
        doc = doc.update_modified_time();
        Ok(doc.seal(keypair))
    }

    pub fn verify(&self, doc: &TracedDocument) -> Result<CompilationReport> {
        let mut seen = HashSet::new();
        // We don't have a reliable unique ID for the initial doc if it's not saved,
        // but recursion guard will handle it.
        self.verify_recursive(doc, 0, &mut seen)
    }

    fn verify_recursive(
        &self,
        doc: &TracedDocument,
        depth: usize,
        seen: &mut HashSet<String>,
    ) -> Result<CompilationReport> {
        let mut report = CompilationReport::default();

        if depth > MAX_INCLUDE_DEPTH {
            return Err(RhodiError::Security(SecurityError::MaxRecursionDepth {
                depth: MAX_INCLUDE_DEPTH,
            }));
        }

        // 1. Verify integrity/signature
        if doc.frontmatter.doc_status == DocStatus::Published
            || doc.frontmatter.doc_status == DocStatus::Revoked
        {
            if let Some(pk_hex) = &doc.frontmatter.public_key {
                if let Ok(pk_bytes) = hex::decode(pk_hex) {
                    if let Ok(pk) = VerifyingKey::from_bytes(
                        &pk_bytes
                            .try_into()
                            .map_err(|_| RhodiError::Crypto("Invalid key len".to_string()))?,
                    ) {
                        if let Err(e) = doc.verify(&pk) {
                            report.errors.push(e);
                        }
                    } else {
                        report
                            .errors
                            .push(RhodiError::Crypto("Invalid public key format".to_string()));
                    }
                } else {
                    report.errors.push(RhodiError::Crypto(
                        "Public key is not valid hex".to_string(),
                    ));
                }
            } else {
                report.warnings.push(
                    "No public key found in metadata, skipping signature verification".to_string(),
                );
            }
        }

        // 2. Recursive verification
        let sections = parse_tmd_sections(&doc.body);
        for section in sections {
            match section {
                Section::Trace(trace) => {
                    if let Err(e) = self.verify_trace(&trace) {
                        if doc.frontmatter.doc_status == DocStatus::Published {
                            report.errors.push(e);
                        } else {
                            report.warnings.push(format!("Trace warning: {}", e));
                        }
                    }
                }
                Section::Include(content) => {
                    let yaml_content = content
                        .trim()
                        .trim_start_matches("```include")
                        .trim_end_matches("```")
                        .trim();
                    match serde_norway::from_str::<IncludeBlock>(yaml_content) {
                        Ok(include) => {
                            // Cycle detection
                            if seen.contains(&include.path) {
                                return Err(RhodiError::Security(SecurityError::CircularInclude {
                                    path: PathBuf::from(&include.path),
                                }));
                            }
                            seen.insert(include.path.clone());

                            match self.resolver.resolve_document(&include.path) {
                                Ok(included_doc) => {
                                    if !included_doc.frontmatter.policy.allow_include {
                                        report.errors.push(RhodiError::Verification(format!(
                                            "Document {} does not allow inclusion",
                                            include.path
                                        )));
                                    }

                                    let sub_report =
                                        self.verify_recursive(&included_doc, depth + 1, seen)?;
                                    report.errors.extend(sub_report.errors);
                                    report.warnings.extend(sub_report.warnings);
                                }
                                Err(e) => {
                                    report.errors.push(RhodiError::Resolution(format!(
                                        "Failed to resolve include {}: {}",
                                        include.path, e
                                    )));
                                }
                            }
                            seen.remove(&include.path);
                        }
                        Err(e) => {
                            report
                                .errors
                                .push(RhodiError::Format(format!("Invalid include block: {}", e)));
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(report)
    }

    fn verify_trace(&self, trace: &TraceBlock) -> Result<()> {
        let content = self.resolver.resolve_bytes(&trace.source)?;

        // 1. Verify hash if present
        if let Some(expected_hash) = &trace.hash {
            let mut hasher = Sha256::new();
            hasher.update(&content);
            let result = hasher.finalize();
            let computed_hash = format!("sha256:{}", hex::encode(result));

            if &computed_hash != expected_hash {
                return Err(RhodiError::Verification(format!(
                    "Hash mismatch for {}. Expected {}, got {}",
                    trace.source, expected_hash, computed_hash
                )));
            }
        }

        // 2. Truth extraction if selector is present
        if let Some(selector) = &trace.selector {
            let extractor_method = trace.extractor.as_deref().unwrap_or("regex");
            let extractor = crate::extraction::get_extractor(extractor_method)?;
            let extracted_value = extractor.extract(&content, selector)?;

            if extracted_value.trim() != trace.expected.trim() {
                return Err(RhodiError::Verification(format!(
                    "Truth verification failed for {}. Expected '{}', got '{}'",
                    trace.source, trace.expected, extracted_value
                )));
            }
        }

        Ok(())
    }
}
