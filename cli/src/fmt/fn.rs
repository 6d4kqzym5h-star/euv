use crate::*;

/// Formats a single Rust source file by reformatting all euv macro invocations.
///
/// Scans the source code for `html!`, `class!`, `vars!`, and `watch!`
/// macro calls, then applies formatting rules to their token bodies:
///
/// - Tag name and `{` separated by exactly one space
/// - Attribute name immediately followed by `:`, then one space before the value
/// - `if { expr }` with single spaces around each token
/// - `match { expr }` with proper alignment
/// - `for pattern in { expr }` with proper alignment
/// - `=>` in watch! macros with proper spacing
///
/// # Arguments
///
/// - `&str` - The Rust source code content.
///
/// # Returns
///
/// - `FmtResult` - The formatting result indicating whether changes were made.
pub(crate) fn format_source(source: &str) -> FmtResult {
    let formatted: String = format_euv_macros(source);
    let changed: bool = formatted != source;
    FmtResult::new(changed, formatted)
}

/// Finds and reformats all euv macro invocations in the source text.
///
/// Uses a character-level scanner to locate macro calls (e.g., `html! { ... }`),
/// tracks brace depth to find the complete macro body, then formats the body
/// using `format_macro_body`.
///
/// # Arguments
///
/// - `&str` - The Rust source code content.
///
/// # Returns
///
/// - `String` - The source with all euv macros reformatted.
fn format_euv_macros(source: &str) -> String {
    let mut result: String = String::new();
    let chars: Vec<char> = source.chars().collect();
    let len: usize = chars.len();
    let mut position: usize = 0;
    while position < len {
        if is_euv_macro_start(&chars, position, len) {
            let macro_name_end: usize = find_macro_name_end(&chars, position, len);
            let name: String = chars[position..macro_name_end].iter().collect::<String>();
            result.push_str(&name);
            position = macro_name_end;
            position = skip_whitespace_and_comments(&chars, position, len, &mut result);
            if position < len && chars[position] == CHAR_MACRO_BANG {
                result.push(CHAR_MACRO_BANG);
                position += 1;
                position = skip_whitespace_and_comments(&chars, position, len, &mut result);
                if position < len && chars[position] == CHAR_BRACE_LEFT {
                    if !result.ends_with(CHAR_SPACE) {
                        result.push(CHAR_SPACE);
                    }
                    let (body_content, end_pos) = extract_brace_content(&chars, position);
                    let formatted_body: String = format_macro_body(&body_content);
                    if formatted_body.trim().is_empty() {
                        result.push(CHAR_BRACE_LEFT);
                        result.push(CHAR_BRACE_RIGHT);
                    } else {
                        let trimmed_body: &str = formatted_body.trim();
                        let last_newline_pos: usize =
                            result.rfind(CHAR_NEWLINE).map(|i| i + 1).unwrap_or(0);
                        let macro_indent: usize = result[last_newline_pos..]
                            .chars()
                            .take_while(|&c| c == CHAR_SPACE)
                            .count();
                        let min_indent: usize = body_content
                            .lines()
                            .filter(|line| !line.trim().is_empty())
                            .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
                            .min()
                            .unwrap_or(0);
                        let base_indent: usize = if min_indent > 0 {
                            min_indent
                        } else {
                            macro_indent + 4
                        };
                        let indent_str: String = " ".repeat(base_indent);
                        let indented_body: String = trimmed_body
                            .lines()
                            .map(|line| {
                                if line.trim().is_empty() {
                                    line.to_string()
                                } else {
                                    format!("{}{}", indent_str, line)
                                }
                            })
                            .collect::<Vec<String>>()
                            .join("\n");
                        let outer_indent_str: String = " ".repeat(macro_indent);
                        result.push(CHAR_BRACE_LEFT);
                        result.push(CHAR_NEWLINE);
                        result.push_str(&indented_body);
                        result.push(CHAR_NEWLINE);
                        result.push_str(&outer_indent_str);
                        result.push(CHAR_BRACE_RIGHT);
                    }
                    position = end_pos;
                    continue;
                }
            }
            continue;
        }
        if chars[position] == CHAR_DOUBLE_QUOTE || chars[position] == CHAR_SINGLE_QUOTE {
            let (literal, end_pos) = extract_string_literal(&chars, position, len);
            result.push_str(&literal);
            position = end_pos;
            continue;
        }
        if position + 1 < len
            && chars[position] == CHAR_SLASH_FORWARD
            && chars[position + 1] == CHAR_SLASH_FORWARD
        {
            let (comment, end_pos) = extract_line_comment(&chars, position, len);
            result.push_str(&comment);
            position = end_pos;
            continue;
        }
        if position + 1 < len
            && chars[position] == CHAR_SLASH_FORWARD
            && chars[position + 1] == CHAR_ASTERISK
        {
            let (comment, end_pos) = extract_block_comment(&chars, position, len);
            result.push_str(&comment);
            position = end_pos;
            continue;
        }
        result.push(chars[position]);
        position += 1;
    }
    result
}

/// Checks whether the current position starts a known euv macro name.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The current position.
/// - `usize` - The total length of the source.
///
/// # Returns
///
/// - `bool` - Whether a euv macro name starts at the given position.
fn is_euv_macro_start(chars: &[char], pos: usize, len: usize) -> bool {
    for name in EUV_MACRO_NAMES {
        let name_len: usize = name.len();
        if pos + name_len > len {
            continue;
        }
        let candidate: String = chars[pos..pos + name_len].iter().collect::<String>();
        if candidate != *name {
            continue;
        }
        if pos > 0 && is_ident_char(chars[pos - 1]) {
            continue;
        }
        if pos + name_len < len && is_ident_char(chars[pos + name_len]) {
            continue;
        }
        return true;
    }
    false
}

/// Finds the end position of a macro name starting at the given position.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The start position of the macro name.
/// - `usize` - The total length of the source.
///
/// # Returns
///
/// - `usize` - The end position (exclusive) of the macro name.
fn find_macro_name_end(chars: &[char], pos: usize, _len: usize) -> usize {
    let mut end: usize = pos;
    while end < chars.len() && is_ident_char(chars[end]) {
        end += 1;
    }
    end
}

/// Checks whether the keyword at the given position is preceded by `r#`,
/// indicating a Rust raw identifier rather than a keyword.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The position of the keyword.
///
/// # Returns
///
/// - `bool` - Whether the keyword is preceded by `r#`.
fn is_raw_prefix(chars: &[char], pos: usize) -> bool {
    if pos < 2 {
        return false;
    }
    chars[pos - 2] == CHAR_LETTER_R && chars[pos - 1] == CHAR_HASH
}

/// Checks if a character can be part of a Rust identifier.
///
/// # Arguments
///
/// - `char` - The character to check.
///
/// # Returns
///
/// - `bool` - `true` if the character is alphanumeric or underscore.
fn is_ident_char(character: char) -> bool {
    character.is_alphanumeric() || character == CHAR_UNDERSCORE
}

/// Skips whitespace and comments, preserving them in the output.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The current position.
/// - `usize` - The total length.
/// - `&mut String` - The output string to append preserved whitespace/comments to.
///
/// # Returns
///
/// - `usize` - The new position after skipping whitespace and comments.
fn skip_whitespace_and_comments(
    chars: &[char],
    mut pos: usize,
    len: usize,
    result: &mut String,
) -> usize {
    while pos < len {
        if chars[pos].is_whitespace() {
            result.push(chars[pos]);
            pos += 1;
        } else if pos + 1 < len
            && chars[pos] == CHAR_SLASH_FORWARD
            && chars[pos + 1] == CHAR_SLASH_FORWARD
        {
            let (comment, end_pos) = extract_line_comment(chars, pos, len);
            result.push_str(&comment);
            pos = end_pos;
        } else if pos + 1 < len
            && chars[pos] == CHAR_SLASH_FORWARD
            && chars[pos + 1] == CHAR_ASTERISK
        {
            let (comment, end_pos) = extract_block_comment(chars, pos, len);
            result.push_str(&comment);
            pos = end_pos;
        } else {
            break;
        }
    }
    pos
}

/// Extracts a brace-enclosed block verbatim (including the braces).
///
/// Assumes `chars[start]` is `{`. Returns the full content (including the
/// outer `{` and `}`) and the position after the closing `}`.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The position of the opening `{`.
///
/// # Returns
///
/// - `(String, usize)` - The content including braces and the position after the closing `}`.
fn extract_brace_block(chars: &[char], start: usize) -> (String, usize) {
    let mut depth: i32 = 0;
    let mut position: usize = start;
    while position < chars.len() {
        if chars[position] == CHAR_DOUBLE_QUOTE || chars[position] == CHAR_SINGLE_QUOTE {
            let (_, end) = extract_string_literal(chars, position, chars.len());
            position = end;
            continue;
        }
        if position + 1 < chars.len()
            && chars[position] == CHAR_SLASH_FORWARD
            && chars[position + 1] == CHAR_SLASH_FORWARD
        {
            let (_, end) = extract_line_comment(chars, position, chars.len());
            position = end;
            continue;
        }
        if position + 1 < chars.len()
            && chars[position] == CHAR_SLASH_FORWARD
            && chars[position + 1] == CHAR_ASTERISK
        {
            let (_, end) = extract_block_comment(chars, position, chars.len());
            position = end;
            continue;
        }
        if chars[position] == CHAR_BRACE_LEFT {
            depth += 1;
        } else if chars[position] == CHAR_BRACE_RIGHT {
            depth -= 1;
            if depth == 0 {
                let content: String = chars[start..=position].iter().collect();
                return (content, position + 1);
            }
        }
        position += 1;
    }
    let content: String = chars[start..].iter().collect();
    (content, chars.len())
}

/// Extracts the content inside a brace-delimited block (without the outer braces).
///
/// Assumes `chars[start]` is `{`. Returns the inner content (without the
/// outer braces) and the position after the closing `}`.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The position of the opening `{`.
///
/// # Returns
///
/// - `(String, usize)` - The content inside the braces and the position after the closing `}`.
fn extract_brace_content(chars: &[char], start: usize) -> (String, usize) {
    let mut depth: i32 = 0;
    let mut position: usize = start;
    let mut content_start: usize = start + 1;
    while position < chars.len() {
        if chars[position] == CHAR_DOUBLE_QUOTE || chars[position] == CHAR_SINGLE_QUOTE {
            let (_, end) = extract_string_literal(chars, position, chars.len());
            position = end;
            continue;
        }
        if position + 1 < chars.len()
            && chars[position] == CHAR_SLASH_FORWARD
            && chars[position + 1] == CHAR_SLASH_FORWARD
        {
            let (_, end) = extract_line_comment(chars, position, chars.len());
            position = end;
            continue;
        }
        if position + 1 < chars.len()
            && chars[position] == CHAR_SLASH_FORWARD
            && chars[position + 1] == CHAR_ASTERISK
        {
            let (_, end) = extract_block_comment(chars, position, chars.len());
            position = end;
            continue;
        }
        if chars[position] == CHAR_BRACE_LEFT {
            if depth == 0 {
                content_start = position + 1;
            }
            depth += 1;
        } else if chars[position] == CHAR_BRACE_RIGHT {
            depth -= 1;
            if depth == 0 {
                let content: String = chars[content_start..position].iter().collect();
                return (content, position + 1);
            }
        }
        position += 1;
    }
    let content: String = chars[content_start..].iter().collect();
    (content, chars.len())
}

/// Extracts a string literal (single-quoted or double-quoted) from the source.
///
/// Handles escape sequences (`\"`, `\'`, `\\`).
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The start position (at the opening quote).
/// - `usize` - The total length.
///
/// # Returns
///
/// - `(String, usize)` - The literal text and the position after the closing quote.
fn extract_string_literal(chars: &[char], start: usize, len: usize) -> (String, usize) {
    let quote: char = chars[start];
    let mut position: usize = start + 1;
    let mut result: String = String::new();
    result.push(quote);
    while position < len {
        if chars[position] == CHAR_SLASH_BACK && position + 1 < len {
            result.push(chars[position]);
            result.push(chars[position + 1]);
            position += 2;
            continue;
        }
        result.push(chars[position]);
        if chars[position] == quote {
            return (result, position + 1);
        }
        position += 1;
    }
    (result, position)
}

/// Extracts a line comment (`// ...`) from the source.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The start position (at the first `/`).
/// - `usize` - The total length.
///
/// # Returns
///
/// - `(String, usize)` - The comment text and the position after the newline.
fn extract_line_comment(chars: &[char], start: usize, len: usize) -> (String, usize) {
    let mut position: usize = start;
    let mut result: String = String::new();
    while position < len && chars[position] != CHAR_NEWLINE {
        result.push(chars[position]);
        position += 1;
    }
    if position < len {
        result.push(CHAR_NEWLINE);
        position += 1;
    }
    (result, position)
}

/// Extracts a block comment (`/* ... */`) from the source.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The start position (at the first `/`).
/// - `usize` - The total length.
///
/// # Returns
///
/// - `(String, usize)` - The comment text and the position after the closing `*/`.
fn extract_block_comment(chars: &[char], start: usize, len: usize) -> (String, usize) {
    let mut position: usize = start + 2;
    let mut result: String = String::from(BLOCK_COMMENT_START);
    while position + 1 < len {
        result.push(chars[position]);
        if chars[position] == CHAR_ASTERISK && chars[position + 1] == CHAR_SLASH_FORWARD {
            result.push(CHAR_SLASH_FORWARD);
            return (result, position + 2);
        }
        position += 1;
    }
    while position < len {
        result.push(chars[position]);
        position += 1;
    }
    (result, position)
}

fn format_macro_body(body: &str) -> String {
    let raw: String = format_macro_body_raw(body);
    add_indentation(&raw)
}

/// Formats the body of a euv macro invocation without adding indentation.
///
/// Uses a single-pass scanner that tracks context to apply formatting rules:
///
/// - Tag name and `{` separated by exactly one space
/// - Attribute name immediately followed by `:`, then one space before the value
/// - `if { expr }` / `match { expr }` / `for pattern in { expr }` with proper spacing
/// - `=>` in watch macros with proper spacing
///
/// Content inside expression braces `{ expr }` is preserved verbatim.
/// Only template-level whitespace is normalized.
///
/// # Arguments
///
/// - `&str` - The raw macro body text (without outer braces).
///
/// # Returns
///
/// - `String` - The formatted macro body text.
fn format_macro_body_raw(body: &str) -> String {
    let chars: Vec<char> = body.chars().collect();
    let len: usize = chars.len();
    let mut result: String = String::new();
    let mut position: usize = 0;
    while position < len {
        if chars[position] == CHAR_DOUBLE_QUOTE || chars[position] == CHAR_SINGLE_QUOTE {
            let (literal, end) = extract_string_literal(&chars, position, len);
            result.push_str(&literal);
            position = end;
            continue;
        }
        if position + 1 < len
            && chars[position] == CHAR_SLASH_FORWARD
            && chars[position + 1] == CHAR_SLASH_FORWARD
        {
            let (comment, end) = extract_line_comment(&chars, position, len);
            result.push_str(&comment);
            position = end;
            continue;
        }
        if position + 1 < len
            && chars[position] == CHAR_SLASH_FORWARD
            && chars[position + 1] == CHAR_ASTERISK
        {
            let (comment, end) = extract_block_comment(&chars, position, len);
            result.push_str(&comment);
            position = end;
            continue;
        }
        if is_if_keyword(&chars, position, len) {
            let after_if_colon: usize = skip_spaces_on_same_line(&chars, position + 2, len);
            if after_if_colon < len
                && chars[after_if_colon] == CHAR_COLON
                && (after_if_colon + 1 >= len || chars[after_if_colon + 1] != CHAR_COLON)
            {
                result.push_str(KEYWORD_IF);
                position += 2;
                if position < len && chars[position] == CHAR_SPACE {
                    result.push(CHAR_SPACE);
                    position += 1;
                }
                continue;
            }
            result.push_str(KEYWORD_IF);
            position += 2;
            let after_if: usize = skip_spaces_on_same_line(&chars, position, len);
            if position < len && chars[after_if] == CHAR_BRACE_LEFT {
                result.push(CHAR_SPACE);
                let (block, end) = extract_brace_block(&chars, after_if);
                result.push_str(&format_brace_block(&block));
                position = end;
                position = skip_spaces_on_same_line(&chars, position, len);
                if position < len && chars[position] == CHAR_BRACE_LEFT {
                    result.push(CHAR_SPACE);
                }
            } else {
                result.push(CHAR_SPACE);
                position = after_if;
            }
            continue;
        }
        if is_else_keyword(&chars, position, len) {
            if !result.ends_with(CHAR_SPACE)
                && !result.ends_with(CHAR_NEWLINE)
                && !result.ends_with(CHAR_TAB)
            {
                result.push(CHAR_SPACE);
            }
            result.push_str(KEYWORD_ELSE);
            position += 4;
            position = skip_spaces_on_same_line(&chars, position, len);
            if is_if_keyword(&chars, position, len) {
                result.push(CHAR_SPACE);
                continue;
            }
            if position < len && chars[position] == CHAR_BRACE_LEFT {
                result.push(CHAR_SPACE);
            }
            continue;
        }
        if is_match_keyword(&chars, position, len) {
            let after_match_colon: usize = skip_spaces_on_same_line(&chars, position + 5, len);
            if after_match_colon < len
                && chars[after_match_colon] == CHAR_COLON
                && (after_match_colon + 1 >= len || chars[after_match_colon + 1] != CHAR_COLON)
            {
                result.push_str(KEYWORD_MATCH);
                position += 5;
                if position < len && chars[position] == CHAR_SPACE {
                    result.push(CHAR_SPACE);
                    position += 1;
                }
                continue;
            }
            result.push_str(KEYWORD_MATCH);
            position += 5;
            let after_match: usize = skip_spaces_on_same_line(&chars, position, len);
            if after_match < len && chars[after_match] == CHAR_BRACE_LEFT {
                result.push(CHAR_SPACE);
                let (block, end) = extract_brace_block(&chars, after_match);
                result.push_str(&format_brace_block(&block));
                position = end;
                position = skip_spaces_on_same_line(&chars, position, len);
                if position < len && chars[position] == CHAR_BRACE_LEFT {
                    result.push(CHAR_SPACE);
                }
            } else {
                result.push(CHAR_SPACE);
                position = after_match;
            }
            continue;
        }
        if is_for_keyword(&chars, position, len) {
            let after_for: usize = skip_spaces_on_same_line(&chars, position + 3, len);
            if after_for < len
                && chars[after_for] == CHAR_COLON
                && (after_for + 1 >= len || chars[after_for + 1] != CHAR_COLON)
            {
                result.push_str(KEYWORD_FOR);
                position += 3;
                if position < len && chars[position] == CHAR_SPACE {
                    result.push(CHAR_SPACE);
                    position += 1;
                }
                continue;
            }
            result.push_str(KEYWORD_FOR);
            position += 3;
            position = skip_spaces_on_same_line(&chars, position, len);
            if position < len && !is_in_keyword(&chars, position, len) {
                result.push(CHAR_SPACE);
            }
            while position < len && !is_in_keyword(&chars, position, len) {
                if chars[position] == CHAR_BRACE_LEFT {
                    let (block, end) = extract_brace_block(&chars, position);
                    result.push_str(&format_brace_block(&block));
                    position = end;
                    continue;
                }
                if chars[position] == CHAR_DOUBLE_QUOTE || chars[position] == CHAR_SINGLE_QUOTE {
                    let (literal, end) = extract_string_literal(&chars, position, len);
                    result.push_str(&literal);
                    position = end;
                    continue;
                }
                result.push(chars[position]);
                position += 1;
            }
            if result.ends_with(CHAR_SPACE) {
                result.truncate(result.len() - 1);
            }
            position = skip_spaces_on_same_line(&chars, position, len);
            if is_in_keyword(&chars, position, len) {
                result.push(CHAR_SPACE);
                result.push_str(KEYWORD_IN);
                position += 2;
                position = skip_spaces_on_same_line(&chars, position, len);
                if position < len && chars[position] == CHAR_BRACE_LEFT {
                    result.push(CHAR_SPACE);
                    let (block, end) = extract_brace_block(&chars, position);
                    result.push_str(&format_brace_block(&block));
                    position = end;
                    position = skip_spaces_on_same_line(&chars, position, len);
                    if position < len && chars[position] == CHAR_BRACE_LEFT {
                        result.push(CHAR_SPACE);
                    }
                }
            }
            continue;
        }
        if chars[position] == CHAR_COLON && position + 1 < len && chars[position + 1] != CHAR_COLON
        {
            if result.ends_with(CHAR_COLON) {
                result.push(CHAR_COLON);
                position += 1;
                continue;
            }
            let colon_prefix: String = find_ident_before(&result);
            if is_raw_ident_before(&result, &colon_prefix) {
                result.push(CHAR_COLON);
                position += 1;
                continue;
            }
            if !colon_prefix.is_empty() {
                let before_colon: String = remove_trailing_spaces(&result, colon_prefix.len());
                result = before_colon;
                result.push_str(&colon_prefix);
            }
            result.push(CHAR_COLON);
            position += 1;
            while position < len && (chars[position] == CHAR_SPACE || chars[position] == CHAR_TAB) {
                position += 1;
            }
            if position < len
                && chars[position] != CHAR_NEWLINE
                && chars[position] != CHAR_CARRIAGE_RETURN
                && !is_pseudo_selector_after_colon(&chars, position, len)
            {
                result.push(CHAR_SPACE);
            }
            continue;
        }
        if position + 1 < len
            && chars[position] == CHAR_EQUALS
            && chars[position + 1] == CHAR_GREATER_THAN
        {
            let trailing: String = find_trailing_spaces(&result);
            if !trailing.is_empty() {
                result.truncate(result.len() - trailing.len());
            }
            result.push(CHAR_SPACE);
            result.push_str(ARROW_FAT);
            position += 2;
            while position < len && (chars[position] == CHAR_SPACE || chars[position] == CHAR_TAB) {
                position += 1;
            }
            result.push(CHAR_SPACE);
            continue;
        }
        if chars[position] == CHAR_BRACE_LEFT {
            let (inner, end) = extract_brace_content(&chars, position);
            let trimmed_inner: &str = inner.trim();
            if trimmed_inner.is_empty() {
                result.push(CHAR_BRACE_LEFT);
                result.push(CHAR_BRACE_RIGHT);
            } else {
                let formatted_inner: String = format_macro_body_raw(trimmed_inner);
                result.push(CHAR_BRACE_LEFT);
                result.push(CHAR_NEWLINE);
                result.push_str(&formatted_inner);
                result.push(CHAR_NEWLINE);
                result.push(CHAR_BRACE_RIGHT);
            }
            position = end;
            continue;
        }
        if is_ident_char(chars[position]) {
            let start: usize = position;
            while position < len && is_ident_char(chars[position]) {
                position += 1;
            }
            let ident: String = chars[start..position].iter().collect();
            result.push_str(&ident);
            let ws_start: usize = position;
            while position < len && (chars[position] == CHAR_SPACE || chars[position] == CHAR_TAB) {
                position += 1;
            }
            let had_whitespace: bool = position > ws_start;
            if position < len && chars[position] == CHAR_BRACE_LEFT {
                result.push(CHAR_SPACE);
            } else if had_whitespace {
                let next_pos: usize = position;
                if next_pos < len && is_ident_char(chars[next_pos]) {
                    let mut ident_end: usize = next_pos;
                    while ident_end < len && is_ident_char(chars[ident_end]) {
                        ident_end += 1;
                    }
                    let after_ident: usize = skip_spaces_on_same_line(&chars, ident_end, len);
                    if after_ident < len
                        && chars[after_ident] == CHAR_COLON
                        && (after_ident + 1 >= len || chars[after_ident + 1] != CHAR_COLON)
                    {
                        result.push(CHAR_NEWLINE);
                        continue;
                    }
                }
                let ws: String = chars[ws_start..position].iter().collect();
                result.push_str(&ws);
            }
            continue;
        }
        result.push(chars[position]);
        position += 1;
    }
    result
}

/// Adds 4-space indentation to each line based on brace depth.
///
/// String literals and comments are skipped so braces inside them do not affect depth.
/// The first non-empty line receives a base indent of 4 spaces (depth 1).
///
/// # Arguments
///
/// - `&str` - The raw formatted macro body text.
///
/// # Returns
///
/// - `String` - The indented macro body text.
fn add_indentation(body: &str) -> String {
    let chars: Vec<char> = body.chars().collect();
    let len: usize = chars.len();
    let mut result: String = String::new();
    let mut depth: i32 = 0;
    let mut i: usize = 0;

    while i < len {
        if chars[i] == CHAR_DOUBLE_QUOTE || chars[i] == CHAR_SINGLE_QUOTE {
            let quote: char = chars[i];
            result.push(chars[i]);
            i += 1;
            while i < len {
                if chars[i] == CHAR_SLASH_BACK && i + 1 < len {
                    result.push(chars[i]);
                    result.push(chars[i + 1]);
                    i += 2;
                    continue;
                }
                result.push(chars[i]);
                if chars[i] == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }

        if i + 1 < len && chars[i] == CHAR_SLASH_FORWARD && chars[i + 1] == CHAR_SLASH_FORWARD {
            while i < len && chars[i] != CHAR_NEWLINE {
                result.push(chars[i]);
                i += 1;
            }
            continue;
        }

        if i + 1 < len && chars[i] == CHAR_SLASH_FORWARD && chars[i + 1] == CHAR_ASTERISK {
            result.push(chars[i]);
            result.push(chars[i + 1]);
            i += 2;
            while i + 1 < len {
                if chars[i] == CHAR_ASTERISK && chars[i + 1] == CHAR_SLASH_FORWARD {
                    result.push(chars[i]);
                    result.push(chars[i + 1]);
                    i += 2;
                    break;
                }
                result.push(chars[i]);
                i += 1;
            }
            continue;
        }

        if chars[i] == CHAR_NEWLINE {
            result.push(CHAR_NEWLINE);
            i += 1;
            while i < len && (chars[i] == CHAR_SPACE || chars[i] == CHAR_TAB) {
                i += 1;
            }
            if i >= len || chars[i] == CHAR_NEWLINE || chars[i] == CHAR_CARRIAGE_RETURN {
                continue;
            }
            let mut closing_count: i32 = 0;
            let mut j: usize = i;
            while j < len && chars[j] == CHAR_BRACE_RIGHT {
                closing_count += 1;
                j += 1;
            }
            let indent_depth: i32 = (depth - closing_count).max(0);
            for _ in 0..indent_depth * 4 {
                result.push(CHAR_SPACE);
            }
            continue;
        }

        if chars[i] == CHAR_BRACE_LEFT {
            depth += 1;
            result.push(CHAR_BRACE_LEFT);
            i += 1;
            continue;
        }

        if chars[i] == CHAR_BRACE_RIGHT {
            depth -= 1;
            result.push(CHAR_BRACE_RIGHT);
            i += 1;
            let mut j: usize = i;
            while j < len && (chars[j] == CHAR_SPACE || chars[j] == CHAR_TAB) {
                j += 1;
            }
            if j < len
                && chars[j] != CHAR_BRACE_RIGHT
                && chars[j] != CHAR_BRACE_LEFT
                && chars[j] != CHAR_NEWLINE
                && chars[j] != CHAR_CARRIAGE_RETURN
                && chars[j] != CHAR_RIGHT_PAREN
                && chars[j] != CHAR_COMMA
                && chars[j] != CHAR_SEMICOLON
            {
                let next_chars: String = chars[j..(j + 4).min(len)].iter().collect();
                if next_chars != "else" {
                    result.push(CHAR_NEWLINE);
                    let indent_depth: i32 = depth.max(0);
                    for _ in 0..indent_depth * 4 {
                        result.push(CHAR_SPACE);
                    }
                    i = j;
                }
            }
            continue;
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Checks if the current position is the start of the `if` keyword.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The current position.
/// - `usize` - The total length.
///
/// # Returns
///
/// - `bool` - Whether `if` starts at the given position.
fn is_if_keyword(chars: &[char], pos: usize, len: usize) -> bool {
    if pos + 2 > len {
        return false;
    }
    chars[pos] == CHAR_LETTER_I
        && chars[pos + 1] == CHAR_LETTER_F
        && (pos + 2 >= len || !is_ident_char(chars[pos + 2]))
        && (pos == 0 || !is_ident_char(chars[pos - 1]))
        && !is_raw_prefix(chars, pos)
}

/// Checks if the current position is the start of the `else` keyword.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The current position.
/// - `usize` - The total length.
///
/// # Returns
///
/// - `bool` - Whether `else` starts at the given position.
fn is_else_keyword(chars: &[char], pos: usize, len: usize) -> bool {
    if pos + 4 > len {
        return false;
    }
    chars[pos] == CHAR_LETTER_E
        && chars[pos + 1] == CHAR_LETTER_L
        && chars[pos + 2] == CHAR_LETTER_S
        && chars[pos + 3] == CHAR_LETTER_E
        && (pos + 4 >= len || !is_ident_char(chars[pos + 4]))
        && (pos == 0 || !is_ident_char(chars[pos - 1]))
        && !is_raw_prefix(chars, pos)
}

/// Checks if the current position is the start of the `match` keyword.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The current position.
/// - `usize` - The total length.
///
/// # Returns
///
/// - `bool` - Whether `match` starts at the given position.
fn is_match_keyword(chars: &[char], pos: usize, len: usize) -> bool {
    if pos + 5 > len {
        return false;
    }
    chars[pos] == CHAR_LETTER_M
        && chars[pos + 1] == CHAR_LETTER_A
        && chars[pos + 2] == CHAR_LETTER_T
        && chars[pos + 3] == CHAR_LETTER_C
        && chars[pos + 4] == CHAR_LETTER_H
        && (pos + 5 >= len || !is_ident_char(chars[pos + 5]))
        && (pos == 0 || !is_ident_char(chars[pos - 1]))
        && !is_raw_prefix(chars, pos)
}

/// Checks if the current position is the start of the `for` keyword.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The current position.
/// - `usize` - The total length.
///
/// # Returns
///
/// - `bool` - Whether `for` starts at the given position.
fn is_for_keyword(chars: &[char], pos: usize, len: usize) -> bool {
    if pos + 3 > len {
        return false;
    }
    chars[pos] == CHAR_LETTER_F
        && chars[pos + 1] == CHAR_LETTER_O
        && chars[pos + 2] == CHAR_LETTER_R
        && (pos + 3 >= len || !is_ident_char(chars[pos + 3]))
        && (pos == 0 || !is_ident_char(chars[pos - 1]))
        && !is_raw_prefix(chars, pos)
}

/// Checks if the current position is the start of the `in` keyword.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The current position.
/// - `usize` - The total length.
///
/// # Returns
///
/// - `bool` - Whether `in` starts at the given position.
fn is_in_keyword(chars: &[char], pos: usize, len: usize) -> bool {
    if pos + 2 > len {
        return false;
    }
    chars[pos] == CHAR_LETTER_I
        && chars[pos + 1] == CHAR_LETTER_N
        && (pos + 2 >= len || !is_ident_char(chars[pos + 2]))
        && (pos == 0 || !is_ident_char(chars[pos - 1]))
        && !is_raw_prefix(chars, pos)
}

/// Formats a brace block by adding spaces inside single-line braces.
///
/// For blocks that do not contain newlines, trims inner content and adds
/// single spaces around it: `{ content }`. Empty blocks remain `{}`.
/// For blocks containing newlines, returns the block unchanged.
///
/// # Arguments
///
/// - `&str` - The brace block text (including outer `{` and `}`).
///
/// # Returns
///
/// - `String` - The formatted brace block text.
fn format_brace_block(block: &str) -> String {
    let block_chars: Vec<char> = block.chars().collect();
    if block_chars.len() < 2
        || block_chars[0] != CHAR_BRACE_LEFT
        || *block_chars.last().unwrap() != CHAR_BRACE_RIGHT
    {
        return block.to_string();
    }
    let inner: String = block_chars[1..block_chars.len() - 1].iter().collect();
    if inner.contains(CHAR_NEWLINE) {
        return block.to_string();
    }
    let trimmed_inner: &str = inner.trim();
    if trimmed_inner.is_empty() {
        let mut empty_result: String = String::new();
        empty_result.push(CHAR_BRACE_LEFT);
        empty_result.push(CHAR_BRACE_RIGHT);
        return empty_result;
    }
    let mut formatted: String = String::new();
    formatted.push(CHAR_BRACE_LEFT);
    formatted.push(CHAR_SPACE);
    formatted.push_str(trimmed_inner);
    formatted.push(CHAR_SPACE);
    formatted.push(CHAR_BRACE_RIGHT);
    formatted
}

/// Skips spaces (but not newlines) on the same line.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The current position.
/// - `usize` - The total length.
///
/// # Returns
///
/// - `usize` - The position after skipping same-line spaces.
fn skip_spaces_on_same_line(chars: &[char], mut pos: usize, len: usize) -> usize {
    while pos < len && (chars[pos] == CHAR_SPACE || chars[pos] == CHAR_TAB) {
        pos += 1;
    }
    pos
}

/// Checks whether the content after a colon is a CSS pseudo-class or pseudo-element selector.
///
/// A pseudo-class/pseudo-element selector is identified by looking ahead:
/// after the colon, if an identifier is found, and that identifier
/// (possibly followed by more hyphen-separated identifiers like `focus-visible`)
/// is immediately followed by `{` (not `!` which indicates a macro call like `format!`),
/// then this is a selector colon and should not have a space after it.
///
/// Also handles functional pseudo-classes like `:nth-child(2n+1)`, `:not(.class)`,
/// where the identifier is followed by a parenthesized argument list before `{`.
///
/// Returns `false` if the first identifier after the colon is a Rust keyword
/// (e.g., `if`, `match`, `for`) since those are attribute value expressions,
/// not CSS selectors.
///
/// # Arguments
///
/// - `&[char]` - The source character slice.
/// - `usize` - The current position (right after the colon, spaces already skipped).
/// - `usize` - The total length of the source.
///
/// # Returns
///
/// - `bool` - Whether the content after the colon is a CSS selector.
fn is_pseudo_selector_after_colon(chars: &[char], mut pos: usize, len: usize) -> bool {
    if pos >= len || !is_ident_char(chars[pos]) {
        return false;
    }
    let ident_start: usize = pos;
    while pos < len && is_ident_char(chars[pos]) {
        pos += 1;
    }
    let first_ident: String = chars[ident_start..pos].iter().collect();
    if is_rust_keyword(&first_ident) {
        return false;
    }
    while pos < len && chars[pos] == CHAR_HYPHEN && pos + 1 < len && is_ident_char(chars[pos + 1]) {
        pos += 1;
        while pos < len && is_ident_char(chars[pos]) {
            pos += 1;
        }
    }
    let after_ident: usize = skip_spaces_on_same_line(chars, pos, len);
    if after_ident < len && chars[after_ident] == CHAR_BRACE_LEFT {
        return true;
    }
    if after_ident < len && chars[after_ident] == CHAR_LEFT_PAREN {
        let mut depth: i32 = 0;
        let mut paren_pos: usize = after_ident;
        while paren_pos < len {
            if chars[paren_pos] == CHAR_LEFT_PAREN {
                depth += 1;
            } else if chars[paren_pos] == CHAR_RIGHT_PAREN {
                depth -= 1;
                if depth == 0 {
                    let after_paren: usize = skip_spaces_on_same_line(chars, paren_pos + 1, len);
                    return after_paren < len && chars[after_paren] == CHAR_BRACE_LEFT;
                }
            }
            paren_pos += 1;
        }
    }
    false
}

/// Checks whether a string is a Rust keyword that can appear after a colon
/// in euv macro attribute syntax (e.g., `class: if { ... }`).
///
/// These keywords indicate attribute value expressions, not CSS selectors.
///
/// # Arguments
///
/// - `&str` - The identifier string to check.
///
/// # Returns
///
/// - `bool` - Whether the string is a Rust keyword.
fn is_rust_keyword(ident: &str) -> bool {
    matches!(ident, KEYWORD_IF | KEYWORD_MATCH | KEYWORD_FOR)
}

/// Checks whether the identifier found before the colon is a Rust raw identifier (r#prefix).
///
/// Raw identifiers use the `r#` prefix (e.g., `r#for`), and their colons
/// should not be formatted as attribute separators.
///
/// # Arguments
///
/// - `&str` - The result string built so far.
/// - `&str` - The identifier found before the colon.
///
/// # Returns
///
/// - `bool` - Whether the identifier is preceded by `r#`.
fn is_raw_ident_before(result: &str, ident: &str) -> bool {
    result
        .trim_end()
        .ends_with(&format!("{RAW_IDENT_PREFIX}{ident}"))
}

/// Finds the identifier immediately before the current position in the result string.
///
/// # Arguments
///
/// - `&str` - The result string built so far.
///
/// # Returns
///
/// - `String` - The identifier found before the current position, or empty if none.
fn find_ident_before(result: &str) -> String {
    let chars: Vec<char> = result.chars().collect();
    let mut end: usize = chars.len();
    while end > 0 && chars[end - 1] == CHAR_SPACE {
        end -= 1;
    }
    let mut start: usize = end;
    while start > 0 && is_ident_char(chars[start - 1]) {
        start -= 1;
    }
    if start < end {
        chars[start..end].iter().collect()
    } else {
        String::new()
    }
}

/// Removes trailing spaces and the identified prefix from the result string.
///
/// # Arguments
///
/// - `&str` - The result string built so far.
/// - `usize` - The length of the prefix to remove.
///
/// # Returns
///
/// - `String` - The string with trailing spaces and prefix removed.
fn remove_trailing_spaces(result: &str, prefix_len: usize) -> String {
    let chars: Vec<char> = result.chars().collect();
    let total_len: usize = chars.len();
    let mut end: usize = total_len;
    while end > 0 && chars[end - 1] == CHAR_SPACE {
        end -= 1;
    }
    if prefix_len > end {
        return result.to_string();
    }
    let new_end: usize = end - prefix_len;
    chars[..new_end].iter().collect()
}

/// Finds trailing spaces in the result string.
///
/// # Arguments
///
/// - `&str` - The result string.
///
/// # Returns
///
/// - `String` - The trailing spaces.
fn find_trailing_spaces(result: &str) -> String {
    let mut spaces: String = String::new();
    for ch in result.chars().rev() {
        if ch == CHAR_SPACE || ch == CHAR_TAB {
            spaces.push(ch);
        } else {
            break;
        }
    }
    spaces.chars().rev().collect()
}

/// Formats all Rust source files in the given directory that contain euv macros.
///
/// Recursively walks the directory tree, finds `.rs` files, and formats
/// any euv macro invocations found within them.
///
/// # Arguments
///
/// - `&Path` - The root directory to search.
/// - `FmtMode` - Whether to check or write formatting.
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure.
pub(crate) async fn format_dir(path: &Path, mode: FmtMode) -> Result<()> {
    if path.is_file() {
        let changed: bool = format_file(path, &mode).await?;
        match mode {
            FmtMode::Check => {
                if changed {
                    return Err(EuvError::Message(format!(
                        "{} needs formatting.",
                        path.display()
                    )));
                }
                log::info!("{} is properly formatted.", path.display());
            }
            FmtMode::Write => {
                if changed {
                    log::info!("Formatted: {}", path.display());
                } else {
                    log::info!("Already formatted: {}", path.display());
                }
            }
        }
        return Ok(());
    }
    let mut entries: Vec<PathBuf> = collect_rs_files(path).await?;
    entries.sort();
    let mut changed_count: usize = 0;
    let mut unchanged_count: usize = 0;
    for entry in entries {
        match format_file(&entry, &mode).await {
            Ok(changed) => {
                if changed {
                    changed_count += 1;
                } else {
                    unchanged_count += 1;
                }
            }
            Err(error) => {
                log::warn!("Failed to format {}: {error}", entry.display());
            }
        }
    }
    match mode {
        FmtMode::Check => {
            if changed_count > 0 {
                return Err(EuvError::Message(format!(
                    "{} file(s) need formatting. Run `euv fmt` to fix.",
                    changed_count
                )));
            }
            log::info!("All {} file(s) are properly formatted.", unchanged_count);
        }
        FmtMode::Write => {
            log::info!(
                "Formatted {} file(s), {} unchanged.",
                changed_count,
                unchanged_count
            );
        }
    }
    Ok(())
}

/// Collects all `.rs` files in a directory recursively.
///
/// # Arguments
///
/// - `&Path` - The directory to search.
///
/// # Returns
///
/// - `Result<Vec<PathBuf>>` - The list of `.rs` file paths.
async fn collect_rs_files(path: &Path) -> Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = Vec::new();
    let mut stack: Vec<PathBuf> = vec![path.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let mut entries: ReadDir =
            read_dir(&dir)
                .await
                .map_err(|error: io::Error| EuvError::IoPath {
                    message: String::from("Failed to read directory"),
                    path: dir.clone(),
                    error,
                })?;
        while let Some(entry) =
            entries
                .next_entry()
                .await
                .map_err(|error: io::Error| EuvError::IoPath {
                    message: String::from("Failed to read entry in directory"),
                    path: dir.clone(),
                    error,
                })?
        {
            let entry_path: PathBuf = entry.path();
            if entry_path.is_dir() {
                let file_name: String = entry_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                if file_name != TARGET_DIR_NAME && file_name != NODE_MODULES_DIR_NAME {
                    stack.push(entry_path);
                }
            } else if entry_path
                .extension()
                .is_some_and(|ext: &std::ffi::OsStr| ext == RS_EXTENSION)
            {
                result.push(entry_path);
            }
        }
    }
    Ok(result)
}

/// Formats a single Rust source file.
///
/// # Arguments
///
/// - `&Path` - The file path.
/// - `&FmtMode` - Whether to check or write formatting.
///
/// # Returns
///
/// - `Result<bool>` - Whether the file was changed.
async fn format_file(path: &Path, mode: &FmtMode) -> Result<bool> {
    let content: String =
        read_to_string(path)
            .await
            .map_err(|error: io::Error| EuvError::IoPath {
                message: String::from("Failed to read"),
                path: path.to_path_buf(),
                error,
            })?;
    let fmt_result: FmtResult = format_source(&content);
    if fmt_result.get_changed() {
        match mode {
            FmtMode::Write => {
                write(path, fmt_result.get_output())
                    .await
                    .map_err(|error: io::Error| EuvError::IoPath {
                        message: String::from("Failed to write"),
                        path: path.to_path_buf(),
                        error,
                    })?;
                log::info!("Formatted: {}", path.display());
            }
            FmtMode::Check => {
                log::warn!("Needs formatting: {}", path.display());
            }
        }
    }
    Ok(fmt_result.get_changed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_if() {
        let input = "if{a}{b}";
        let expected = "if { a } {\n    b\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_if_with_else() {
        let input = "if{a}{b}else{c}";
        let expected = "if { a } {\n    b\n} else {\n    c\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_if_else_if_else() {
        let input = "if{a}{b}else if{c}{d}else{e}";
        let expected = "if { a } {\n    b\n} else if { c } {\n    d\n} else {\n    e\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_nested_if() {
        let input = "if{a}{if{b}{c}}";
        let expected = "if { a } {\n    if { b } {\n        c\n    }\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_match_single_arm() {
        let input = "match{a}{\"b\"=>{c}}";
        let expected = "match { a } {\n    \"b\" => {\n        c\n    }\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_match_multiple_arms() {
        let input = "match{a}{\"b\"=>{c}\"d\"=>{e}}";
        let expected =
            "match { a } {\n    \"b\" => {\n        c\n    }\n    \"d\" => {\n        e\n    }\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_for_loop() {
        let input = "for i in {a}{b}";
        let expected = "for i in { a } {\n    b\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_simple_div() {
        let input = "div{a}";
        let expected = "div {\n    a\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_nested_div() {
        let input = "div{a{b}c}";
        let expected = "div {\n    a {\n        b\n    }\n    c\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_empty_block() {
        let input = "div{}";
        let expected = "div {}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_bare_empty_block() {
        let input = "{}";
        let expected = "{}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_string_in_body() {
        let input = "div{\"hello\"}";
        let expected = "div {\n    \"hello\"\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_braces_in_string() {
        let input = "div{\"hello { world }\"}";
        let expected = "div {\n    \"hello { world }\"\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_comment_in_body() {
        let input = "div{// comment\n    a\n}";
        let expected = "div {\n    // comment\n    a\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_class_macro() {
        let input = "class!{c_app_root{display:\"flex\"}}";
        let expected = "class!{\n    c_app_root {\n        display: \"flex\"\n    }\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_vars_macro() {
        let input = "vars!{a:\"1\";b:\"2\"}";
        let expected = "vars!{\n    a: \"1\";b: \"2\"\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_deeply_nested() {
        let input = "div{a{div{b{div{c}}}}}";
        let expected = concat!(
            "div {\n",
            "    a {\n",
            "        div {\n",
            "            b {\n",
            "                div {\n",
            "                    c\n",
            "                }\n",
            "            }\n",
            "        }\n",
            "    }\n",
            "}"
        );
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_multiple_siblings() {
        let input = "div{a\n    b\n    c}";
        let expected = "div {\n    a\n    b\n    c\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_if_with_complex_condition() {
        let input = "if{a>0}{b}";
        let expected = "if { a>0 } {\n    b\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_watch_macro() {
        let input = "watch!{signal=>{callback}}";
        let expected = "watch!{\n    signal => {\n        callback\n    }\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_pseudo_selector() {
        let input = "div{:hover{color:\"red\"}}";
        let expected = "div {\n    :hover {\n        color: \"red\"\n    }\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_media_query() {
        let input = "div{@media{(max-width: 767px){display:\"none\"}}}";
        let expected = "div {\n    @media {\n        (max-width: 767px){\n            display: \"none\"\n        }\n    }\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_block_comment() {
        let input = "div{/* comment */a}";
        let expected = "div {\n    /* comment */a\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_escape_in_string() {
        let input = "div{\"hello \\\"world\\\"\"}";
        let expected = "div {\n    \"hello \\\"world\\\"\"\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_attribute_with_if() {
        let input = "class:if{a}{b}else{c}";
        let expected = "class: if { a } {\n    b\n} else {\n    c\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_nested_match_in_if() {
        let input = "if{a}{match{b}{\"c\"=>{d}}}";
        let expected =
            "if { a } {\n    match { b } {\n        \"c\" => {\n            d\n        }\n    }\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_empty_line_preservation() {
        let input = "div{a\n\n    b}";
        let expected = "div {\n    a\n\n    b\n}";
        assert_eq!(format_macro_body(input), expected);
    }

    #[test]
    fn test_format_euv_macros_html() {
        let input = "html! {if{a}{b}}";
        let expected = "html! {\n    if { a } {\n        b\n    }\n}";
        assert_eq!(format_euv_macros(input), expected);
    }

    #[test]
    fn test_format_euv_macros_class() {
        let input = "class! {c{display:\"flex\"}}";
        let expected = "class! {\n    c {\n        display: \"flex\"\n    }\n}";
        assert_eq!(format_euv_macros(input), expected);
    }

    #[test]
    fn test_format_euv_macros_vars() {
        let input = "vars! {a:\"1\"}";
        let expected = "vars! {\n    a: \"1\"\n}";
        assert_eq!(format_euv_macros(input), expected);
    }

    #[test]
    fn test_format_euv_macros_watch() {
        let input = "watch! {s => {f}}";
        let expected = "watch! {\n    s => {\n        f\n    }\n}";
        assert_eq!(format_euv_macros(input), expected);
    }

    #[test]
    fn test_format_euv_macros_no_leading_blank_lines() {
        let input = "    html! {\n            div {\n                class: c_euv_input_wrapper()\n                label {\n                    for: id\n                    class: c_form_label()\n                    label_string\n                }\n                input {\n                    id: id\n                    name: id\n                    type: \"text\"\n                    placeholder: placeholder\n                    value: value\n                    autocomplete: autocomplete\n                    class: c_euv_input()\n                    onfocus: on_focus_scroll_into_view()\n                }\n            }\n        }";
        let expected = "    html! {\n            div {\n                class: c_euv_input_wrapper()\n                label {\n                    for: id\n                    class: c_form_label()\n                    label_string\n                }\n                input {\n                    id: id\n                    name: id\n                    type: \"text\"\n                    placeholder: placeholder\n                    value: value\n                    autocomplete: autocomplete\n                    class: c_euv_input()\n                    onfocus: on_focus_scroll_into_view()\n                }\n            }\n    }";
        assert_eq!(format_euv_macros(input), expected);
    }

    #[test]
    fn test_sibling_elements() {
        let input = "html! {\n    vconsole_fab {\n        panel_open: panel_open\n        console_signal: console_signal\n    }\n    vconsole_drawer {\n        console_signal: console_signal\n        panel_open: panel_open\n    }\n}";
        let expected = "html! {\n    vconsole_fab {\n        panel_open: panel_open\n        console_signal: console_signal\n    }\n    vconsole_drawer {\n        console_signal: console_signal\n        panel_open: panel_open\n    }\n}";
        assert_eq!(format_euv_macros(input), expected);
    }

    #[test]
    fn test_sibling_elements_with_indent() {
        let input = "    html! {\n            vconsole_fab {\n                panel_open: panel_open\n                console_signal: console_signal\n            }\n            vconsole_drawer {\n                console_signal: console_signal\n                panel_open: panel_open\n            }\n        }";
        let expected = "    html! {\n            vconsole_fab {\n                panel_open: panel_open\n                console_signal: console_signal\n            }\n            vconsole_drawer {\n                console_signal: console_signal\n                panel_open: panel_open\n            }\n    }";
        assert_eq!(format_euv_macros(input), expected);
    }

    #[test]
    fn test_sibling_elements_compressed() {
        let input = "html!{vconsole_fab{panel_open:panel_open console_signal:console_signal}vconsole_drawer{console_signal:console_signal panel_open:panel_open}}";
        let expected = "html! {\n    vconsole_fab {\n        panel_open: panel_open\n        console_signal: console_signal\n    }\n    vconsole_drawer {\n        console_signal: console_signal\n        panel_open: panel_open\n    }\n}";
        assert_eq!(format_euv_macros(input), expected);
    }
}
