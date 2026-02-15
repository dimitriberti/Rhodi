# Trace Protocol

> **Standardizing the Chain of Custody for Human & Artificial Intelligence.**

Trace Protocol is a document format designed to promote trustworthy content in agentic and hybrid research workflows (AI + humans).

---

**This is an experiment in accountability.** Rhodi was born from a growing concern about information corruption in the digital age. As AI-generated content proliferates and attention economies incentivize engagement over truth, I wanted to build something practical rather than just think abstractly about these problems. It explores how we might verify the origins of information, track the chain of custody for ideas, and distinguish human reasoning from synthetic generation.

I've chosen to release this as open source because:
- **Feedback is welcome** – Recommendations, alternative approaches, and critiques will only make this better
- **Debate is essential** – The problems we're trying to solve are complex and require diverse perspectives
- **Collaboration strengthens** – Information integrity is a shared concern, not a solo endeavor

This is an alpha implementation. It's a starting point, not a finished solution. If you're interested in these problems—whether from journalism, research, AI safety, cryptography, or just curiosity—I encourage you to try it, break it, and share your thoughts.

---

This project stems from my experience working in innovation and research consulting. I observed that by the time information reached its final stage, it was often corrupted and arbitrary—not through malice, but through accumulated distortion in the chain of custody.

## 0. Background

Trace Protocol emerged from years of observation in innovation and research consulting. In high-stakes environments, the "chain of custody" for an idea is often non-existent. As insights move from raw data to final executive summaries, they undergo a process of information entropy: a gradual decay where nuance is lost, and claims are stripped of their evidence.

Traditional document formats (.docx, .pdf, .pptx) are opaque binary blobs that prioritize visual layout over data integrity. They are not "machine-readable" in a meaningful way, making it nearly impossible to audit how a specific claim evolved or who is responsible for a change.

The Core Friction
The current research landscape suffers from five critical failures:

- The "Blob" Problem: Proprietary formats are black boxes. They cannot be easily version-controlled (via diff), making the evolution of a document invisible.
- The Accountability Gap: Content can be altered or fabricated without leaving a trace. There is no cryptographic link between a claim and its author.
- The Metadata Divorce: Technical metadata (who, when, where) is stored separately from the content, making it easy to strip, fake, or lose during file transfers.
- Information Entropy: Manual "copy-pasting" between tools causes data degradation. By the final report, a "suggested trend" often becomes "hard fact" through sheer repetition.
- Biological vs. Synthetic Reasoning: As AI agents enter the workflow, we lack a standard to distinguish between human-verified insights and AI-generated synthesis. This leads to "reasoning pollution," where hallucinations are ingested into the research chain as truth.
- Trace Protocol is a response to this crisis. It is a transition from opaque, "dead" files to a ledger of truth: a system where every claim is signed, every edit is hashed, and the lineage of an idea is baked into its very structure.

## 1. The Problem: Information Pollution

In modern research—both commercial and academic—**Content** is inextricably mixed with **Form**. Documents are heavy, opaque binary blobs (DOCX, PDF) that do not "speak" to each other.

Worse, as AI agents enter the workflow, we face a crisis of **reasoning pollution**. When hallucinated or anecdotal information is ingested into a research process without flags, it corrupts the entire output.

## 2. The Solution: Trace

Trace is a document format and engine designed to standardize research workflows, with specific regard to **Agentic and Hybrid (AI + Human) collaboration**.

It is not just a file format; it is a **ledger of truth**.

### Key Principles

* **Immutability:** Every edit creates a new version hash. History is preserved, never overwritten.
* **Identity First:** Every document is signed. We explicitly distinguish between **Human Authorship** (biological reasoning) and **Agent Generation** (synthetic reasoning).
* **Backward Compatibility Only:** To prevent circular logic, documents can only cite versions that existed before them.
* **Separation of Concerns:**
    * **The Envelope (YAML Frontmatter):** Contains the metadata, lineage, and signatures.
    * **The Payload (Markdown Body):** Contains the pure narrative and evidence blocks.

### Use Cases

This protocol is designed for contexts where minimizing bias and information corruption is critical:

* Commercial and academic research
* Journalism
* Regulatory compliance
* Any domain requiring auditable reasoning trails

## 3. Architecture

We follow a **"Core + Bindings"** philosophy to ensure speed, safety, and ubiquity.

### The Core (Rust) 🦀

The heart of the protocol is written in Rust. This ensures:

* **Memory Safety:** Critical for handling untrusted data inputs.
* **Performance:** Fast hashing and signing operations for large datasets.
* **Single Source of Truth:** The logic for "Is this document valid?" exists in only one place.

**Current Implementation:**
* **Canonicalization:** Deterministic normalization (LF line endings, sorted YAML keys, Unicode control character stripping)
* **Hashing:** SHA-256 for content integrity
* **Signing:** Ed25519 for cryptographic authenticity
* **Trace Verification:** Granular evidence locking for Markdown sources
* **Protocol Versioning:** Version field in frontmatter with registry (Current/Deprecated/Obsolete)
* **Document Versioning:** Auto-incrementing version with previous hash chaining
* **CLI:** Full command-line tool for document management

### The Ecosystem

The core is exposed via FFI (Foreign Function Interface) to:

* **Python:** For data scientists and AI researchers _(planned)_
* **WASM/TypeScript:** For web-based editors and visualizations _(planned)_
* **CLI:** For easy interaction via terminal _(✅ implemented)_

## 4. Usage Example (Rust Core)

```rust
use core::{TracedDocument, KeyPair, DocStatus};

// 1. Create a new document
let mut doc = TracedDocument::new(
    "Market Analysis Q4 2025",
    "The market shows signs of saturation in the consumer electronics sector."
)
.author("Research Team")
.set_status(DocStatus::Draft);

// 2. Add evidence with a trace block
doc.body.push_str("\n\n```trace\nsource: ./data/market_report.md\nexpected: \"15% decline\"\nmethod: automatic\n```\n");

// 3. Update all trace hashes (verify sources exist and compute SHA-256)
doc.update_all_traces(&std::env::current_dir().unwrap()).unwrap();

// 4. Seal the document (compute version_hash, sign with Ed25519, set status to Published)
//    This also increments doc_version and chains prev_version_hash
let keypair = KeyPair::generate();
doc = doc.seal(&keypair);

// 5. Verify the document's integrity and authenticity
doc.verify(&keypair.verifying_key).expect("Document verification failed");

// 6. The document is now cryptographically sealed
println!("Version Hash: {:?}", doc.frontmatter.version_hash);
println!("Status: {:?}", doc.frontmatter.doc_status); // Published
println!("Protocol Version: {:?}", doc.frontmatter.protocol_version); // "1.0"
println!("Document Version: {:?}", doc.frontmatter.doc_version); // 1
```

## 5. Specifications

Detailed documentation for the protocol and its implementation:

* **[Concept](specs/concept.md):** The philosophy and "Truth Engine" vision behind the project.
* **[Architecture](specs/architecture.md):** High-level system design, data flow, and Mermaid diagrams.
* **[Trace Protocol](specs/trace_protocol.md):** Detailed specification of the `trace` block for evidence verification.
* **[Include Protocol](specs/include_protocol.md):** Specification for modular document composition using `include` blocks.
* **[Versioning](specs/versioning.md):** Protocol and document versioning strategy.
* **[JSON Schema](specs/schema.json):** Formal schema definition for Traced Markdown Documents.
* **[Sample Document](specs/sample_file.tmd):** An example of a `.tmd` file following the protocol.

## 6. Document Lifecycle

| Status | Hash Required | Signature Required | Use Case |
|--------|--------------|-------------------|---------|
| **Notes** | No | No | Brainstorming, raw ideas |
| **Draft** | Recommended | No | Work in progress, assembling evidence |
| **Published** | **Yes** | **Yes** | Final, immutable, auditable documents |

When a document is "sealed", it:
1. Updates the document status to `Published` and sets the `modified_at` timestamp
2. Canonicalizes the content (normalizes line endings to LF, strips trailing whitespace, sorts YAML keys)
3. Computes a SHA-256 `version_hash` of the frontmatter (excluding `version_hash` and `signature`) + body
4. Signs the hash with the author's Ed25519 private key

Any modification to the document after sealing will cause verification to fail.

## 7. Status

**Alpha.** Core Rust implementation complete with:
- ✅ Document hashing and canonicalization
- ✅ Ed25519 signing and verification
- ✅ Trace block parsing and granular hash updates
- ✅ Full seal-and-verify workflow
- ✅ Protocol versioning (Current/Deprecated/Obsolete)
- ✅ Document versioning with hash chaining
- ✅ CLI tool with seal, verify, status, init, keygen commands
- ✅ Include block for modular composition

**Next Steps:**
- Python bindings via PyO3
- WASM bindings for browser-based validation
- External version registry support
- Citation chain verification (warn when citing deprecated docs)

## 8. Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/rhodi.git
cd rhodi/core

# Run tests
cargo test

# Build the core library
cargo build --release
```

## 9. Contributing

_Guidelines to be established._

## 10. License

_To be decided._

---

As AI agents increasingly participate in knowledge work, the "Chain of Custody" for information is becoming critical for trust. If you cannot trace a piece of reasoning back to a verified human or a signed data source, the output becomes unreliable.


