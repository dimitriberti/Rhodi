# 📝 Project Readme: The Modular Manuscript Format

## 🚀 1. The Philosophy: Why This Format?

Traditional document formats (Word, PDF) encourage ambiguity, resist version control, and break the chain of evidence. This format, referred to here as the **`.tmd` Manuscript**, shifts the focus from simple formatting to **Rigor, Traceability, and Auditable Truth**.

This system is designed for individuals who demand **verifiable claims**—journalists, academic researchers, and development teams focusing on immutable documentation.


## 📄 2. The Unified `.tmd` Document/Manuscript Format

We maintain a minimalist approach by using a **single file format** (Markdown, usually saved as a `.md` or `.tmd` file). The document is organized into three distinct structural zones:

1.  **The Envelope (YAML Frontmatter):** Machine-readable metadata for tracking rigor and status.
2.  **The Body (Markdown):** Human-readable text and narrative.
3.  **The Blocks (Fenced Code):** Structured sections for evidence (`trace`) and composition (`include`).

### The Document Lifecycle (Progressive Rigor)

The file format remains the same, but the **`status`** field in the Frontmatter dictates the strictness of the compiler.

| Status | Rigor Level | Compiler Action | Use Case |
| :--- | :--- | :--- | :--- |
| **`Notes`** | Low | Passive. Renders only. | Brainstorming, raw ideas, meeting minutes. |
| **`Draft`** | Medium | **Warnings.** Flags missing traces and unverified claims. | Work in progress, assembling evidence. |
| **`Published`** | High (100% Traceable) | **Errors.** Halts compilation if any trace is broken or hash mismatched. | Publication, delivery, archival. |
| **`Revoked`** | N/A | **Errors.** Marks the document as invalid/retracted. | Retraction, deprecation. |

-----

## 🏗️ 3. Core Components: Syntax

This format relies on two key structural blocks for ensuring rigor and modularity.

### A. The Evidence Block: `trace`

The `trace` block is used **inline** within the narrative to provide mathematical proof for a claim.

  * **Purpose:** To make a claim 100% traceable. In a final document, every factual assertion should be backed by a trace.
  * **Syntax:** Uses YAML metadata to define the source, integrity, and extraction logic.
  * **Detailed Spec:** See [trace_protocol.md](trace_protocol.md) for the full technical specification.

  **Example:**
  ````markdown
  We found a high failure rate in the data.

  ```trace
  source: ./evidence/analysis.md
  hash: sha256:d82e1d7...
  selector: "col:failure_rate,row:last"
  expected: "85%"
  method: automatic
  timestamp: 2025-12-27T14:00:00Z
  ```
  ````

### B. The Composition Block: `include` (Signed Transclusion)

The `include` block is used in a **Master Document** to pull in smaller, independently versioned **Modules** (other `.tmd` files). This is the key to managing large documents and maintaining the version chain.

  * **Purpose:** To prevent the integrity of the Master Document from being broken if a source file is later modified.
  * **Minimal Syntax (You Write):**
    ````markdown
    # Introduction
    ```include
    path: ./sections/01_intro.md
    ````
    ```
    ```
  * **Compiler Action (Finalizing):** The Truth Engine automatically locks the content by injecting the hash:
    ````markdown
    # Introduction
    ```include
    path: ./sections/01_intro.md
    hash: "sha256:d82e1d7..."  # <--- Locked version
    ````
    ```
    
    ```

-----

## ⚙️ 4. The Compilation Pipeline (The Truth Engine)

Compilation is not just rendering; it is a **multi-stage audit** that verifies the entire **Chain of Trust** before generating the final artifact (PDF, HTML, etc.).

### Stages of Compilation:

1.  **Resolution Pass:** The compiler follows all `include` paths, recursively assembling the final document content.
2.  **Integrity Check:** For every `include` block, the compiler calculates the hash of the referenced file and compares it to the locked `hash` in the block.
      * *Result:* Ensures **immutability** of included content.
3.  **Trace Verification Pass:** The compiler checks every `trace` block. It verifies external file paths, runs embedded queries, and ensures the claimed data/quote is present in the source.
      * *Result:* Ensures **100% traceability** of all facts in the final document.
4.  **Solidity Scoring:** The compiler analyzes the sources used across all traces (e.g., internal databases, journals, unverified blogs) to generate an overall **Rigor Score**.
      * *Result:* **The Solidity Report**, warning the author if the claim-to-evidence ratio is too low or if too many low-tier sources are used.

> 💡 **NOTE:** A document with `status: final` will only compile if the integrity check passes and all traces are verifiable.


### NOTES

##### Keywords

- digital manuscript
- truth engine
- digital wax seal


---


1. Integrity: "Has this been tampered with?"
The primary role of the signature is to ensure that not a single comma, decimal point, or trace block has been altered since the author finished the document.

How it works: The signature is a mathematical snapshot of the entire file. If a bad actor (or a corrupted hard drive) changes a "failure rate" from 85% to 8.5%, the signature becomes instantly invalid.
The Benefit: In your "Final" status, the compiler checks the signature first. If the signature doesn’t match the content, the compiler refuses to generate the PDF/HTML, protecting the reader from misinformation.
2. Authenticity: "Who actually wrote this?"
The public key identifies the author in a way that a typed name (author: Som) cannot.

The Role: The Public Key is like your global ID number or a DNA profile. The Signature is proof that the person who owns that "ID" actually clicked "Sign."
Non-Repudiation: Because only you have the Private Key associated with your Public Key, you cannot later claim, "I didn't write that." This is critical for investigative journalism or legal evidence.
3. The "Chain of Trust" (Recursive Logic)
This is where it gets powerful for your Modular system. When you use the include block, the signature of the Master Document covers the Hashes of the included modules.

The Scenario:
You write a methodology.tmd file and sign it.
You include it in your main_report.tmd.
The Master Document records the hash of the methodology.
You sign the Master Document.
The Result: You have now "frozen" the entire hierarchy. If someone tries to swap out your methodology.tmd for a fake version, the Master Document's signature will break because the reference inside it no longer matches.
4. Machine-Readable Evidence (Zero-Trust)
In a traditional Word doc, you have to trust the metadata (Created Ar, Modified At). In an .tmd document, the Truth Engine doesn't trust anything.

The Role: The compiler uses the public key to verify that the "Claims" (Traces) and the "Structure" (Includes) were authorized by the same entity.
Verification without the Author: Anyone with the .tmd file and your Public Key can verify the document's validity offline, without needing a central server or your permission.
How the Process Looks (The Workflow)
Drafting: You write the Markdown. The signature field in the Frontmatter is empty.
Sealing: You run a command (e.g., tmd sign --key my_private.key).
The tool hashes the text: Hello world -> a1b2c3...
The tool encrypts that hash with your Private Key to create the Signature.
The tool injects the Signature into the YAML.
Auditing (The Reader): A reader gets your file. Their "Truth Engine":
Takes the Public Key from the header.
Decrypts the Signature to see the "Original Hash."
Calculates a "New Hash" from the current text.
If Original Hash == New Hash: The document is "Veracious."
Summary Table
Feature	Without Signature	With Signature (.tmd)
Trust Model	Trust the sender.	Trust the math.
Tamper Detection	Hard to spot (manual proofreading).	Instant (compiler error).
Identity	Vulnerable to "spoofing."	Cryptographically proven.
Modularity	"Includes" can change secretly.	"Includes" are hash-locked.
In short: The Public Key is your Identity, and the Signature is your Oath that the contents are exactly as you left them.


--- 

This is a sophisticated design, but if you want to move from a "good idea" to a "robust standard," we need to stress-test it. Below are challenges to the current design and suggested improvements to make the `.tmd` format truly "battle-hardened."

---

### 1. The "Whitespacing" Challenge (Technical Fragility)
**The Problem:** Cryptographic signatures are extremely sensitive. If someone opens your `.tmd` file, adds a single space at the end of a line, or their editor changes the line endings from LF to CRLF, the signature will break. The document will appear "tampered with" even though the content is the same.

*   **Improvement:** **Canonicalization.** Your "Truth Engine" needs a "Strict Clean" pass before signing or verifying. It should strip trailing whitespace, standardize line endings, and alphabetize YAML keys before hashing.
*   **Alternative:** Instead of signing the *file*, sign a **Merkle Tree** of the file’s sections (Frontmatter, Body, Blocks). This allows you to say "The Body is intact, but the Metadata has been updated," rather than the whole thing failing.

### 2. The "Trace Rot" Problem (Data Persistence)
**The Problem:** You have a `trace` block pointing to `./evidence/analysis.csv`. If you email that `.tmd` file to a colleague without the CSV, the document is "broken." If the CSV is deleted, the document's truth is lost forever.

*   **Improvement:** **Inlining vs. Referencing.**
    *   **Small Data:** Allow the `trace` block to hold a base64-encoded snippet or a "snapshot" of the result.
    *   **Large Data:** Use **Content-Addressable Storage (CAS)**. Instead of a file path, use a hash (e.g., `cid: bafy...` from IPFS or a git-blob hash). This makes the evidence "immutable" and searchable across the web.
*   **Improvement:** **Evidence Sidecar.** When "Finalizing," the compiler should create a `.bundle` or `.zip` containing the `.tmd` file *and* every file referenced in a `trace` or `include`.

### 3. The "Semantic Gap" (Logical Challenge)
**The Problem:** A `trace` block proves that a specific query in a specific file results in "85%." It **does not** prove that the author’s sentence ("We found a high failure rate") is a fair interpretation of that 85%.

*   **Improvement:** **Assertion Logic.** Add a `threshold` or `expectation` field to the trace.
    ```trace
    source: ./data.csv
    query: "SELECT rate FROM results"
    operator: GREATER_THAN
    threshold: 80%
    result: 85%  # Verified
    ```
    This forces the author to define *why* they are making the claim, narrowing the gap between raw data and narrative opinion.

### 4. Privacy vs. Transparency
**The Problem:** Journalists often have "Verified Evidence" that they **cannot** share publicly (e.g., a leaked document or a witness identity). If the `trace` requires the source file to be present to pass the audit, you can never publish the document without exposing the source.

*   **Improvement:** **Zero-Knowledge Traces.** Allow the compiler to "Witness" a trace on the author's machine and generate a **Proof of Observation**. The final document would contain a hash of the source data and a "Signature of Verification" from a trusted third-party auditing tool, without including the sensitive source data itself.

### 5. The "Include" Loop (Circular Dependency)
**The Problem:** If `File A` includes `File B`, and `File B` includes `File A`, the Truth Engine will crash or enter an infinite loop.

*   **Improvement:** **Dependency Graphing.** The compiler must build a Directed Acyclic Graph (DAG) of all includes before processing. It should also have a `Max-Depth` setting (e.g., you can't nest includes more than 5 levels deep) to prevent "File Bloat."

### 6. Challenging the YAML Meta-data
**The Challenge:** You have `created_at` and `modified_at`.
*   In an immutable audit system, these are "claims" by the author, not "facts."
*   **Improvement:** Integrate with a **Timestamping Authority (TSA)** or a Blockchain. The signature shouldn't just be "I am Som," it should be "I am Som, and this hash existed at this specific Unix Epoch," verified by a third-party server.

---

### Updated Syntax Suggestion: The "Lock" Section

To solve the "Trace Rot" and "Portability" issues, I suggest adding a `locks:` section to the YAML that the compiler handles automatically.

```yaml
---
id: UUID
status: final
# ... usual headers ...
locks:
  - path: ./sections/01_intro.md
    hash: sha256:d82e1...
  - path: ./evidence/analysis.csv
    hash: sha256:e3b0c...
    source_url: "https://archive.org/details/my-data-v1"
---
```

### The "Big Idea" Challenge: Why not just use Git?
**The Critique:** Most of what you want (v-chain, hashing, authorship) is what Git does. 
*   **The Counter-Argument (Your Value Prop):** Git tracks *files*. Your format tracks *claims*. 
*   **Design Shift:** Position the `.tmd` format not as a replacement for version control, but as a **Presentation Layer for Truth**. Git is where the work happens; `.tmd` is the **Certified Export** that guarantees the data and the text are still in sync.

**Which of these challenges feels most critical for your specific use case (Academic, Journalism, or Legal)?**

---

This is a compelling evolution of the "Document as Code" concept. By treating a document like a compiled binary—where dependencies are hashed and claims are unit-tested—you are moving toward **computational journalism/research.**

Below is a brainstormed expansion on your logic, followed by a prototype `.tmd` file.

---

### 1. Conceptual Brainstorming (Expanding the Rigor)

#### Why Hex for the Ed25519 Signature?
You asked: *"(why not Base64)"*.
*   **Collision with Markdown:** Base64 often includes `/` and `+`, and always ends with `=`. In some parsers, `/` can be interpreted as a path or a regex start. Hex is purely alphanumeric `[0-9a-f]`, making it "web-safe" and "shell-safe" by default. 
*   **Auditability:** Humans are better at spotting patterns/errors in Hex strings than in the mixed-case, symbol-heavy strings of Base64.
*   **Standardization:** Tools like `clis` for Ed25519 (and many hardware security modules) default to Hex for public keys and signatures.

#### The "Solidity Score" Logic
To make the "Rigor Score" objective, you could categorize sources:
*   **Tier 0 (Hash-Locked):** Local `.csv` or `.tmd` files with a SHA-256 hash. (High Score)
*   **Tier 1 (Persistent Identifiers):** DOIs, ISBNs, or IPFS CIDs. (High Score)
*   **Tier 2 (Live Web):** URLs (HTTPS). (Medium-Low Score, as links rot).
*   **Tier 3 (Unverified):** Quotes without a source block. (Zero Score).

#### The Signature "Self-Reference" Problem
To sign a document, the `signature` field must be empty *while* the signature is being calculated, then injected afterward. The "Truth Engine" would need to:
1. Strip the `signature` value.
2. Hash the remaining document.
3. Verify the hash against the hex signature using the author's public key (stored in a `keys/` directory or a global registry).

---

### 2. Prototype File: `report_2023_001.tmd`

This file demonstrates a "Master Document" in the `draft` phase, moving toward `final`.

```markdown
---
id: 550e8400-e29b-41d4-a716-446655440000
title: "Quarterly Environmental Impact Audit: Sector 7G"
author: "Som"
status: draft
created_at: 2023-10-27T10:00:00Z
modified_at: 2023-10-27T14:30:00Z
public_key: 8be5...af01 (Ed25519 Hex)
signature: 4f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a  # Calculated on all content excluding this line
---

# Executive Summary

The transition to modular energy units has resulted in a measurable decrease in carbon output, though the raw data suggests an anomaly in the cooling systems.

```include
path: ./modules/methodology.tmd
hash: sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
```

## 1. Key Findings: Carbon Output

According to our sensors, the average carbon reduction across all testing sites reached 22.4% this quarter.

```trace
source: ./data/sensor_readings.csv
query: "SELECT AVG(reduction_pct) FROM sensors WHERE period='Q3'"
expected: "22.4%"
confidence: 0.98
witness: "Climate Integrity API"
```

> **Note to Editor:** The trace above is pulled directly from the CSV. If the CSV is updated and the average changes, the `final` compilation will fail, alerting us that the text is now factually outdated.

## 2. Infrastructure Status

Recent inspections of the cooling towers revealed structural micro-fractures in three units. 

```include
path: ./modules/structural_analysis.tmd
# No hash provided yet; the compiler will flag this as a 'Warning' in draft status.
```

## 3. Comparative Rigor

The claims in this document are backed by the following evidence chain:

```trace
source: "doi:10.1038/s41586-020-2012-7"
claim: "Standardized cooling coefficients."
verification: "Cross-referenced with internal thermal models."
```

---

### Solidity Report (Pre-rendering Preview)
*   **Total Claims:** 2
*   **Verified Traces:** 2/2
*   **Unlocked Includes:** 1 (Warning: `structural_analysis.tmd` is not version-locked)
*   **Current Rigor Score:** 84/100
```

---

### 3. Proposed Folder Structure for the Project
To make this work, the "Truth Engine" (compiler) would expect a workspace like this:

```text
/my-project/
├── .tmd-config          # Settings for the Solidity Scoring
├── main_report.tmd      # The Master Document
├── modules/              # Sub-documents (Chapters/Sections)
│   ├── methodology.tmd
│   └── structural_analysis.tmd
├── data/                 # Evidence sources
│   └── sensor_readings.csv
└── keys/                 # Author Public Keys
    └── som.pub.hex
```

### 4. Next Steps for Brainstorming
*   **The "Diff" Logic:** How do we show what changed in a trace between versions? (e.g., "The query result moved from 21% to 22.4%").
*   **The Reader UI:** How should a non-technical reader see the traces? Perhaps a "Verify" button on a PDF that links back to the git commit of the source data?
*   **Ephemeral Data:** How do you handle a `trace` that points to a live API that might change tomorrow? (Perhaps the compiler should "cache" the response and hash the cache). 

What part of the **Compilation Pipeline** or **Syntax** would you like to refine next?


--- 

INITIAL SCRIPT 

For a format that prides itself on **Truth and Traceability**, the authoring script should be as frictionless as possible while ensuring the "DNA" of the document (ID, Timestamp, and Structure) is established correctly.

Here is a design for the **simplest authoring script**, using a "Wizard" approach or a "One-Liner" approach.

### 1. The Minimum Viable Frontmatter
To keep it simple, the script should automatically generate the things a human shouldn't have to type (UUIDs and Timestamps).

**The Input:**
*   Title
*   Author
*   Content (Body)

**The Optional:**
*   Signature (usually added *after* the draft is finished).

---

### 2. Experimental Script (Python Example)
This script creates a new `.tmd` (Traceable Markdown) file. It handles the "boring" parts automatically.

```python
import uuid
from datetime import datetime
import os

def create_manuscript(title, author, content):
    # 1. Generate Metadata
    doc_id = str(uuid.uuid4())
    timestamp = datetime.utcnow().isoformat() + "Z"
  
    # 2. Construct the Frontmatter
    frontmatter = f"""---
id: {doc_id}
title: "{title}"
author: "{author}"
created_at: {timestamp}
status: draft
trace_logic: enabled
signature: null
---

"""
  
    # 3. Create the filename
    filename = f"{title.lower().replace(' ', '_')}.tmd"
  
    # 4. Write the file
    with open(filename, "w") as f:
        f.write(frontmatter + content)
  
    print(f"✅ Created: {filename}")

# Usage
if __name__ == "__main__":
    t = input("Enter Title: ")
    a = input("Enter Author: ")
    print("Paste your content below (Press Ctrl+D or Ctrl+Z to finish):")
    c = []
    while True:
        try:
            line = input()
            c.append(line)
        except EOFError:
            break
  
    create_manuscript(t, a, "\n".join(c))
```

---

### 3. How the "Signature" fits in
The signature should **not** be provided at the moment of creation. Here is why:

1.  **The Draft Phase:** You write the content. The signature is `null`.
2.  **The Signing Phase:** Once you are ready to "publish" or "finalize," you run a separate `tmd sign` command.
3.  **The Logic:** Signing happens to the *entire* file. If you signed it while writing it, every time you fixed a typo, the signature would break.

**The Workflow should be:**
1.  `tmd new` (Creates the file shell)
2.  `Author edits file` (Writing and adding `trace` blocks)
3.  `tmd verify` (Compiler checks if the traces work)
4.  `tmd sign` (Appends the cryptographic signature)

---

### 4. Improvements to the Authoring Script

If you want to make the script smarter for your specific format, add these three features:

#### A. The "Slug" System
Instead of asking for a filename, the script should take the **Title** and turn it into a `slug` (e.g., "Daily Report" becomes `daily_report.tmd`).

#### B. The "Evidence" Folder
When the script creates `my_report.tmd`, it should also create a folder named `my_report_assets/`. This discourages users from losing the data they intend to `trace`.

#### C. Trace Template Auto-fill
The script could append a sample `trace` block at the bottom so the user remembers the syntax:
```markdown
[comment]: # (Sample Trace Block)
'''trace
source: 
query: 
'''
```

### Should you provide the signature in the script?
**No.** Keep the script for *creation*. The **Signature** is a "Sealing" event. 

**Think of it like this:**
*   **The Script** is the pen and paper.
*   **The Signature** is the notary's stamp. 
You don't stamp the paper before you've even written the words. 

**Proposed Command Line Interface (CLI):**
*   `tmd init "My Document"` → Generates the file.
*   `tmd sign my_doc.tmd` → Prompts for your private key and seals it.


---

This is a foundational decision for the architecture of your format. In a "Truth Engine" or "Traceability" system, the ID serves two roles: **Identity** (who are you?) and **Integrity** (have you changed?).

Here is the recommendation for addressing both:

### 1. Should the ID be created at document creation?
**Yes.** 
If you wait until the document is finished to give it an ID, you cannot easily reference it while it is "in progress." However, you should distinguish between a **Document ID** and a **Version ID**.

*   **Document ID (The "Birth Certificate"):** Created once when the file is born. It never changes, even if you rename the file or edit every word. It allows you to track the *lineage* of a thought.
*   **Version ID (The "Fingerprint"):** Changes every time the file is saved or signed. This is usually a hash (like SHA-256) of the content.

### 2. Should you use UUID?
For your specific "Trace" and "Truth" focus, I would actually **discourage** standard UUID (v4).

**Why UUID v4 is "meh" for your project:**
A random UUID like `550e8400-e29b-41d4-a716-446655440000` is just noise. It doesn't tell a story. It’s hard to sort and contains no metadata.

### The Alternatives (Better for Traceability):

#### Option A: ULID (Universally Unique Lexicographical Identifier) — *Highly Recommended*
ULIDs are 128-bit (same size as UUID) but they are **time-sortable**.
*   **Example:** `01H7B6K4X5...`
*   **Why it's better:** The first half of the string is a timestamp. If you have 1,000 `.tmd` files in a folder, sorting by ID automatically sorts them by the exact millisecond they were created.
*   **Vibe:** It feels "scientific" and structured.

#### Option B: Content-Addressable ID (The Git Way)
You don't put an ID in the YAML at all. Instead, the "Truth Engine" calculates a hash of the file.
*   **Pro:** Total integrity. If a single bit changes, the ID changes.
*   **Con:** You can’t refer to a "Project" as a whole because the ID is always shifting.

---

### Suggested ID Strategy for your Script

I recommend a **Hybrid Approach** in your YAML frontmatter. This gives you the best of both worlds (identity and verification).

```yaml
---
# The Permanent Identity (Generated at 'init')
# Use ULID for time-sorting and uniqueness.
id: 01HRC699VDR8H05SK618B1N78J 

# The Integrity Check (Generated at 'sign' or 'save')
# This is a hash of the content below the frontmatter.
version_hash: sha256:e3b0c442... 
---
```

### Why this is the "Truth" way:
1.  **Traceability:** If you link to `id: 01HRC...`, I can find all versions of that document in my database.
2.  **Verification:** The `version_hash` ensures that the specific version I am reading hasn't been tampered with.
3.  **Simplicity:** Your authoring script only needs to generate the **ID** once. The `version_hash` is left as `null` or `pending` until the "Truth Engine" processes it.

### Revised "Simplest Script" Logic:

1.  **User runs:** `trace-md init "Medical Results"`
2.  **Script generates:**
    *   **Filename:** `medical_results_01HRC.tmd` (Slug + short ID for uniqueness).
    *   **ID:** A fresh **ULID**.
    *   **Timestamp:** Current UTC time.
3.  **User writes:** Content...
4.  **User runs:** `trace-md seal`
    *   The script calculates the hash of the body.
    *   The script signs the hash with the user's key.
    *   The script updates the `version_hash` and `signature` fields.

**Verdict:** Use **ULID** at creation. It makes your documents feel like "entries in a ledger" rather than just "random files on a disk."


Hash **the canonical document bytes**—which usually means **frontmatter + body**, but with **some fields excluded** (because they would create a circular dependency or change for non-content reasons).

If you hash only the body, you lose integrity for metadata (title, author, created_at, etc.). If you hash everything including `version_hash` and `signature`, you create a self-referential loop. So the practical answer is:

## Recommended: hash the whole document except “mutable/derived” fields
### Include in the hash
- All frontmatter fields that you want protected (e.g. `id`, `title`, `author`, `created_at`, `tags`, etc.)
- The entire body content

### Exclude from the hash (must not be part of what is hashed)
- `version_hash` (because you’re computing it)
- `signature` (because it signs the hash)
- Potentially other derived fields your tooling rewrites (e.g. `last_verified_at`, `verification_status`, `rendered_at`, etc.)

This gives you: “This exact document (metadata + content) is what was signed.”

## Important: canonicalization (so hashing is stable)
Hashing “the file as-is” can be fragile because of:
- YAML key order changes
- whitespace differences
- line ending differences (LF vs CRLF)

To avoid accidental hash changes, define a canonical form. Two common approaches:

### Approach A (simple, effective): hash the raw bytes but enforce formatting rules
- Normalize line endings to `\n`
- Require UTF‑8
- Require stable frontmatter formatting (your tool writes it, users don’t reorder keys)
- Exclude `version_hash` and `signature` lines from frontmatter before hashing

This is easiest if your tool is the only writer of frontmatter.

### Approach B (more robust): parse frontmatter and re-serialize canonically
- Parse YAML into a data structure
- Remove excluded keys (`version_hash`, `signature`, etc.)
- Serialize YAML in a canonical way (sorted keys, consistent quoting)
- Concatenate with body using a defined delimiter
- Hash the result

This is more work but resilient to reformatting.

## When would “hash body only” be acceptable?
Only if your frontmatter is considered “non-authoritative” or “mutable labels.” That’s uncommon in traceability systems. If title/author/timestamps matter, hash them too.

## A clean spec rule you can adopt
> `version_hash = sha256( canonicalize(frontmatter_without(version_hash, signature)) + "\n---\n" + canonicalize(body) )`

(Use whatever delimiter you already use; just define it.)

## Minimal practical recommendation
- Hash **frontmatter + body**
- Exclude `version_hash` and `signature`
- Normalize to UTF‑8 and LF
- Keep a stable frontmatter key order

-----

## 🔐 5. Chain of Trust & Permissions

To support multi-author collaboration and secure embedding, the format includes a **Policy** system in the Frontmatter.

### Metadata Policy Fields
*   **`public_key`**: Binds the author's identity to the document signature.
*   **`policy`**:
    *   `allow_include`: (bool) Can this document be embedded in others?
    *   `allow_quote`: (bool) Can snippets be traced/quoted?
    *   `require_attribution`: (bool) Must the author be credited?

### Verification Logic
When compiling a Master Document, the Truth Engine checks the `policy` of every included file. If `allow_include` is false, compilation fails. This ensures authors retain control over how their work is reused.


If you tell me your exact file layout (YAML frontmatter delimiter, whether you allow comments, whether tools may reorder keys), I can propose a precise canonicalization rule that won’t surprise users.