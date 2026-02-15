# Protocol Versioning Specification

> **Status:** Implemented  
> **Created:** 2026-02-15  
> **Author:** Rhodi Core Team
> **Last Updated:** 2026-02-15

---

## 1. Overview

This document outlines the versioning strategy for the Trace Protocol. Protocol versioning ensures that:

1. **Breaking changes** can be introduced without breaking existing documents
2. **Verifiers** can determine which rules to apply for a given document
3. **Downgrade attacks** are prevented (someone can't re-sign an old document with new rules)

---

## 2. Background

The Trace Protocol relies on deterministic hashing and signing. The `version_hash` is computed from:

1. **Canonicalized frontmatter** (excluding `version_hash` and `signature`)
2. **Canonicalized body**

Currently, there is no mechanism to indicate which canonicalization rules were used. This creates problems when:

- We change how text is canonicalized (e.g., adding Unicode control character stripping)
- We upgrade the hash algorithm (e.g., SHA-256 → SHA-3)
- We add new required fields

---

## 3. Problem Statement

### Scenario: Canonicalization Change

We recently added Unicode control character stripping to `canonicalize_text()`. Consider:

1. **Alice** creates a document with control characters in 2025
2. The document is sealed with `version_hash_A`
3. **In 2026**, we add control character stripping
4. **Bob** tries to verify Alice's document

**Questions:**
- Should the document verify?
- If yes, should we use old or new rules?
- Can someone exploit this to create a "fixed" document that Alice never signed?

### Scenario: Algorithm Upgrade

In 5 years, we might want to upgrade from SHA-256 to SHA-3 (post-quantum). How do we:

- Verify old SHA-256 documents?
- Create new SHA-3 documents?
- Prevent attackers from re-signing old content with new algorithms?

---

## 4. Versioning Approaches

### Approach A: Implicit (Hash-Based)

The version is determined by analyzing the document itself.

**How it works:**
- No `protocol_version` field
- Verifier tries different canonicalization rules until hash matches
- OR: Hash format indicates version (e.g., `sha256:...` vs `sha3:...`)

**Example:**
```yaml
# No version field - verifier must infer
version_hash: "sha256:abc123..."
```

**Pros:**
- No schema changes required
- Backward compatible

**Cons:**
- **Ambiguity**: Multiple versions might produce same hash
- **Security risk**: Attacker could craft content that matches in multiple versions
- **Complexity**: Verifier needs to try all known versions
- **No forward compatibility**: Old verifiers can't reject new formats safely

**Verdict:** ❌ Not recommended

---

### Approach B: Major.Minor in Frontmatter

A `protocol_version` field indicates which rules to use.

**How it works:**
```yaml
protocol_version: "1.0"
version_hash: "abc123..."
signature: "def456..."
```

- **Major** (1.x): Breaking changes to canonicalization or algorithms
- **Minor** (x.0): Additive changes (new optional fields)

**Pros:**
- **Explicit**: Clear which version to use
- **Forward compatibility**: Unknown versions can be rejected
- **Flexible**: Easy to add new versions

**Cons:**
- Requires updating all documents when major version changes
- Verifier must implement version-aware logic

**Verdict:** ✅ Recommended

---

### Approach C: Hash Prefixed with Version

The version is embedded in the hash itself.

**How it works:**
```yaml
version_hash: "v1:sha256:abc123..."
# or
version_hash: "v2:sha3:def456..."
```

**Pros:**
- Self-describing hash
- No separate version field needed

**Cons:**
- Changes hash format (breaking change)
- More complex parsing
- Still needs version field for canonicalization rules

**Verdict:** ⚠️ Consider later

---

## 5. Recommended Approach: B (Major.Minor)

### 5.1 Field Definition

Add to `FrontMatter`:

```rust
/// Protocol version in "major.minor" format
/// Default: "1.0"
pub protocol_version: Option<String>,
```

**Default behavior:**
- If not present → assume "1.0" (for backward compatibility)
- If present → use exact string for matching

### 5.2 Version String Format

```
protocol_version: "major.minor"
```

| Component | Meaning | When to Bump |
|-----------|---------|--------------|
| **major** | Breaking changes | Canonicalization changes, new algorithms, removed fields |
| **minor** | Additive changes | New optional fields, new features |

**Examples:**
- `1.0` - Initial release
- `1.1` - Added optional `confidence` field to trace blocks
- `2.0` - Changed canonicalization (e.g., different Unicode handling)
- `2.1` - Added new optional metadata

### 5.3 Included in Hash

**The version string MUST be included in hash computation:**

```
version_hash = SHA256(
    protocol_version + "\n" +
    canonical_frontmatter(excluding version_hash, signature) + "\n---\n" +
    canonical_body
)
```

**Why?**
- Prevents downgrade attacks (can't re-sign v2 document as v1)
- Ensures version is cryptographically bound to document

### 5.4 Verification Logic

```rust
fn verify_document(doc: &TracedDocument, public_key: &VerifyingKey) -> Result<()> {
    // 1. Get protocol version (default to "1.0")
    let version = doc.frontmatter.protocol_version.as_deref().unwrap_or("1.0");
    
    // 2. Apply version-specific canonicalization
    let canonical_body = match version {
        "1.0" | "1.1" => canonicalize_v1(&doc.body),
        "2.0" | "2.1" => canonicalize_v2(&doc.body),
        _ => return Err(RhodiError::Verification(
            format!("Unsupported protocol version: {}", version)
        )),
    };
    
    // 3. Compute hash and verify
    // ... (existing logic)
}
```

### 5.5 Migration Strategy

| Scenario | Action |
|----------|--------|
| Minor version bump | Document continues to work; new features available |
| Major version bump | Document must be re-sealed to verify with new rules |
| Unknown version | Verifier returns error (safe default) |

**Re-sealing process:**
```bash
# User re-seals document with new protocol version
rhodi seal document.tmd --key default --force
```

This:
1. Reads existing document
2. Applies new canonicalization
3. Computes new hash
4. Signs with key
5. Updates `protocol_version` to new version

---

## 6. Implementation Plan

### Phase 1: Add Field (Low Risk)

1. Add `protocol_version: Option<String>` to `FrontMatter`
2. Default to `"1.0"` when serializing if not present
3. Include in hash computation

### Phase 2: Version-Aware Verification (Medium Risk)

1. Implement version-specific canonicalization functions
2. Add version check in `verify()`

### Phase 3: Migration Tools (When Needed)

1. Add `--force` flag to `seal` command
2. Document migration guide

---

## 7. Schema Changes

### FrontMatter (JSON Schema)

```json
{
  "type": "object",
  "properties": {
    "protocol_version": {
      "type": "string",
      "pattern": "^\\d+\\.\\d+$",
      "default": "1.0",
      "description": "Protocol version in major.minor format. Defaults to 1.0 for backward compatibility."
    },
    ...
  }
}
```

### Sample Document

```yaml
---
protocol_version: "1.0"
id: 019c610b-c123-4567-8901-234567890abc
version_hash: fa05cb525758399222664ef46de1a5340410c2388b9650f94e3216d387269c26
title: Research Findings
author: Alice
public_key: b9aee865985d892fe2a3127665cf2718c49dd49776fa0d78fb1e606770545f74
signature: b508accd9f47c4ab1ac16f03e35ee499d19c76331e3da00ee1ec88a2977c35d079de8ab2d50bb65f64b6e65228d7d6272b013982963b4384929dc49ec7ba830b
created_at: 2026-02-15T11:23:36Z
doc_status: published
---

# Research Findings

This document uses protocol version 1.0.
```

---

## 8. Backward Compatibility

### For Verifiers

| Document Version | Verifier 1.x | Verifier 2.x |
|-----------------|--------------|--------------|
| 1.0 | ✅ Works | ✅ Works |
| 1.1 | ✅ Works (ignores unknown fields) | ✅ Works |
| 2.0 | ❌ Error: Unknown version | ✅ Works |

### For Authors

| Action | Behavior |
|--------|----------|
| Create new document | Gets current protocol version |
| Re-seal existing | Keeps original version (unless --upgrade flag) |
| Major version upgrade | Must explicitly re-seal |

---

## 9. Security Considerations

### Downgrade Attack Prevention

**Attack:** Attacker takes a v2 document and re-publishes it as v1.

**Protection:**
- Version is in hash → changing version changes hash
- Signature only valid for specific version's hash

### Unknown Version Handling

**Recommendation:** Fail closed (return error) for unknown versions.

```rust
match version {
    known @ "1.0" | "1.1" | "2.0" | "2.1" => { /* verify */ }
    _ => return Err(UnknownProtocolVersion), // Safe default
}
```

---

## 10. Open Questions

1. **Should unknown minor versions be allowed?**
   - e.g., document says "1.5", verifier knows "1.0"-"1.4"
   - Option A: Allow (forward compatible)
   - Option B: Reject (strict)

2. **How long to support old versions?**
   - Option A: Indefinitely (more work)
   - Option B: Sunset after N years

3. **Auto-upgrade on seal?**
   - Should `rhodi seal` automatically upgrade to latest version?
   - Or require explicit `--upgrade` flag?

---

## 11. Summary

| Decision | Recommendation |
|----------|----------------|
| Approach | B (Major.Minor in frontmatter) |
| Default version | "1.0" |
| Version in hash | Yes (mandatory) |
| Unknown version handling | Reject (fail closed) |
| Migration | Manual re-seal with `--force` flag |

---

## 12. Implementation Notes (2026-02-15)

### Implemented Fields

**FrontMatter additions:**

```rust
pub protocol_version: String,    // Default: "1.0"
pub doc_version: u32,            // Starts at 0, auto-increments on seal
pub prev_version_hash: Option<[u8; 32]>,  // Chains to previous version
```

### Version Registry

The protocol version registry is hardcoded in `core/src/version.rs`:

```rust
pub const VERSION_REGISTRY: &[(&str, VersionStatus)] = &[
    ("1.0", VersionStatus::Current),
    ("1.1", VersionStatus::Current),
    ("2.0", VersionStatus::Current),
];
```

Unknown versions are treated as `Obsolete` and verification fails.

### Document Versioning

- `doc_version` starts at 0
- Auto-increments by 1 on each seal
- `prev_version_hash` stores the hash of the previous version before sealing
- Creates an optional chain for version verification

### CLI Updates

- `rhodi status` now displays: protocol version, protocol status (Current/Deprecated/Obsolete), document version, previous hash
- `rhodi seal` now displays: protocol version, document version

### Deprecation Workflow

1. When a new major version is released, update the registry to mark old versions as `Deprecated`
2. Deprecated versions still verify but produce warnings
3. Obsolete versions fail verification

---

## 13. References

- [Semantic Versioning](https://semver.org/) - Major.Minor.Patch convention
- [Architecture Spec](./architecture.md) - Current design
- [Trace Protocol Spec](./trace_protocol.md) - Trace block details
