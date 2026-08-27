use super::*;

/// Substitutes `{name}`-style placeholders in `template`
/// with values from `vars`.
///
/// Placeholder names may contain alphanumerics and
/// underscores. Missing placeholders are left as the
/// literal `{name}` token. No escaping is performed — the
/// literal text `{` in a translation is treated as the
/// start of a placeholder.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
/// - `&HashMap<&'static str, &'static str>` - Shared reference to a `HashMap<&'static str, &'static str>`.
///
/// # Returns
///
/// - `String` - A `String` value.
pub(crate) fn interpolate(template: &str, vars: &HashMap<&'static str, &'static str>) -> String {
    let mut output: String = String::with_capacity(template.len());
    let mut chars: Chars<'_> = template.chars();
    while let Some(c) = chars.next() {
        if c != '{' {
            output.push(c);
            continue;
        }
        // Try to read until the matching `}`. If we
        // hit EOF or another `{` before `}`, treat
        // the `{` as a literal.
        let mut name: String = String::new();
        let mut closed: bool = false;
        for inner in chars.by_ref() {
            if inner == '}' {
                closed = true;
                break;
            }
            name.push(inner);
        }
        if !closed {
            // Unterminated `{` — emit literally.
            output.push('{');
            output.push_str(&name);
            continue;
        }
        // Look up the placeholder.
        match vars.get(name.as_str()) {
            Some(value) => output.push_str(value),
            None => {
                // Leave as literal `{name}` so the
                // missing-translation bug is visible.
                output.push('{');
                output.push_str(&name);
                output.push('}');
            }
        }
    }
    output
}
