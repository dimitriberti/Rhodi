use crate::error::{Result, RhodiError};
use directories::ProjectDirs;
use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const KEY_DIR_NAME: &str = "keys";

#[derive(Serialize, Deserialize)]
pub struct KeyFile {
    pub name: String,
    pub public_key: String,
    signing_key: String,
}

pub struct KeyManager {
    keys_dir: PathBuf,
}

impl KeyManager {
    pub fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("com", "rhodi", "rhodi")
            .ok_or_else(|| RhodiError::Resolution("Could not determine config directory".into()))?;

        let keys_dir = proj_dirs.config_dir().join(KEY_DIR_NAME);

        if !keys_dir.exists() {
            fs::create_dir_all(&keys_dir)?;
        }

        Ok(Self { keys_dir })
    }

    pub fn get_key(&self, name: &str) -> Result<SigningKey> {
        let key_path = self.keys_dir.join(format!("{}.json", name));

        if !key_path.exists() {
            return Err(RhodiError::Resolution(format!(
                "Key '{}' not found. Run 'rhodi keygen --name {}' to create it.",
                name, name
            )));
        }

        let content = fs::read_to_string(&key_path)?;
        let key_file: KeyFile = serde_json::from_str(&content)
            .map_err(|e| RhodiError::Format(format!("Invalid key file: {}", e)))?;

        let sk_bytes = hex::decode(&key_file.signing_key)
            .map_err(|e| RhodiError::Crypto(format!("Invalid hex in key file: {}", e)))?;

        let sk_bytes: [u8; 32] = sk_bytes
            .try_into()
            .map_err(|_| RhodiError::Crypto("Invalid key length".into()))?;

        Ok(SigningKey::from_bytes(&sk_bytes))
    }

    pub fn get_public_key_hex(&self, name: &str) -> Result<String> {
        let key_path = self.keys_dir.join(format!("{}.json", name));

        if !key_path.exists() {
            return Err(RhodiError::Resolution(format!("Key '{}' not found", name)));
        }

        let content = fs::read_to_string(&key_path)?;
        let key_file: KeyFile = serde_json::from_str(&content)
            .map_err(|e| RhodiError::Format(format!("Invalid key file: {}", e)))?;

        Ok(key_file.public_key)
    }

    pub fn list_keys(&self) -> Result<Vec<String>> {
        let mut keys = Vec::new();

        for entry in fs::read_dir(&self.keys_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json")
                && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
            {
                keys.push(stem.to_string());
            }
        }

        Ok(keys)
    }

    pub fn set_key_permissions(path: &PathBuf) -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
        }
        Ok(())
    }
}

pub fn generate_key(name: &str, show: bool) -> Result<KeyFile> {
    let manager = KeyManager::new()?;

    let key_path = manager.keys_dir.join(format!("{}.json", name));

    if key_path.exists() {
        return Err(RhodiError::Resolution(format!(
            "Key '{}' already exists. Use a different name or delete the existing key.",
            name
        )));
    }

    let mut csprng = rand::rngs::OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();

    let key_file = KeyFile {
        name: name.to_string(),
        public_key: hex::encode(verifying_key.as_bytes()),
        signing_key: hex::encode(signing_key.to_bytes()),
    };

    let content = serde_json::to_string_pretty(&key_file)
        .map_err(|e| RhodiError::Serialization(format!("Failed to serialize key: {}", e)))?;

    fs::write(&key_path, content)?;
    KeyManager::set_key_permissions(&key_path)?;

    if show {
        println!("Key '{}' created successfully.", name);
        println!("Public key (share this): {}", key_file.public_key);
    }

    Ok(key_file)
}
