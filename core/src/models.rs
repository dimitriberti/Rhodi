use crate::error::{Result, RhodiError};
use crate::version::{
    get_version_status, is_version_known, VersionStatus, DEFAULT_PROTOCOL_VERSION,
};
use chrono::{DateTime, Utc};
use ed25519_dalek::Signature;
use serde::{Deserialize, Serialize, Serializer};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

fn serialize_hex<S>(bytes: &Option<[u8; 32]>, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match bytes {
        Some(b) => serializer.serialize_str(&hex::encode(b)),
        None => serializer.serialize_none(),
    }
}

fn serialize_signature<S>(
    sig: &Option<Signature>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match sig {
        Some(s) => serializer.serialize_str(&hex::encode(s.to_bytes())),
        None => serializer.serialize_none(),
    }
}

fn deserialize_hex<'de, D>(deserializer: D) -> std::result::Result<Option<[u8; 32]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(hex_str) => {
            let bytes =
                hex::decode(&hex_str).map_err(|e| serde::de::Error::custom(e.to_string()))?;
            let arr: [u8; 32] = bytes
                .try_into()
                .map_err(|_| serde::de::Error::custom("Invalid hash length: expected 32 bytes"))?;
            Ok(Some(arr))
        }
        None => Ok(None),
    }
}

fn deserialize_signature<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<Signature>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(hex_str) => {
            let bytes =
                hex::decode(&hex_str).map_err(|e| serde::de::Error::custom(e.to_string()))?;
            let arr: [u8; 64] = bytes.try_into().map_err(|_| {
                serde::de::Error::custom("Invalid signature length: expected 64 bytes")
            })?;
            Ok(Some(Signature::from_bytes(&arr)))
        }
        None => Ok(None),
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocStatus {
    #[default]
    Notes, // Do not require any hashing
    Draft,     // Hashing required but not implemented
    Published, // Properly signed and hashed
    Revoked,   // Document has been revoked
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontMatter {
    pub id: Uuid,
    #[serde(
        serialize_with = "serialize_hex",
        deserialize_with = "deserialize_hex",
        default
    )]
    pub version_hash: Option<[u8; 32]>,
    pub title: String,
    pub author: Option<String>,
    /// Hex-encoded Ed25519 public key of the author
    pub public_key: Option<String>,
    #[serde(
        serialize_with = "serialize_signature",
        deserialize_with = "deserialize_signature",
        default
    )]
    pub signature: Option<Signature>,
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub doc_status: DocStatus,
    #[serde(default)]
    pub policy: Policy,
    #[serde(default = "default_protocol_version")]
    pub protocol_version: String,
    #[serde(default)]
    pub doc_version: u32,
    #[serde(
        serialize_with = "serialize_hex",
        deserialize_with = "deserialize_hex",
        default
    )]
    pub prev_version_hash: Option<[u8; 32]>,
    pub extra: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Policy {
    #[serde(default = "default_true")]
    pub allow_include: bool,
    #[serde(default = "default_true")]
    pub allow_quote: bool,
    #[serde(default = "default_false")]
    pub require_attribution: bool,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            allow_include: true,
            allow_quote: true,
            require_attribution: false,
        }
    }
}

fn default_true() -> bool {
    true
}
fn default_false() -> bool {
    false
}
fn default_protocol_version() -> String {
    DEFAULT_PROTOCOL_VERSION.to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentMetadata {
    pub model: String,
    pub prompt_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TraceMethod {
    Automatic,
    Manual,
    Agent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TraceBlock {
    pub source: String,
    pub hash: Option<String>,
    pub selector: Option<String>,
    pub expected: String,
    #[serde(default = "default_trace_method")]
    pub method: TraceMethod,
    pub extractor: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
    pub context: Option<String>,
    pub confidence: Option<f64>,
    pub agent_metadata: Option<AgentMetadata>,
}

impl TraceBlock {
    /// Update the hash of the source file.
    /// Currently supports local files.
    pub fn update_hash(&mut self, base_path: &Path) -> Result<()> {
        let path = base_path.join(&self.source);
        if !path.exists() {
            return Err(RhodiError::Resolution(format!(
                "Source file not found: {:?}",
                path
            )));
        }

        let content = fs::read(&path)?;

        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = hasher.finalize();

        self.hash = Some(format!("sha256:{}", hex::encode(result)));
        Ok(())
    }
}

fn default_trace_method() -> TraceMethod {
    TraceMethod::Automatic
}

impl Default for FrontMatter {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            version_hash: None,
            title: "Untitled".to_string(),
            author: None,
            public_key: None,
            signature: None,
            created_at: Utc::now(),
            modified_at: None,
            doc_status: DocStatus::Notes,
            policy: Policy::default(),
            protocol_version: DEFAULT_PROTOCOL_VERSION.to_string(),
            doc_version: 0,
            prev_version_hash: None,
            extra: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TracedDocument {
    pub frontmatter: FrontMatter,
    pub body: String,
}

impl TracedDocument {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            frontmatter: FrontMatter {
                title: title.to_string(),
                ..FrontMatter::default()
            },
            body: content.trim().to_string(),
        }
    }

    pub fn with_options<E>(title: &str, content: &str, extra: E) -> Self
    where
        E: Into<Option<BTreeMap<String, String>>>,
    {
        Self {
            frontmatter: FrontMatter {
                title: title.to_string(),
                extra: extra.into(),
                ..FrontMatter::default()
            },
            body: content.trim().to_string(),
        }
    }

    pub fn author(mut self, author: &str) -> Self {
        self.frontmatter.author = Some(author.to_string());
        self
    }

    pub fn set_status(mut self, status: DocStatus) -> Self {
        self.frontmatter.doc_status = status;
        self
    }

    pub fn update_modified_time(mut self) -> Self {
        self.frontmatter.modified_at = Some(Utc::now());
        self
    }

    pub fn extra_info(mut self, key: &str, value: &str) -> Self {
        if let Some(ref mut extra_map) = self.frontmatter.extra {
            extra_map.insert(key.to_string(), value.to_string());
        } else {
            let mut new_map = BTreeMap::new();
            new_map.insert(key.to_string(), value.to_string());
            self.frontmatter.extra = Some(new_map);
        }
        self
    }

    pub fn set_version_hash(mut self, hash: [u8; 32]) -> Self {
        self.frontmatter.version_hash = Some(hash);
        self
    }

    pub fn set_signature(mut self, signature: Signature) -> Self {
        self.frontmatter.signature = Some(signature);
        self
    }

    /// Compute the SHA-256 hash of the document for integrity.
    /// This hashes the canonicalized body and the frontmatter (excluding version_hash and signature).
    pub fn compute_version_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        // 1. Hash the canonicalized body
        let canonical_body = crate::markdown::canonicalize_text(&self.body);
        hasher.update(canonical_body.as_bytes());

        // 2. Hash the frontmatter (excluding version_hash and signature)
        // We use a temporary BTreeMap to ensure sorted keys for deterministic hashing
        let mut fm_map = BTreeMap::new();
        fm_map.insert("id", self.frontmatter.id.to_string());
        fm_map.insert("title", self.frontmatter.title.clone());
        if let Some(ref author) = self.frontmatter.author {
            fm_map.insert("author", author.clone());
        }
        if let Some(ref pk) = self.frontmatter.public_key {
            fm_map.insert("public_key", pk.clone());
        }

        // Policy fields
        fm_map.insert(
            "policy_allow_include",
            self.frontmatter.policy.allow_include.to_string(),
        );
        fm_map.insert(
            "policy_allow_quote",
            self.frontmatter.policy.allow_quote.to_string(),
        );
        fm_map.insert(
            "policy_require_attribution",
            self.frontmatter.policy.require_attribution.to_string(),
        );

        fm_map.insert("created_at", self.frontmatter.created_at.to_rfc3339());
        if let Some(ref modified_at) = self.frontmatter.modified_at {
            fm_map.insert("modified_at", modified_at.to_rfc3339());
        }
        fm_map.insert(
            "doc_status",
            format!("{:?}", self.frontmatter.doc_status).to_lowercase(),
        );

        // Version fields
        fm_map.insert(
            "protocol_version",
            self.frontmatter.protocol_version.clone(),
        );
        fm_map.insert("doc_version", self.frontmatter.doc_version.to_string());
        if let Some(ref prev_hash) = self.frontmatter.prev_version_hash {
            fm_map.insert("prev_version_hash", hex::encode(prev_hash));
        }

        if let Some(ref extra) = self.frontmatter.extra {
            for (k, v) in extra {
                fm_map.insert(k, v.clone());
            }
        }

        let fm_json = serde_json::to_string(&fm_map).unwrap();
        hasher.update(fm_json.as_bytes());

        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }

    /// Seal the document by computing the version hash and signing it.
    /// This sets the status to Published unless it is already Revoked.
    pub fn seal(mut self, keypair: &crate::crypto::KeyPair) -> Self {
        if self.frontmatter.doc_status != DocStatus::Revoked {
            self.frontmatter.doc_status = DocStatus::Published;
        }
        self.frontmatter.modified_at = Some(Utc::now());

        // Chain previous version hash
        if let Some(current_hash) = self.frontmatter.version_hash {
            self.frontmatter.prev_version_hash = Some(current_hash);
        }

        // Increment document version
        self.frontmatter.doc_version += 1;

        let hash = self.compute_version_hash();
        let signature = keypair.sign(&hash);

        self.frontmatter.version_hash = Some(hash);
        self.frontmatter.signature = Some(signature);
        self
    }

    /// Verify the document's integrity and authenticity.
    pub fn verify(&self, public_key: &ed25519_dalek::VerifyingKey) -> Result<()> {
        // 1. Check protocol version status
        let version = &self.frontmatter.protocol_version;
        if !is_version_known(version) {
            return Err(RhodiError::Verification(format!(
                "Unknown protocol version: {}. Document may be from a future or obsolete version.",
                version
            )));
        }

        let status = get_version_status(version);
        if status == VersionStatus::Obsolete {
            return Err(RhodiError::Verification(format!(
                "Protocol version {} is obsolete and no longer supported.",
                version
            )));
        }

        // 2. Check if the document is sealed
        let stored_hash = self.frontmatter.version_hash.ok_or_else(|| {
            RhodiError::Verification("Document is not sealed (missing version_hash)".to_string())
        })?;
        let signature = self.frontmatter.signature.ok_or_else(|| {
            RhodiError::Verification("Document is not signed (missing signature)".to_string())
        })?;

        // 3. Re-compute the hash and compare
        let computed_hash = self.compute_version_hash();
        if computed_hash != stored_hash {
            return Err(RhodiError::Verification(
                "Integrity check failed: version_hash mismatch".to_string(),
            ));
        }

        // 4. Verify the signature
        public_key
            .verify_strict(&computed_hash, &signature)
            .map_err(|e| RhodiError::Crypto(format!("Authenticity check failed: {}", e)))?;

        Ok(())
    }

    /// Update all trace blocks in the document body with current source hashes.
    pub fn update_all_traces(&mut self, base_path: &Path) -> Result<()> {
        let sections = crate::markdown::parse_tmd_sections(&self.body);
        let mut new_body = String::new();

        for section in sections {
            match section {
                crate::markdown::Section::Paragraph(p) => {
                    new_body.push_str(&p);
                }
                crate::markdown::Section::Trace(mut t) => {
                    t.update_hash(base_path)?;
                    new_body.push_str("```trace\n");
                    let yaml = serde_norway::to_string(&t).map_err(|e| {
                        RhodiError::Serialization(format!("Failed to serialize trace: {}", e))
                    })?;
                    new_body.push_str(&yaml);
                    new_body.push_str("```\n");
                }
                crate::markdown::Section::Include(i) => {
                    new_body.push_str(&i);
                }
            }
        }

        self.body = new_body;
        Ok(())
    }
}
