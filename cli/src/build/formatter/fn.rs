use super::*;

use std::path::Path;

use anyhow::Result;

/// Scans a Rust source file and formats every supported macro invocation
/// in place, writing the result back to disk.
///
/// Supported macros: `html!`, `class!`, `css_vars!`, `var!`
///
/// # Arguments
///
/// - `&Path`: Path to the `.rs` file to format.
///
/// # Returns
///
/// - `Result<()>`: Indicates success or failure of the formatting operation.
pub(crate) fn format_file(path: &Path) -> Result<()> {
    let source: String = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", path.display(), e))?;
    let formatted: String = format_source(&source);
    if formatted != source {
        std::fs::write(path, &formatted)
            .map_err(|e| anyhow::anyhow!("Failed to write file {}: {}", path.display(), e))?;
        log::info!("Formatted: {}", path.display());
    }
    Ok(())
}

/// Recursively walks a directory and formats all `.rs` files found.
///
/// # Arguments
///
/// - `&Path`: The root directory to scan recursively.
///
/// # Returns
///
/// - `Result<()>`: Indicates success or failure.
pub(crate) fn format_dir(dir: &Path) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    let entries: Vec<std::fs::DirEntry> = std::fs::read_dir(dir)
        .map_err(|e| anyhow::anyhow!("Failed to read dir {}: {}", dir.display(), e))?
        .filter_map(|entry| entry.ok())
        .collect();
    for entry in &entries {
        let path: std::path::PathBuf = entry.path();
        if path.is_dir() {
            format_dir(&path)?;
        } else if path.extension().is_some_and(|ext| ext == "rs")
            && let Err(error) = format_file(&path)
        {
            log::warn!("Failed to format {}: {}", path.display(), error);
        }
    }
    Ok(())
}

/// Formats all supported macro invocations within a source string.
///
/// # Arguments
///
/// - `&str`: The Rust source code.
///
/// # Returns
///
/// - `String`: The source with all supported macros formatted.
pub(crate) fn format_source(source: &str) -> String {
    let mut result: String = String::new();
    let mut last_end: usize = 0;
    for span in find_macro_spans(source) {
        result.push_str(&source[last_end..span.start]);
        let original: &str = &source[span.start..span.end];
        let formatted: String = format_brace_block(original);
        result.push_str(&formatted);
        last_end = span.end;
    }
    result.push_str(&source[last_end..]);
    result
}

/// Finds all top-level macro invocations in the source.
///
/// # Arguments
///
/// - `&str`: The Rust source code to scan.
///
/// # Returns
///
/// - `Vec<MacroSpan>`: The located macro spans, in order of appearance.
pub(crate) fn find_macro_spans(source: &str) -> Vec<MacroSpan> {
    let chars: Vec<char> = source.chars().collect();
    let len: usize = chars.len();
    let mut spans: Vec<MacroSpan> = Vec::new();
    let mut i: usize = 0;
    while i < len {
        if chars[i] == '"' {
            i = skip_string_literal(&chars, i);
        } else if chars[i] == '\''
            && i + 2 < len
            && chars[i + 1] != '\''
            && chars[i + 1] != '\\'
            && is_char_literal(&chars, i)
        {
            i = skip_char_literal(&chars, i);
        } else {
            let matched: Option<usize> = try_match_macro_name(&chars, i, len);
            if let Some(name_end) = matched {
                let bang_pos: usize = skip_ws_and_comments(&chars, name_end, len);
                if bang_pos < len && chars[bang_pos] == '!' {
                    let brace_pos: usize = skip_ws_and_comments(&chars, bang_pos + 1, len);
                    if brace_pos < len && chars[brace_pos] == '{' {
                        let close_brace: usize = find_matching_brace(&chars, brace_pos);
                        spans.push(MacroSpan {
                            start: brace_pos,
                            end: close_brace + 1,
                        });
                        i = close_brace + 1;
                        continue;
                    }
                }
                i = name_end;
            } else {
                i += 1;
            }
        }
    }
    spans
}

/// Tries to match a supported macro name at position `i`.
///
/// # Arguments
///
/// - `&[char]`: The source characters.
/// - `usize`: The starting index.
/// - `usize`: The total length.
///
/// # Returns
///
/// - `Option<usize>`: The end position of the matched name, or `None`.
fn try_match_macro_name(chars: &[char], i: usize, len: usize) -> Option<usize> {
    let names: &[&str] = &["html", "class", "css_vars", "var"];
    for name in names {
        let name_chars: Vec<char> = name.chars().collect();
        let name_len: usize = name_chars.len();
        if i + name_len > len {
            continue;
        }
        let mut matched: bool = true;
        for j in 0..name_len {
            if chars[i + j] != name_chars[j] {
                matched = false;
                break;
            }
        }
        if !matched {
            continue;
        }
        if i + name_len < len && is_identifier_continue(chars[i + name_len]) {
            continue;
        }
        if i > 0 && is_identifier_continue(chars[i - 1]) {
            continue;
        }
        return Some(i + name_len);
    }
    None
}

/// Skips whitespace and comments forward from position `i`.
///
/// # Arguments
///
/// - `&[char]`: The source characters.
/// - `usize`: The starting index.
/// - `usize`: The total length.
///
/// # Returns
///
/// - `usize`: The index of the first non-whitespace, non-comment character.
fn skip_ws_and_comments(chars: &[char], mut i: usize, len: usize) -> usize {
    while i < len {
        if chars[i] == ' ' || chars[i] == '\t' || chars[i] == '\n' || chars[i] == '\r' {
            i += 1;
        } else if i + 1 < len && chars[i] == '/' && chars[i + 1] == '/' {
            i = skip_line_comment(chars, i);
        } else if i + 1 < len && chars[i] == '/' && chars[i + 1] == '*' {
            i = skip_block_comment(chars, i);
        } else {
            break;
        }
    }
    i
}

/// Skips a string literal starting at the given position.
///
/// # Arguments
///
/// - `&[char]`: The source characters.
/// - `usize`: The starting index of the opening quote.
///
/// # Returns
///
/// - `usize`: The index immediately after the closing quote.
fn skip_string_literal(chars: &[char], start: usize) -> usize {
    let len: usize = chars.len();
    let mut i: usize = start + 1;
    while i < len {
        if chars[i] == '\\' {
            i += 2;
        } else if chars[i] == '"' {
            return i + 1;
        } else {
            i += 1;
        }
    }
    len
}

/// Determines whether a `'` at position `i` is a character literal.
///
/// # Arguments
///
/// - `&[char]`: The source characters.
/// - `usize`: The position of the opening `'`.
///
/// # Returns
///
/// - `bool`: `true` if it looks like a character literal.
fn is_char_literal(chars: &[char], i: usize) -> bool {
    let len: usize = chars.len();
    if i == 0 {
        return true;
    }
    let prev: char = chars[i - 1];
    if prev.is_alphanumeric() || prev == '_' {
        return false;
    }
    let mut j: usize = i + 1;
    while j < len && chars[j] != '\'' {
        if chars[j] == '\\' {
            j += 1;
        }
        j += 1;
    }
    j < len
}

/// Skips a character literal starting at the given position.
///
/// # Arguments
///
/// - `&[char]`: The source characters.
/// - `usize`: The starting index.
///
/// # Returns
///
/// - `usize`: The index immediately after the closing `'`.
fn skip_char_literal(chars: &[char], start: usize) -> usize {
    let len: usize = chars.len();
    let mut i: usize = start + 1;
    while i < len {
        if chars[i] == '\\' {
            i += 2;
        } else if chars[i] == '\'' {
            return i + 1;
        } else {
            i += 1;
        }
    }
    len
}

/// Skips a line comment starting at `//`.
///
/// # Arguments
///
/// - `&[char]`: The source characters.
/// - `usize`: The position of the first `/`.
///
/// # Returns
///
/// - `usize`: The index of the first character after the newline.
fn skip_line_comment(chars: &[char], start: usize) -> usize {
    let len: usize = chars.len();
    let mut i: usize = start + 2;
    while i < len && chars[i] != '\n' {
        i += 1;
    }
    if i < len { i + 1 } else { len }
}

/// Skips a block comment starting at `/*`.
///
/// # Arguments
///
/// - `&[char]`: The source characters.
/// - `usize`: The position of the first `/`.
///
/// # Returns
///
/// - `usize`: The index immediately after `*/`.
fn skip_block_comment(chars: &[char], start: usize) -> usize {
    let len: usize = chars.len();
    let mut i: usize = start + 2;
    while i + 1 < len {
        if chars[i] == '*' && chars[i + 1] == '/' {
            return i + 2;
        }
        i += 1;
    }
    len
}

/// Finds the matching `}` for an opening `{` at the given position.
///
/// # Arguments
///
/// - `&[char]`: The source characters.
/// - `usize`: The position of the opening `{`.
///
/// # Returns
///
/// - `usize`: The position of the matching `}`.
fn find_matching_brace(chars: &[char], open: usize) -> usize {
    let len: usize = chars.len();
    let mut depth: i32 = 1;
    let mut i: usize = open + 1;
    while i < len && depth > 0 {
        match chars[i] {
            '"' => {
                i = skip_string_literal(chars, i);
                continue;
            }
            '\'' => {
                if is_char_literal(chars, i) {
                    i = skip_char_literal(chars, i);
                    continue;
                }
                i += 1;
                continue;
            }
            '/' if i + 1 < len && chars[i + 1] == '/' => {
                i = skip_line_comment(chars, i);
                continue;
            }
            '/' if i + 1 < len && chars[i + 1] == '*' => {
                i = skip_block_comment(chars, i);
                continue;
            }
            '{' => {
                depth += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return i;
                }
            }
            _ => {}
        }
        i += 1;
    }
    len - 1
}

/// Checks if a character can continue a Rust identifier.
///
/// # Arguments
///
/// - `char`: The character to check.
///
/// # Returns
///
/// - `bool`: Whether the character can continue an identifier.
fn is_identifier_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Checks if a character can start a Rust identifier.
///
/// # Arguments
///
/// - `char`: The character to check.
///
/// # Returns
///
/// - `bool`: Whether the character can start an identifier.
fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

/// Formats a brace block in-place by normalizing spacing without
/// restructuring lines or indentation.
///
/// Rules applied:
/// - `{ content }` : after `{`, skip spaces/tabs (not newlines), emit one space;
///   before `}`, remove trailing spaces/tabs on same line, emit one space.
/// - `key: value`  : identifier followed by `:` gets no space before colon,
///   exactly one space after.
///
/// # Arguments
///
/// - `&str`: The raw brace block including outer `{` and `}`.
///
/// # Returns
///
/// - `String`: The formatted block with the same line structure preserved.
pub(crate) fn format_brace_block(block: &str) -> String {
    let chars: Vec<char> = block.chars().collect();
    let len: usize = chars.len();
    let mut result: String = String::new();
    let mut i: usize = 0;
    while i < len {
        if chars[i] == '"' {
            let next: usize = skip_string_literal(&chars, i);
            for c in &chars[i..next] {
                result.push(*c);
            }
            i = next;
        } else if chars[i] == '\'' && is_char_literal(&chars, i) {
            let next: usize = skip_char_literal(&chars, i);
            for c in &chars[i..next] {
                result.push(*c);
            }
            i = next;
        } else if i + 1 < len && chars[i] == '/' && chars[i + 1] == '/' {
            let next: usize = skip_line_comment(&chars, i);
            for c in &chars[i..next] {
                result.push(*c);
            }
            i = next;
        } else if i + 1 < len && chars[i] == '/' && chars[i + 1] == '*' {
            let next: usize = skip_block_comment(&chars, i);
            for c in &chars[i..next] {
                result.push(*c);
            }
            i = next;
        } else if chars[i] == '{' {
            if !result.is_empty() {
                let last: u8 = result.as_bytes()[result.len() - 1];
                if last != b' ' && last != b'\t' && last != b'\n' && last != b'{' && last != b'(' {
                    result.push(' ');
                }
            }
            result.push('{');
            let mut j: usize = i + 1;
            let mut found_newline: bool = false;
            while j < len && (chars[j] == ' ' || chars[j] == '\t') {
                j += 1;
            }
            if j < len && chars[j] == '\n' {
                found_newline = true;
            }
            if !found_newline && j < len && chars[j] != '}' {
                result.push(' ');
            }
            i = j;
        } else if chars[i] == '}' {
            let mut j: usize = result.len();
            while j > 0 && (result.as_bytes()[j - 1] == b' ' || result.as_bytes()[j - 1] == b'\t') {
                j -= 1;
            }
            if j > 0 && result.as_bytes()[j - 1] == b'\n' {
                result.push('}');
            } else {
                result.truncate(j);
                if j > 0 {
                    result.push(' ');
                }
                result.push('}');
            }
            i += 1;
        } else if is_identifier_start(chars[i])
            || (chars[i] == 'r' && i + 1 < len && chars[i + 1] == '#')
        {
            let (ident, next) = consume_identifier(&chars, i, len);
            let ws_end: usize = skip_horizontal_ws(&chars, next, len);
            if ws_end < len
                && chars[ws_end] == ':'
                && (ws_end + 1 >= len || chars[ws_end + 1] != ':')
            {
                result.push_str(&ident);
                result.push(':');
                let after_colon: usize = skip_horizontal_ws(&chars, ws_end + 1, len);
                if after_colon < len && chars[after_colon] != '\n' && chars[after_colon] != '\r' {
                    result.push(' ');
                    i = after_colon;
                } else {
                    result.push(' ');
                    i = ws_end + 1;
                }
            } else {
                result.push_str(&ident);
                i = next;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

/// Consumes an identifier (including `r#` and kebab-case) starting at position `i`.
///
/// # Arguments
///
/// - `&[char]`: The source characters.
/// - `usize`: The starting index.
/// - `usize`: The total length.
///
/// # Returns
///
/// - `(String, usize)`: The consumed identifier and the next position.
fn consume_identifier(chars: &[char], i: usize, len: usize) -> (String, usize) {
    let mut result: String = String::new();
    let mut j: usize = i;
    if j + 1 < len && chars[j] == 'r' && chars[j + 1] == '#' {
        result.push('r');
        result.push('#');
        j += 2;
    }
    while j < len && is_identifier_continue(chars[j]) {
        result.push(chars[j]);
        j += 1;
    }
    while j + 1 < len && chars[j] == '-' && is_identifier_start(chars[j + 1]) {
        result.push('-');
        j += 1;
        if j + 1 < len && chars[j] == 'r' && chars[j + 1] == '#' {
            result.push('r');
            result.push('#');
            j += 2;
        }
        while j < len && is_identifier_continue(chars[j]) {
            result.push(chars[j]);
            j += 1;
        }
    }
    (result, j)
}

/// Skips horizontal whitespace (spaces and tabs) forward from position `i`.
///
/// # Arguments
///
/// - `&[char]`: The source characters.
/// - `usize`: The starting index.
/// - `usize`: The total length.
///
/// # Returns
///
/// - `usize`: The index of the first non-horizontal-whitespace character.
fn skip_horizontal_ws(chars: &[char], mut i: usize, len: usize) -> usize {
    while i < len && (chars[i] == ' ' || chars[i] == '\t') {
        i += 1;
    }
    i
}
