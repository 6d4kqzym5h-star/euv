use super::*;

// =====================================================================
// vdom::attribute::const
// =====================================================================

#[test]
fn const_euv_css_injected_id_value() {
    assert_eq!(EUV_CSS_INJECTED_ID, "euv-css-injected");
}

#[test]
fn const_style_tag_value() {
    assert_eq!(STYLE_TAG, "style");
}

#[test]
fn const_callback_event_name_value() {
    assert_eq!(CALLBACK_EVENT_NAME, "callback");
}

#[test]
fn const_css_rule_open_value() {
    assert_eq!(CSS_RULE_OPEN, " { ");
}

#[test]
fn const_css_media_prefix_value() {
    assert_eq!(CSS_MEDIA_PREFIX, "@media ");
}

#[test]
fn const_char_space_value() {
    assert_eq!(CHAR_SPACE, ' ');
}

#[test]
fn const_css_prop_separator_value() {
    assert_eq!(CSS_PROP_SEPARATOR, ": ");
}

#[test]
fn const_char_css_decl_terminator_value() {
    assert_eq!(CHAR_CSS_DECL_TERMINATOR, ';');
}

#[test]
fn const_char_css_rule_close_value() {
    assert_eq!(CHAR_CSS_RULE_CLOSE, '}');
}

#[test]
fn const_char_css_class_prefix_value() {
    assert_eq!(CHAR_CSS_CLASS_PREFIX, '.');
}

#[test]
fn const_css_rule_open_format_value() {
    assert_eq!(CSS_RULE_OPEN_FORMAT, " { ");
}

#[test]
fn const_css_rule_close_format_value() {
    assert_eq!(CSS_RULE_CLOSE_FORMAT, " }");
}

#[test]
fn const_char_css_rule_separator_value() {
    assert_eq!(CHAR_CSS_RULE_SEPARATOR, '\n');
}

#[test]
fn const_char_signal_addrs_separator_value() {
    assert_eq!(CHAR_SIGNAL_ADDRS_SEPARATOR, ',');
}

#[test]
fn const_char_css_escape_value() {
    assert_eq!(CHAR_CSS_ESCAPE, '\\');
}

#[test]
fn const_char_hyphen_value() {
    assert_eq!(CHAR_HYPHEN, '-');
}

#[test]
fn const_char_underscore_value() {
    assert_eq!(CHAR_UNDERSCORE, '_');
}

#[test]
fn const_class_param_hash_fnv_offset_value() {
    // FNV-1a offset basis is the standard
    // 14695981039346656037 (0xcbf29ce484222325).
    assert_eq!(CLASS_PARAM_HASH_FNV_OFFSET, 14695981039346656037);
}

#[test]
fn const_class_param_hash_fnv_prime_value() {
    // FNV-1a prime is the standard
    // 1099511628211 (0x100000001b3).
    assert_eq!(CLASS_PARAM_HASH_FNV_PRIME, 1099511628211);
}

#[test]
fn const_class_param_hash_fnv_offset_is_correct_hex() {
    // Verify the well-known FNV-1a offset basis
    // hex encoding (0xcbf29ce484222325).
    assert_eq!(
        format!("{:016x}", CLASS_PARAM_HASH_FNV_OFFSET),
        "cbf29ce484222325"
    );
}

#[test]
fn const_class_param_hash_fnv_prime_is_correct_hex() {
    // Verify the well-known FNV-1a prime hex
    // encoding (0x100000001b3).
    assert_eq!(
        format!("{:014x}", CLASS_PARAM_HASH_FNV_PRIME),
        "000100000001b3"
    );
}

// =====================================================================
// FNV-1a hash determinism (verifies the FNV constants
// work together to produce stable hash suffixes)
// =====================================================================

#[test]
fn fnv_constants_produce_stable_hash_for_empty() {
    let mut hash: u64 = CLASS_PARAM_HASH_FNV_OFFSET;
    for byte in b"" {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(CLASS_PARAM_HASH_FNV_PRIME);
    }
    // Hashing empty bytes leaves the offset basis
    // unchanged.
    assert_eq!(hash, CLASS_PARAM_HASH_FNV_OFFSET);
}

#[test]
fn fnv_constants_produce_stable_hash_for_short_input() {
    let mut hash: u64 = CLASS_PARAM_HASH_FNV_OFFSET;
    for byte in b"abc" {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(CLASS_PARAM_HASH_FNV_PRIME);
    }
    // FNV-1a hash of "abc" is the well-known
    // 0xe71fa2190541574b.
    assert_eq!(format!("{:016x}", hash), "e71fa2190541574b");
}

#[test]
fn fnv_constants_produce_deterministic_results() {
    let compute = |input: &[u8]| -> u64 {
        let mut hash: u64 = CLASS_PARAM_HASH_FNV_OFFSET;
        for byte in input {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(CLASS_PARAM_HASH_FNV_PRIME);
        }
        hash
    };
    // Same input must always produce the same hash.
    let a: u64 = compute(b"class-name-1");
    let b: u64 = compute(b"class-name-1");
    assert_eq!(a, b);
}

#[test]
fn fnv_constants_produce_distinct_results_for_distinct_inputs() {
    let compute = |input: &[u8]| -> u64 {
        let mut hash: u64 = CLASS_PARAM_HASH_FNV_OFFSET;
        for byte in input {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(CLASS_PARAM_HASH_FNV_PRIME);
        }
        hash
    };
    // Distinct inputs must produce distinct hashes
    // (with overwhelming probability).
    let a: u64 = compute(b"a");
    let b: u64 = compute(b"b");
    assert_ne!(a, b);
}

// =====================================================================
// Char constants cross-validation (format / character usage)
// =====================================================================

#[test]
fn char_constants_are_ascii() {
    // All single-char constants are ASCII.
    assert!(CHAR_SPACE.is_ascii());
    assert!(CHAR_CSS_DECL_TERMINATOR.is_ascii());
    assert!(CHAR_CSS_RULE_CLOSE.is_ascii());
    assert!(CHAR_CSS_CLASS_PREFIX.is_ascii());
    assert!(CHAR_CSS_RULE_SEPARATOR.is_ascii());
    assert!(CHAR_SIGNAL_ADDRS_SEPARATOR.is_ascii());
    assert!(CHAR_CSS_ESCAPE.is_ascii());
    assert!(CHAR_HYPHEN.is_ascii());
    assert!(CHAR_UNDERSCORE.is_ascii());
}

#[test]
fn string_constants_are_not_empty() {
    assert!(!EUV_CSS_INJECTED_ID.is_empty());
    assert!(!STYLE_TAG.is_empty());
    assert!(!CALLBACK_EVENT_NAME.is_empty());
    assert!(!CSS_RULE_OPEN.is_empty());
    assert!(!CSS_MEDIA_PREFIX.is_empty());
    assert!(!CSS_PROP_SEPARATOR.is_empty());
    assert!(!CSS_RULE_OPEN_FORMAT.is_empty());
    assert!(!CSS_RULE_CLOSE_FORMAT.is_empty());
}

#[test]
fn css_rule_open_matches_open_format() {
    // CSS_RULE_OPEN and CSS_RULE_OPEN_FORMAT share
    // the same value (used in different code paths).
    assert_eq!(CSS_RULE_OPEN, CSS_RULE_OPEN_FORMAT);
}

#[test]
fn css_rule_close_format_starts_with_space() {
    // The close format is " }" — space then brace.
    assert!(CSS_RULE_CLOSE_FORMAT.starts_with(' '));
    assert!(CSS_RULE_CLOSE_FORMAT.ends_with('}'));
}

#[test]
fn css_media_prefix_includes_at_sign() {
    assert!(CSS_MEDIA_PREFIX.starts_with('@'));
}

#[test]
fn char_css_escape_is_backslash() {
    // The escape char is a single backslash.
    assert_eq!(CHAR_CSS_ESCAPE as u32, 0x5c);
}

#[test]
fn char_signal_addrs_separator_is_comma() {
    assert_eq!(CHAR_SIGNAL_ADDRS_SEPARATOR as u32, 0x2c);
}

#[test]
fn css_prop_separator_format() {
    // The CSS property separator is colon then space.
    assert!(CSS_PROP_SEPARATOR.starts_with(':'));
    assert!(CSS_PROP_SEPARATOR.ends_with(' '));
}
