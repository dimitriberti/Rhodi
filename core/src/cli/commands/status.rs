use crate::error::Result;
use crate::markdown::parse_tmd;
use crate::version::{get_version_status, VersionStatus};
use std::fs;
use std::path::PathBuf;

pub fn run(path: PathBuf) -> Result<()> {
    let content = fs::read_to_string(&path)?;
    let doc = parse_tmd(&content)?;

    println!("Document: {}", path.display());
    println!("{}", "=".repeat(50));
    println!("Title:      {}", doc.frontmatter.title);
    println!("ID:         {}", doc.frontmatter.id);
    println!(
        "Author:     {}",
        doc.frontmatter.author.as_deref().unwrap_or("(none)")
    );
    println!("Status:     {:?}", doc.frontmatter.doc_status);
    println!(
        "Created:    {}",
        doc.frontmatter.created_at.format("%Y-%m-%d %H:%M:%S UTC")
    );

    if let Some(modified) = doc.frontmatter.modified_at {
        println!("Modified:   {}", modified.format("%Y-%m-%d %H:%M:%S UTC"));
    }

    println!("Protocol Version: {}", doc.frontmatter.protocol_version);
    let version_status = get_version_status(&doc.frontmatter.protocol_version);
    println!(
        "Protocol Status:  {:?}",
        match version_status {
            VersionStatus::Current => "Current",
            VersionStatus::Deprecated => "Deprecated",
            VersionStatus::Obsolete => "Obsolete",
        }
    );
    println!("Document Version: {}", doc.frontmatter.doc_version);
    if doc.frontmatter.doc_version > 0
        && let Some(ref prev_hash) = doc.frontmatter.prev_version_hash
    {
        println!("Previous Hash:    {}", hex::encode(prev_hash));
    }

    if let Some(ref pk) = doc.frontmatter.public_key {
        println!("Public Key: {}", pk);
    }

    if let Some(ref hash) = doc.frontmatter.version_hash {
        println!("Version Hash: {}", hex::encode(hash));
    } else {
        println!("Version Hash: (not set)");
    }

    if doc.frontmatter.signature.is_some() {
        println!("Signature:  Present ✓");
    } else {
        println!("Signature:  (not set)");
    }

    println!("{}", "=".repeat(50));
    println!("Body length: {} characters", doc.body.len());

    Ok(())
}
