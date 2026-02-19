use crate::cli::keys::KeyManager;
use crate::crypto::KeyPair;
use crate::error::Result;
use crate::markdown::parse_tmd;
use crate::models::DocStatus;
use std::fs;
use std::path::PathBuf;

pub fn run(path: PathBuf, key_name: Option<String>) -> Result<()> {
    let key_name = key_name.unwrap_or_else(|| "default".to_string());

    let content = fs::read_to_string(&path)?;
    let mut doc = parse_tmd(&content)?;

    if doc.frontmatter.doc_status == DocStatus::Published {
        return Err(crate::error::RhodiError::Verification(
            "Document is already published. Create a new version instead.".into(),
        ));
    }

    let base_path = if let Some(p) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        p.to_path_buf()
    } else {
        std::env::current_dir()?
    };

    doc.update_all_traces(&base_path)?;

    let manager = KeyManager::new()?;
    let signing_key = manager.get_key(&key_name)?;
    let verifying_key = signing_key.verifying_key();

    let keypair = KeyPair {
        signing_key,
        verifying_key,
    };

    doc.frontmatter.public_key = Some(hex::encode(keypair.verifying_key.as_bytes()));

    doc = doc.seal(&keypair);

    let fm_yaml = serde_norway::to_string(&doc.frontmatter).map_err(|e| {
        crate::error::RhodiError::Serialization(format!("Failed to serialize frontmatter: {}", e))
    })?;

    let full_content = format!("---\n{}\n---\n\n{}", fm_yaml.trim(), doc.body);
    fs::write(&path, full_content)?;

    println!("Document sealed successfully: {}", path.display());
    println!("  Status: Published");
    println!(
        "  Version hash: {}",
        hex::encode(doc.frontmatter.version_hash.unwrap())
    );
    println!("  Protocol version: {}", doc.frontmatter.protocol_version);
    println!("  Document version: {}", doc.frontmatter.doc_version);

    Ok(())
}
