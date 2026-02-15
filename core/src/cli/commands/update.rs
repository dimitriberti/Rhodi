use crate::error::Result;
use crate::markdown::parse_tmd;
use std::fs;
use std::path::PathBuf;

pub fn run(path: PathBuf) -> Result<()> {
    let content = fs::read_to_string(&path)?;
    let mut doc = parse_tmd(&content)?;

    let base_path = if let Some(p) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        p.to_path_buf()
    } else {
        std::env::current_dir()?
    };

    doc.update_all_traces(&base_path)?;

    let fm_yaml = serde_norway::to_string(&doc.frontmatter).map_err(|e| {
        crate::error::RhodiError::Serialization(format!("Failed to serialize frontmatter: {}", e))
    })?;

    let full_content = format!("---\n{}---\n\n{}", fm_yaml.trim(), doc.body);
    fs::write(&path, full_content)?;

    Ok(())
}
