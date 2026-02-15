//! Core implementation of the document tracing library.
//!
//! This library provides the fundamental structures and functionalities for creating and managing traced documents.

pub mod cli;
pub mod compiler;
pub mod crypto;
pub mod error;
pub mod extraction;
pub mod markdown;
pub mod models;
pub mod resolver;
pub mod version;

pub use crypto::KeyPair;
pub use error::{Result, RhodiError};
pub use markdown::{parse_tmd, parse_trace_block};
pub use models::{DocStatus, FrontMatter, TracedDocument};
pub use version::{get_version_status, get_latest_version, is_version_known, VersionStatus};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{DocStatus, TraceBlock};
    #[test]
    fn create_traced_document() {
        let doc = TracedDocument::new("Test Title", "This is the body of the document.")
            .author("John Doe")
            .set_status(DocStatus::Draft)
            .update_modified_time()
            .extra_info("category", "test");
        print!("{:#?}", doc);
    }

    #[test]
    fn test_signature_flow() {
        let keypair = KeyPair::generate();
        let doc = TracedDocument::new("Signed Doc", "Content");
        let signature = keypair.sign(doc.body.as_bytes());
        let doc = doc.set_signature(signature);

        assert!(doc.frontmatter.signature.is_some());
    }

    #[test]
    fn test_markdown_parsing() {
        let content = r#"---
id: 019b407a-29c7-7752-9284-ca6406bb08cc
title: Test Document
created_at: 2025-12-21T10:35:06Z
doc_status: draft
---
This is the body."#;

        let doc = parse_tmd(content).unwrap();
        assert_eq!(doc.frontmatter.title, "Test Document");
        assert_eq!(doc.body, "This is the body.");
        assert_eq!(doc.frontmatter.doc_status, DocStatus::Draft);
    }

    #[test]
    fn test_trace_block_parsing() {
        let block = r#"```trace
source: ./data.csv
expected: "85%"
method: agent
agent_metadata:
  model: gpt-4o
```"#;
        let trace = parse_trace_block(block).unwrap();
        assert_eq!(trace.source, "./data.csv");
        assert_eq!(trace.expected, "85%");
        assert_eq!(trace.method, crate::models::TraceMethod::Agent);
        assert_eq!(trace.agent_metadata.unwrap().model, "gpt-4o");
    }

    #[test]
    fn test_full_seal_and_verify_workflow() {
        let keypair = KeyPair::generate();
        let mut doc = TracedDocument::new("Final Report", "This is a verified claim.\n\n```trace\nsource: evidence.md\nexpected: \"Verified\"\n```\n");

        // Create a dummy evidence file
        let evidence_path = std::env::current_dir().unwrap().join("evidence.md");
        std::fs::write(&evidence_path, "Verified").unwrap();

        // 1. Update traces
        doc.update_all_traces(&std::env::current_dir().unwrap())
            .unwrap();

        // 2. Seal the document
        doc = doc.seal(&keypair);

        assert_eq!(doc.frontmatter.doc_status, DocStatus::Published);
        assert!(doc.frontmatter.version_hash.is_some());
        assert!(doc.frontmatter.signature.is_some());

        // 3. Verify the document
        doc.verify(&keypair.verifying_key)
            .expect("Verification should pass");

        // 4. Tamper with the body and verify it fails
        doc.body.push_str("Tampered!");
        doc.verify(&keypair.verifying_key)
            .expect_err("Verification should fail after tampering");

        // Clean up
        std::fs::remove_file(evidence_path).unwrap();
    }

    #[test]
    fn test_trace_serialization_consistency() {
        let trace = TraceBlock {
            source: "test.md".to_string(),
            hash: None,
            selector: None,
            expected: "Value".to_string(),
            method: crate::models::TraceMethod::Automatic,
            extractor: None,
            timestamp: None,
            context: None,
            confidence: None,
            agent_metadata: None,
        };

        let yaml = serde_norway::to_string(&trace).unwrap();
        let block = format!("```trace\n{}```\n", yaml);
        let parsed = parse_trace_block(&block).unwrap();

        assert_eq!(trace.source, parsed.source);
        assert_eq!(trace.expected, parsed.expected);
    }

    #[test]
    fn test_extraction_logic() {
        use crate::extraction::{Extractor, JsonPathExtractor, RegexExtractor};

        // 1. Regex Test
        let text = b"The value is 42 units.";
        let regex_extractor = RegexExtractor;
        let res = regex_extractor.extract(text, r"value is (\d+)").unwrap();
        assert_eq!(res, "42");

        // 2. JSONPath Test
        let json = b"{\"stats\": {\"count\": 100}}";
        let json_extractor = JsonPathExtractor;
        let res = json_extractor.extract(json, "$.stats.count").unwrap();
        assert_eq!(res, "100");
    }

    #[test]
    fn test_path_traversal_protection() {
        use crate::resolver::{FileResolver, SourceResolver};
        let temp_dir = std::env::temp_dir();
        let resolver = FileResolver::new(&temp_dir).unwrap();

        // Should fail for absolute path
        let res = resolver.resolve_bytes("/etc/passwd");
        assert!(matches!(res, Err(RhodiError::Security(_))));

        // Should fail for path outside root
        let res = resolver.resolve_bytes("../secret.txt");
        assert!(matches!(res, Err(RhodiError::Security(_))));
    }

    #[test]
    fn test_recursion_guard() {
        use crate::compiler::Compiler;
        use crate::error::Result as ResolverResult;
        use crate::models::TracedDocument;
        use crate::resolver::SourceResolver;

        struct DeepResolver;
        impl SourceResolver for DeepResolver {
            fn resolve_bytes(&self, _s: &str) -> ResolverResult<Vec<u8>> {
                Ok(vec![])
            }
            fn resolve_document(&self, s: &str) -> ResolverResult<TracedDocument> {
                let depth: usize = s.parse().unwrap_or(0);
                Ok(TracedDocument::new(
                    "Recursive",
                    &format!("```include\npath: {}\n```", depth + 1),
                ))
            }
        }

        let resolver = DeepResolver;
        let compiler = Compiler::new(&resolver);
        let doc = TracedDocument::new("Root", "```include\npath: 1\n```");

        let res = compiler.verify(&doc);
        assert!(matches!(
            res,
            Err(RhodiError::Security(
                crate::error::SecurityError::MaxRecursionDepth { .. }
            ))
        ));
    }

    #[test]
    fn test_canonicalize_text_strips_control_characters() {
        use crate::markdown::canonicalize_text;

        // Test ASCII control characters are removed
        let input = "Hello\x00World\x07Test\n";
        let result = canonicalize_text(input);
        assert!(!result.contains('\x00'));
        assert!(!result.contains('\x07'));
        assert!(result.contains("Hello"));
        assert!(result.contains("World"));

        // Test BOM (Unicode Cf) is removed
        let input_with_bom = "\u{FEFF}Hello World\n";
        let result = canonicalize_text(input_with_bom);
        assert!(!result.contains('\u{FEFF}'));
        assert!(result.contains("Hello World"));

        // Test zero-width space (Unicode Cf) is removed
        let input_with_zwsp = "Hello\u{200B}World\n";
        let result = canonicalize_text(input_with_zwsp);
        assert!(!result.contains('\u{200B}'));
        assert!(result.contains("HelloWorld"));

        // Test tabs and newlines are preserved
        let input_with_tabs = "Hello\tWorld\nSecond\nLine\n";
        let result = canonicalize_text(input_with_tabs);
        assert!(result.contains('\t'));
        assert!(result.contains("Hello\tWorld"));

        // Test trailing whitespace is stripped
        let input_trailing = "Hello World   \n";
        let result = canonicalize_text(input_trailing);
        assert!(!result.contains("   \n"));
        assert!(result.ends_with("Hello World\n"));
    }
}
