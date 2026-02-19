use crate::error::Result;
use crate::models::{DocStatus, FrontMatter};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;

pub fn run(path: Option<PathBuf>, title: Option<String>, author: Option<String>) -> Result<()> {
    let path = path.unwrap_or_else(|| PathBuf::from("document.tmd"));

    if path.exists() {
        return Err(crate::error::RhodiError::Resolution(format!(
            "File '{}' already exists",
            path.display()
        )));
    }

    let title = title.unwrap_or_else(|| "Untitled Document".to_string());
    let author = author.unwrap_or_else(|| "Anonymous".to_string());

    let frontmatter = FrontMatter {
        id: uuid::Uuid::now_v7(),
        title: title.clone(),
        author: Some(author.clone()),
        doc_status: DocStatus::Draft,
        created_at: Utc::now(),
        ..Default::default()
    };

    let fm_yaml = serde_norway::to_string(&frontmatter).map_err(|e| {
        crate::error::RhodiError::Serialization(format!("Failed to serialize frontmatter: {}", e))
    })?;

    let content = format!(
        "---\n{}\n---\n\n# {}\n\nStart writing your document here.\n",
        fm_yaml.trim(),
        title
    );

    fs::write(&path, content)?;

    println!("Created new document: {}", path.display());
    println!("  Title: {}", title);
    println!("  Author: {}", author);
    println!("  Status: Draft");

    Ok(())
}
