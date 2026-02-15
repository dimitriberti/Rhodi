# Rhodi Agent Guidelines

This document provides essential context and instructions for AI agents working on the Rhodi project.

## Project Overview
Rhodi is a "ledger of truth" for research documents, implementing the **Trace Protocol**. It uses Rust for its core cryptographic and verification logic to ensure the integrity and authenticity of human and AI-generated content.

## Environment & Commands
All core development happens in the `core/` directory.

### Build & Maintenance
- **Build**: `cargo build`
- **Check**: `cargo check`
- **Lint**: `cargo clippy -- -D warnings`
- **Format**: `cargo fmt`
- **Rust Edition**: 2024

### Testing
- **Run all tests**: `cargo test`
- **Run a specific test**: `cargo test <test_name>` (e.g., `cargo test test_full_seal_and_verify_workflow`)
- **Run with logs**: `cargo test -- --nocapture`

## Code Style & Conventions

### General Rust Guidelines
- **Naming**: `snake_case` for functions/variables, `PascalCase` for types/traits.
- **Formatting**: Adhere to standard `rustfmt`. Run `cargo fmt` before submitting.
- **Imports**: Prefer explicit imports over glob imports. Group imports: std, external crates, then `crate::`.
  ```rust
  use std::collections::BTreeMap;
  use serde::{Serialize, Deserialize};
  use crate::models::TracedDocument;
  ```

### Error Handling
- **Transitioning**: We are migrating from `Result<T, String>` to structured errors using the `thiserror` crate.
- **New Code**: Define errors in a central `error.rs` or locally using `#[derive(Error, Debug)]`.
- **Safety**: Avoid `unwrap()` and `expect()` in production code. Use `?`, `ok_or`, or `map_err`.
- **Validation**: Always validate external inputs (file paths, URLs) to prevent traversal attacks.

### Data Models & Serialization
- **Models**: Defined in `core/src/models.rs`.
- **Deterministic Hashing**: Use `BTreeMap` for metadata fields to ensure stable serialization order across platforms.
- **Serde**: Use `#[serde(rename_all = "lowercase")]` or `snake_case` to match the protocol specification.
- **UUIDs**: Use `uuid` crate with `v7` features for time-sortable identifiers.
- **Dependencies**: Key crates include `ed25519-dalek`, `serde`, `sha2`, `uuid`, `chrono`, and `serde_norway`.

### Cryptography & Security
- **Algorithm**: Use `ed25519-dalek` for signing/verification and `sha2` for hashing.
- **Canonicalization**: Always canonicalize text (LF line endings, strip trailing whitespace) before hashing.
  - See `crate::markdown::canonicalize_text` for implementation.
- **Hashing**: `version_hash` = `SHA256(canonical_frontmatter + canonical_body)`.
- **Signature**: The signature covers the `version_hash` and must be excluded from the hash itself.

## Implementation Priorities (Phase 0)
1. **Hardening**: Implement path traversal protection for `include` and `trace` blocks.
2. **Truth Engine**: Implement `Extractor` logic for Regex and JSONPath selectors.
3. **Structured Errors**: Refactor existing modules to replace string-based errors.
4. **Safety**: Implement recursion guards and depth limits for modular includes.

## Document Lifecycle
- **Notes**: Passive, no verification required.
- **Draft**: Warnings for missing traces or hash mismatches.
- **Published**: Errors on any verification failure. Immutable.
- **Revoked**: Explicitly invalidates the document.

## Architecture & Philosophy
Rhodi follows a **"Core + Bindings"** philosophy:
- **The Core (Rust)**: Handles memory safety, performance, and the single source of truth for verification.
- **FFI Bindings (Planned)**: Python (PyO3), WASM/TypeScript, and a CLI tool.
- **The Ledger of Truth**: Every claim is signed, every edit is hashed, and the lineage is baked into the structure.

## File Structure Reference
- `core/src/lib.rs`: Entry point and integration tests.
- `core/src/models.rs`: Core structs (FrontMatter, TracedDocument, TraceBlock).
- `core/src/compiler.rs`: Verification and lifecycle logic (The "Truth Engine").
- `core/src/markdown.rs`: TMD parsing and canonicalization logic.
- `core/src/resolver.rs`: Source resolution traits for files/URLs.
- `core/src/crypto.rs`: KeyPair generation and signing logic.
- `specs/`: Markdown specifications for the protocol (Refer to these for logic changes).

## Protocol Specifics
- **Trace Block**: Fenced code block (````trace ````) containing YAML metadata for evidence locking.
- **Include Block**: Fenced code block (````include ````) for recursive document composition.
- **TMD**: Traced Markdown Document, separated by `---` delimiters for YAML frontmatter.

## Common Agent Tasks

### 1. Adding a new Metadata Field
- Update `FrontMatter` struct in `core/src/models.rs`.
- Update `compute_version_hash` in `core/src/models.rs` to include the new field.
- Add the field to `specs/schema.json` if applicable.
- Update `parse_tmd` or relevant parsing logic in `core/src/markdown.rs`.

### 2. Implementing a new Resolver
- Create a new struct in `core/src/resolver.rs` that implements the `SourceResolver` trait.
- Ensure the resolver handles errors gracefully and returns a `Result`.
- Register or use the new resolver in `Compiler` logic.

### 3. Improving Verification Logic
- Modifications should be made in `core/src/compiler.rs`.
- Follow the `CompilationReport` pattern for collecting errors and warnings.
- Ensure that `Published` documents fail on *any* error, while `Drafts` only issue warnings.

### 4. Handling Canonicalization
- If changing how text is hashed, update `canonicalize_text` in `core/src/markdown.rs`.
- **Note**: Changing canonicalization is a breaking change for existing document signatures.

## Security Safety Checklist
- [ ] Does this change involve external file paths? If so, is there path traversal protection?
- [ ] Does this change involve recursive calls (e.g., includes)? If so, is there a depth limit?
- [ ] Am I using `unwrap()` or `expect()`? (Replace with proper error handling).
- [ ] Is sensitive metadata being leaked in logs or error messages?
- [ ] Is the `version_hash` being calculated correctly (excluding signature/hash fields)?

## Troubleshooting
- **Build fails in `core`**: Ensure you are in the `core/` directory or use `cargo build -p core`.
- **Tests fail with "file not found"**: Check if the test relies on local assets and if `std::env::current_dir()` is being resolved correctly in the test environment.
- **Serialization mismatch**: Ensure `BTreeMap` is used for all maps and that `serde` attributes match the spec.

## Documentation Guidelines
- Maintain `specs/*.md` as the source of truth for protocol behavior.
- If an implementation deviates from the spec, update the spec first or raise an issue.
- Use `mermaid` diagrams in Markdown for complex flows.

---
*Last Updated: Jan 2026*
