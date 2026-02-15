use crate::compiler::{CompilationReport, Compiler};
use crate::error::Result;
use crate::markdown::parse_tmd;
use crate::resolver::FileResolver;
use std::fs;
use std::path::PathBuf;

pub fn run(path: PathBuf, strict: bool) -> Result<CompilationReport> {
    let content = fs::read_to_string(&path)?;
    let doc = parse_tmd(&content)?;

    let base_path = if let Some(p) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        p.to_path_buf()
    } else {
        std::env::current_dir()?
    };

    let resolver = FileResolver::new(&base_path)?;
    let compiler = Compiler::new(&resolver);

    let report = compiler.verify(&doc)?;

    if strict && !report.errors.is_empty() {
        return Err(crate::error::RhodiError::Verification(format!(
            "Verification failed with {} error(s)",
            report.errors.len()
        )));
    }

    Ok(report)
}
