#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionStatus {
    Current,
    Deprecated,
    Obsolete,
}

pub const VERSION_REGISTRY: &[(&str, VersionStatus)] = &[
    ("1.0", VersionStatus::Current),
    ("1.1", VersionStatus::Current),
    ("2.0", VersionStatus::Current),
];

pub const DEFAULT_PROTOCOL_VERSION: &str = "1.0";

pub fn get_version_status(version: &str) -> VersionStatus {
    VERSION_REGISTRY
        .iter()
        .find(|(v, _)| *v == version)
        .map(|(_, status)| *status)
        .unwrap_or(VersionStatus::Obsolete)
}

pub fn get_latest_version() -> &'static str {
    VERSION_REGISTRY
        .iter()
        .max_by_key(|(v, _)| {
            let parts: Vec<&str> = v.split('.').collect();
            let major: u32 = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
            let minor: u32 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            (major, minor)
        })
        .map(|(v, _)| *v)
        .unwrap_or(DEFAULT_PROTOCOL_VERSION)
}

pub fn is_version_known(version: &str) -> bool {
    VERSION_REGISTRY.iter().any(|(v, _)| *v == version)
}
