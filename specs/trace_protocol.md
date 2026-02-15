# Trace Protocol Specification (v1)

The `trace` block is the fundamental unit of evidence in a `.tmd` (Truthful Markdown) document. It provides a machine-verifiable link between a narrative claim and its source data.

## 1. Syntax

A `trace` block is a fenced code block with the language identifier `trace`. It uses YAML syntax for its internal metadata.

```markdown
```trace
source: <uri_or_path>
hash: <algorithm>:<value>
selector: <query_or_pattern>
expected: <value>
[optional_metadata...]
```
```

## 2. Metadata Fields

| Field | Required | Description |
| :--- | :--- | :--- |
| `source` | **Yes** | The location of the evidence. Can be a local path, a URL, or a Content Identifier (CID). |
| `hash` | **Yes*** | The cryptographic hash of the source file. *Required for `status: Published` documents.* |
| `selector` | No | A query or pattern used to extract the specific data point from the source. |
| `expected` | **Yes** | The value that the author claims exists at the source. |
| `method` | No | The verification method: `automatic`, `manual`, or `agent`. Defaults to `automatic`. |
| `timestamp` | No | ISO 8601 timestamp of when the trace was last verified. |
| `context` | No | A short snippet of surrounding text from the source to aid human verification. |
| `confidence` | No | A float between `0.0` and `1.0` representing the author's certainty. |
| `agent_metadata` | No | Nested object containing `model` (string) and `prompt_hash` (optional string) for AI-generated traces. |

### Selector Types
The compiler should support multiple selector types based on the source file extension:
- **JSON:** JSONPath (e.g., `$.users[0].name`)
- **CSV/TSV:** Column/Row coordinates (e.g., `col:2,row:10`)
- **Text:** Regex (e.g., `/Total: (\d+)/`)
- **HTML/XML:** XPath or CSS Selectors.
- **PDF:** Page and coordinate/text anchor.

## 3. Compiler Behavior (The Truth Engine)

The compiler processes `trace` blocks differently based on the document's `status` (defined in Frontmatter).

### A. Resolution & Extraction Pipeline
1.  **Resolver Selection:** Based on the `source` URI scheme (e.g., `https://`, `ipfs://`, or local path).
2.  **Integrity Check:**
    *   If `hash` is present: Calculate source hash and compare.
    *   If `hash` is missing and `status` is `final`: **Error.**
3.  **Parser Selection:** Based on source file extension or MIME type.
4.  **Extraction:** Apply the `selector` to get the `actual` value.
5.  **Validation:** Compare `actual` with `expected`.

### B. Verification Methods
- **`automatic`**: The pipeline above runs fully.
- **`manual`**: The compiler checks for a `witness` signature or a `verified: true` flag signed by a trusted public key.
- **`agent`**: Similar to automatic, but the compiler may also verify the `agent_metadata` (model, prompt hash) if provided.

### C. Status-Based Actions

| Document Status | Compiler Action on Trace Failure |
| :--- | :--- |
| `Notes` | Ignore. No verification performed. |
| `Draft` | **Warning.** Emit a warning if the source is missing, hash mismatches, or value differs. |
| `Published` | **Error.** Halt compilation. All traces must be present, hashes must match, and values must be identical. |

### D. Auto-Locking (The "Seal" Command)
When a user runs a "seal" or "finalize" command, the compiler:
1.  Calculates the `hash` for all `source` files.
2.  Updates the `trace` blocks with the current `hash`.
3.  Updates the `timestamp`.
4.  Sets the document `status` to `final`.

## 4. Implementation Roadmap for the Compiler

To implement the Truth Engine, the following modules are required in the Rust core:

1.  **`rhodi-core::resolver`**: Trait for fetching data from various sources.
2.  **`rhodi-core::extractor`**: Logic for JSONPath, Regex, and CSV parsing.
3.  **`rhodi-core::audit`**: Structure for the "Solidity Report" output.
4.  **`rhodi-core::markdown::trace_parser`**: YAML parser for the content inside ` ```trace ` blocks.

## 5. Functionalities Enabled

### 1. The Solidity Report
The compiler generates a summary of all traces, categorizing them by:
- **Source Reliability:** (e.g., `.gov` vs `.com`, signed vs unsigned).
- **Verification Method:** (How many were automatically verified?).
- **Coverage:** Ratio of claims (paragraphs) to traces.

### 2. Interactive Evidence
Downstream renderers (HTML/PDF) can turn `trace` blocks into interactive elements:
- **Hover:** Show the `expected` value and `timestamp`.
- **Click:** Open the `source` at the specific `selector` location.
- **Badge:** A "Verified" green checkmark if the compiler pass succeeded.

### 3. Agentic Accountability
If an AI agent writes a claim, it **must** provide a `trace`. The `method: agent` field allows the compiler to track which claims were synthetic and verify they aren't hallucinations by checking them against the provided `source`.
## 6. Sample Trace Block (Complex)

This example shows a trace for an AI-generated claim, including integrity hashes and confidence scores.

```markdown
```trace
source: https://api.worldbank.org/v2/en/country/all/indicator/SP.POP.TOTL?format=json
hash: sha256:f2ca1bb6c7e907d06dafe4687e579fce76b37e4e93b7605022da52e6ccc26fd2
selector: "$[1][0].value"
expected: "8000000000"
method: agent
agent_metadata:
  model: gpt-4o
  prompt_hash: sha256:a1b2c3d4...
confidence: 0.95
timestamp: 2025-12-27T14:00:00Z
```
```
