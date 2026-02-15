# Rhodi Development Plan

> **Status:** Alpha - Active Development

This document outlines the roadmap for Rhodi (Trace Protocol). Tasks marked with ✅ are completed.

---

## Phase 0: Core Hardening & Truth Engine (In Progress)

### Completed ✅
- ✅ **Extraction Logic**: Implemented `Extractor` trait with Regex support
- ✅ **Path Traversal Protection**: Include and trace sources cannot escape project root
- ✅ **Recursion Guard**: Implemented depth limiting (max 5) and cycle detection
- ✅ **Control Character Stripping**: Canonicalization sanitizes input
- ✅ **Structured Error Handling**: Using `thiserror` with `RhodiError` enum
- ✅ **Protocol Versioning**: Added `protocol_version` field with registry (Current/Deprecated/Obsolete)
- ✅ **Document Versioning**: Auto-incrementing version with previous hash chaining

### In Progress
- [ ] **Deterministic Serialization**: Improve frontmatter hashing robustness

---

## Phase 1: Infrastructure & Resolvers (Completed ✅)

- ✅ **FileResolver**: Local filesystem access
- ✅ **CLI Tool**: Full command-line interface
  - ✅ `rhodi init` - Scaffold new documents
  - ✅ `rhodi seal` - Hash and sign documents
  - ✅ `rhodi verify` - Full recursive verification
  - ✅ `rhodi update` - Refresh trace block hashes
  - ✅ `rhodi status` - Show document metadata
  - ✅ `rhodi keygen` - Generate Ed25519 keypairs

---

## Phase 2: Protocol Extensions (Future)

- [ ] **Witness Signatures**: Support multiple signatories
- [ ] **Citation Chain Verification**: Warn when citing deprecated documents
- [ ] **External Version Registry**: Support distributed version tracking

---

## Phase 3: Ecosystem & Accessibility (Future)

- [ ] **Python Bindings**: PyO3 for data scientists
- [ ] **WASM Bindings**: Browser-based validation
- [ ] **VS Code Extension**: Visual verification in editor

---

## Contributing

See AGENTS.md for development guidelines. All contributions are welcome!

---

## License

Dual licensed under MIT (open source) and commercial terms. See LICENSE file.
