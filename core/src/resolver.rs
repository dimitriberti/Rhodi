use crate::error::{Result, RhodiError, SecurityError};
use crate::models::TracedDocument;
use std::fs;
use std::path::{Path, PathBuf};

pub trait SourceResolver {
    /// Resolve a source (path or URL) to its content (bytes).
    fn resolve_bytes(&self, source: &str) -> Result<Vec<u8>>;

    /// Resolve a source to a parsed Document (for includes).
    fn resolve_document(&self, source: &str) -> Result<TracedDocument>;
}

pub struct FileResolver {
    root: PathBuf,
}

impl FileResolver {
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Self> {
        let root = root.as_ref().canonicalize()?;
        Ok(Self { root })
    }

    fn validate_path(&self, source: &str) -> Result<PathBuf> {
        let path = Path::new(source);

        // 1. Reject absolute paths
        if path.is_absolute() {
            return Err(RhodiError::Security(SecurityError::PathTraversal {
                path: path.to_path_buf(),
                root: self.root.clone(),
            }));
        }

        // 2. Check for ".." components that go above root
        let mut depth: i32 = 0;
        for component in path.components() {
            match component {
                std::path::Component::Normal(_) => depth += 1,
                std::path::Component::ParentDir => {
                    depth -= 1;
                    if depth < 0 {
                        return Err(RhodiError::Security(SecurityError::PathTraversal {
                            path: path.to_path_buf(),
                            root: self.root.clone(),
                        }));
                    }
                }
                _ => {}
            }
        }

        // 3. Join and canonicalize
        let full_path = self.root.join(path);

        // For existing files, we also check canonical path as a second layer of defense (symlinks)
        if full_path.exists() {
            let canonical = full_path.canonicalize()?;
            if !canonical.starts_with(&self.root) {
                return Err(RhodiError::Security(SecurityError::PathTraversal {
                    path: canonical,
                    root: self.root.clone(),
                }));
            }
            Ok(canonical)
        } else {
            Ok(full_path)
        }
    }
}

impl SourceResolver for FileResolver {
    fn resolve_bytes(&self, source: &str) -> Result<Vec<u8>> {
        let safe_path = self.validate_path(source)?;
        fs::read(safe_path).map_err(Into::into)
    }

    fn resolve_document(&self, source: &str) -> Result<TracedDocument> {
        let bytes = self.resolve_bytes(source)?;
        let content = String::from_utf8(bytes)
            .map_err(|e| RhodiError::Format(format!("Invalid UTF-8 in document: {}", e)))?;
        crate::markdown::parse_tmd(&content)
    }
}
