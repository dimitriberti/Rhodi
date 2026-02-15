use crate::error::{Result, RhodiError};
use crate::models::{FrontMatter, TraceBlock, TracedDocument};
use serde_norway;

// ... (rest of the file stays similar but using Result)

#[derive(Debug, Clone, PartialEq)]
pub enum Section {
    /// This is a regular markdown paragraph
    Paragraph(String),
    /// This is a trace block containing evidence metadata
    Trace(TraceBlock),
    /// This is an include block for modular composition
    Include(String),
}

/// A function to parse the markdown body, separating paragraphs, traces, and includes.
pub fn parse_tmd_sections(body: &str) -> Vec<Section> {
    let mut sections = Vec::new();
    let mut in_block = false;
    let mut block_type = ""; // "trace", "include", or "tmd"
    let mut current = String::new();

    for line in body.lines() {
        let s = line.trim_start();

        if in_block {
            current.push_str(line);
            current.push('\n');
            if s.starts_with("```") {
                match block_type {
                    "trace" => {
                        if let Ok(trace) = parse_trace_block(&current) {
                            sections.push(Section::Trace(trace));
                        } else {
                            // Fallback to paragraph if parsing fails, or handle error
                            sections.push(Section::Paragraph(current.clone()));
                        }
                    }
                    "include" => {
                        sections.push(Section::Include(current.clone()));
                    }
                    _ => {
                        sections.push(Section::Paragraph(current.clone()));
                    }
                }
                current.clear();
                in_block = false;
                block_type = "";
            }
        } else if s.starts_with("```trace") {
            if !current.trim().is_empty() {
                sections.push(Section::Paragraph(current.clone()));
            }
            current.clear();
            current.push_str(line);
            current.push('\n');
            in_block = true;
            block_type = "trace";
        } else if s.starts_with("```include") {
            if !current.trim().is_empty() {
                sections.push(Section::Paragraph(current.clone()));
            }
            current.clear();
            current.push_str(line);
            current.push('\n');
            in_block = true;
            block_type = "include";
        } else {
            current.push_str(line);
            current.push('\n');
        }
    }

    if !current.trim().is_empty() {
        sections.push(Section::Paragraph(current));
    }

    sections
}

/// Canonicalize a string by:
/// 1. Normalizing line endings to LF
/// 2. Stripping trailing whitespace
/// 3. Removing control and format characters
pub fn canonicalize_text(text: &str) -> String {
    let mut result = text
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n");

    // Remove control characters and Unicode format characters
    result = result
        .chars()
        .filter(|c| {
            let code = *c as u32;
            // Keep tab, LF, CR
            if code == 9 || code == 10 || code == 13 {
                return true;
            }
            // Remove Unicode Format characters (Cf): BOM, ZWNBSP, zero-width chars, etc.
            if (0x0600..=0x0605).contains(&code)      // Arabic format overrides
                || (0x06DD..=0x06DD).contains(&code)  // Arabic end of Ayah
                || (0x070F..=0x070F).contains(&code)  // Syrian mark
                || (0x08A0..=0x08B4).contains(&code)  // Arabic extension
                || (0x08E3..=0x08FF).contains(&code)  // Arabic extension
                || (0x180E..=0x180E).contains(&code)  // Mongolian Vowel Separator
                || (0x200B..=0x200F).contains(&code)   // Zero width chars
                || (0x202A..=0x202E).contains(&code)  // Bidirectional format
                || (0x2060..=0x206F).contains(&code)  // Invisible operators
                || (0xFEFF..=0xFEFF).contains(&code)  // BOM / ZWNBSP
                || (0xFFF0..=0xFFF8).contains(&code)  // Specials
                || (0x110BD..=0x110BD).contains(&code) // Kaithi Number Sign
                || (0x1BCA0..=0x1BCA4).contains(&code)  // Shorthand format
                || (0x1D173..=0x1D17A).contains(&code)
            // Musical format
            {
                return false;
            }
            // Keep ASCII printable
            if (32..=126).contains(&code) {
                return true;
            }
            // Keep Unicode control (already filtered above) and non-control
            !c.is_control()
        })
        .collect();

    if !result.is_empty() && !result.ends_with('\n') {
        result.push('\n');
    }
    result
}

/// Parse a **trace** block and extract the metadata.
/// The block should include the triple backticks and the "trace" identifier.
pub fn parse_trace_block(block: &str) -> Result<TraceBlock> {
    let lines: Vec<&str> = block.lines().collect();
    if lines.len() < 2 {
        return Err(RhodiError::Format(
            "Invalid trace block: too short".to_string(),
        ));
    }

    // Ensure it starts with ```trace and ends with ```
    if !lines[0].trim().starts_with("```trace") {
        return Err(RhodiError::Format(
            "Invalid trace block: missing opening fence".to_string(),
        ));
    }
    if !lines.last().unwrap().trim().starts_with("```") {
        return Err(RhodiError::Format(
            "Invalid trace block: missing closing fence".to_string(),
        ));
    }

    // Extract the YAML content between the fences
    let yaml_content = lines[1..lines.len() - 1].join("\n");

    let trace: TraceBlock = serde_norway::from_str(&yaml_content)
        .map_err(|e| RhodiError::Format(format!("Failed to parse trace metadata: {}", e)))?;

    Ok(trace)
}

/// Parse a TMD (Traced Markdown Document) content into a TracedDocument struct.
pub fn parse_tmd(content: &str) -> Result<TracedDocument> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();

    if parts.len() < 3 {
        return Err(RhodiError::Format(
            "Invalid TMD format: Missing frontmatter delimiters".to_string(),
        ));
    }

    let yaml_str = parts[1];
    let body = parts[2].trim();

    let frontmatter: FrontMatter = serde_norway::from_str(yaml_str)
        .map_err(|e| RhodiError::Format(format!("Failed to parse frontmatter: {}", e)))?;

    Ok(TracedDocument {
        frontmatter,
        body: body.to_string(),
    })
}
