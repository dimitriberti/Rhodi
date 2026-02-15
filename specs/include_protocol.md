# Include Block Specification (v1)

The `include` block enables modular composition of `.tmd` documents, allowing a "Master Document" to pull in smaller, independently versioned modules while maintaining cryptographic integrity.

## 1. Syntax

An `include` block is a fenced code block with the language identifier `include`. It uses YAML syntax for its internal metadata.

```markdown
```include
path: <relative_or_absolute_path>
hash: <algorithm>:<value>
[optional_metadata...]
```
```

## 2. Metadata Fields

| Field | Required | Description |
| :--- | :--- | :--- |
| `path` | **Yes** | The location of the included file. Must be a relative or absolute path to another `.tmd` or `.md` file. |
| `hash` | **Yes*** | The cryptographic hash of the included file at the time of inclusion. *Required for `status: Published` documents.* |
| `encoding` | No | Character encoding of the included file. Defaults to `utf-8`. |
| `timestamp` | No | ISO 8601 timestamp of when the include was last verified. |

## 3. Compiler Behavior

### A. Resolution & Inclusion Pipeline
1.  **Path Resolution:** The compiler resolves the `path` relative to the current document's location.
2.  **Cycle Detection:** Build a Directed Acyclic Graph (DAG) to detect circular includes.
3.  **Integrity Check:**
    *   If `hash` is present: Calculate the SHA-256 hash of the included file and compare.
    *   If `hash` is missing and `status` is `Published`: **Error.**
4.  **Recursive Processing:** Parse the included document and process its own `include` and `trace` blocks.
5.  **Composition:** Insert the included content at the location of the `include` block.

### B. Depth Limiting
To prevent "include bombs", the compiler enforces a maximum nesting depth (default: 5 levels).

### C. Auto-Locking (The "Seal" Command)
When sealing a document, the compiler:
1.  Recursively processes all `include` blocks.
2.  Calculates the SHA-256 `hash` for each included file.
3.  Updates the `include` blocks with the computed `hash`.
4.  Updates the `timestamp`.

### D. Status-Based Actions

| Document Status | Compiler Action on Include Failure |
| :--- | :--- |
| `Notes` | Ignore. No verification performed. |
| `Draft` | **Warning.** Emit a warning if the included file is missing or hash mismatches. |
| `Published` | **Error.** Halt compilation. All includes must be present and hashes must match. |

## 4. Implementation Roadmap

To implement include block support, the following modules are required:

1.  **`rhodi-core::resolver::include_resolver`**: Logic for resolving file paths and detecting cycles.
2.  **`rhodi-core::models::IncludeBlock`**: Struct definition matching the YAML metadata.
3.  **`rhodi-core::markdown::parse_include_block`**: YAML parser for ` ```include ` blocks.
4.  **`rhodi-core::composer`**: Module for recursively assembling master documents.

## 5. Example Usage

### Before Sealing (Draft)

**main_report.tmd:**
```markdown
---
id: 019d1234-5678-7890-abcd-ef1234567890
title: "Quarterly Report Q4 2025"
status: Draft
---

# Executive Summary

This report covers our findings for Q4 2025.

```include
path: ./sections/methodology.tmd
```

## Results

Our analysis shows...
```

### After Sealing (Published)

The compiler automatically injects the hash:

```markdown
```include
path: ./sections/methodology.tmd
hash: sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
timestamp: 2025-12-27T14:30:00Z
```
```

## 6. Security Considerations

### Preventing Path Traversal
The compiler must validate that `path` values do not escape the document root using `..` sequences or absolute paths to sensitive locations.

### Hash Pinning
By requiring `hash` for `Published` documents, we ensure that:
- The included content cannot be silently modified.
- The Master Document's signature covers the exact state of all dependencies.
- Readers can verify the integrity of the entire composition chain.
