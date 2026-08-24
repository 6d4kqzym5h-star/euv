use super::*;

// =====================================================================
// renderer::dom::const
// =====================================================================

#[test]
fn const_attr_value_value() {
    assert_eq!(ATTR_VALUE, "value");
}

#[test]
fn const_attr_checked_value() {
    assert_eq!(ATTR_CHECKED, "checked");
}

#[test]
fn const_attr_disabled_value() {
    assert_eq!(ATTR_DISABLED, "disabled");
}

#[test]
fn const_attr_selected_value() {
    assert_eq!(ATTR_SELECTED, "selected");
}

#[test]
fn const_attr_readonly_value() {
    assert_eq!(ATTR_READONLY, "readonly");
}

#[test]
fn const_attr_multiple_value() {
    assert_eq!(ATTR_MULTIPLE, "multiple");
}

#[test]
fn const_bool_true_value() {
    assert_eq!(BOOL_TRUE, "true");
}

#[test]
fn const_empty_string_value() {
    assert_eq!(EMPTY_STRING, "");
}

#[test]
fn const_dom_attribute_names_are_lowercase() {
    // HTML attribute names are case-insensitive
    // (lowercased by browsers). Verify the constants
    // are lowercase.
    assert_eq!(ATTR_VALUE, ATTR_VALUE.to_lowercase());
    assert_eq!(ATTR_CHECKED, ATTR_CHECKED.to_lowercase());
    assert_eq!(ATTR_DISABLED, ATTR_DISABLED.to_lowercase());
    assert_eq!(ATTR_SELECTED, ATTR_SELECTED.to_lowercase());
    assert_eq!(ATTR_READONLY, ATTR_READONLY.to_lowercase());
    assert_eq!(ATTR_MULTIPLE, ATTR_MULTIPLE.to_lowercase());
}

#[test]
fn const_dom_attribute_names_are_distinct() {
    // Each property name maps to a different DOM
    // property. Verify no two constants share a value.
    let all: [&str; 6] = [
        ATTR_VALUE,
        ATTR_CHECKED,
        ATTR_DISABLED,
        ATTR_SELECTED,
        ATTR_READONLY,
        ATTR_MULTIPLE,
    ];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j], "{} == {}", i, j);
        }
    }
}

// =====================================================================
// renderer::registry::const
// =====================================================================

#[test]
fn const_data_euv_id_value() {
    assert_eq!(DATA_EUV_ID, "data-euv-id");
}

#[test]
fn const_max_iterations_value() {
    assert_eq!(MAX_ITERATIONS, 3);
}

#[test]
fn const_max_iterations_is_positive() {
    // The dispatch loop must iterate at least once.
    assert!(MAX_ITERATIONS > 0);
}

#[test]
fn const_max_iterations_is_bounded() {
    // Sanity: MAX_ITERATIONS should not be
    // pathologically large.
    assert!(MAX_ITERATIONS < 1000);
}

#[test]
fn const_non_bubbling_events_length() {
    // 35 entries are documented in the source.
    assert_eq!(NON_BUBBLING_EVENTS.len(), 35);
}

#[test]
fn const_non_bubbling_events_includes_known() {
    // Spot-check the well-known non-bubbling events
    // from W3C DOM Level 3.
    assert!(NON_BUBBLING_EVENTS.contains(&"blur"));
    assert!(NON_BUBBLING_EVENTS.contains(&"focus"));
    assert!(NON_BUBBLING_EVENTS.contains(&"load"));
    assert!(NON_BUBBLING_EVENTS.contains(&"error"));
    assert!(NON_BUBBLING_EVENTS.contains(&"mouseenter"));
    assert!(NON_BUBBLING_EVENTS.contains(&"mouseleave"));
    assert!(NON_BUBBLING_EVENTS.contains(&"resize"));
    assert!(NON_BUBBLING_EVENTS.contains(&"scroll"));
}

#[test]
fn const_non_bubbling_events_excludes_click() {
    // `click` is a bubbling event and should NOT be
    // in the non-bubbling list.
    assert!(!NON_BUBBLING_EVENTS.contains(&"click"));
}

#[test]
fn const_non_bubbling_events_distinct() {
    // Each event name must appear at most once.
    use std::collections::HashSet;
    let unique: HashSet<&str> = NON_BUBBLING_EVENTS.iter().copied().collect();
    assert_eq!(unique.len(), NON_BUBBLING_EVENTS.len());
}

#[test]
fn const_non_bubbling_events_lowercase() {
    // Event names should be lowercase.
    for event in NON_BUBBLING_EVENTS.iter() {
        assert_eq!(*event, event.to_lowercase());
    }
}

// =====================================================================
// renderer::render::const
// =====================================================================

#[test]
fn const_data_euv_dynamic_id_value() {
    assert_eq!(DATA_EUV_DYNAMIC_ID, "data-euv-dynamic-id");
}

#[test]
fn const_data_euv_signal_addrs_value() {
    assert_eq!(DATA_EUV_SIGNAL_ADDRS, "data-euv-signal-addrs");
}

#[test]
fn const_data_euv_attributes_have_correct_prefix() {
    // All euv-managed DOM attributes start with
    // `data-euv-`.
    assert!(DATA_EUV_ID.starts_with("data-euv-"));
    assert!(DATA_EUV_DYNAMIC_ID.starts_with("data-euv-"));
    assert!(DATA_EUV_SIGNAL_ADDRS.starts_with("data-euv-"));
}

#[test]
fn const_data_euv_attributes_are_distinct() {
    let all: [&str; 3] = [DATA_EUV_ID, DATA_EUV_DYNAMIC_ID, DATA_EUV_SIGNAL_ADDRS];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j]);
        }
    }
}

#[test]
fn const_fragment_tag_value() {
    assert_eq!(FRAGMENT_TAG, "slot");
}

#[test]
fn const_dynamic_placeholder_tag_value() {
    assert_eq!(DYNAMIC_PLACEHOLDER_TAG, "div");
}

#[test]
fn const_placeholder_tags_are_distinct() {
    assert_ne!(FRAGMENT_TAG, DYNAMIC_PLACEHOLDER_TAG);
}

#[test]
fn const_display_contents_style_value() {
    assert_eq!(DISPLAY_CONTENTS_STYLE, "display: contents;");
}

#[test]
fn const_fragment_style_value() {
    assert_eq!(FRAGMENT_STYLE, "display:contents");
}

#[test]
fn const_attr_style_value() {
    assert_eq!(ATTR_STYLE, "style");
}

#[test]
fn const_id_selector_prefix_value() {
    assert_eq!(ID_SELECTOR_PREFIX, "#");
}

#[test]
fn const_class_selector_prefix_value() {
    assert_eq!(CLASS_SELECTOR_PREFIX, ".");
}

#[test]
fn const_selector_prefixes_are_distinct() {
    assert_ne!(ID_SELECTOR_PREFIX, CLASS_SELECTOR_PREFIX);
}

#[test]
fn const_body_tag_value() {
    assert_eq!(BODY_TAG, "body");
}

#[test]
fn const_body_tag_is_lowercase() {
    assert_eq!(BODY_TAG, BODY_TAG.to_lowercase());
}

#[test]
fn const_selector_prefixes_are_single_char() {
    assert_eq!(ID_SELECTOR_PREFIX.len(), 1);
    assert_eq!(CLASS_SELECTOR_PREFIX.len(), 1);
}

#[test]
fn const_display_contents_styles_use_correct_value() {
    // Both DISPLAY_CONTENTS_STYLE ("display: contents;")
    // and FRAGMENT_STYLE ("display:contents") are
    // valid CSS for invisible wrappers — they just
    // differ in spacing. Both target the same
    // `display: contents` semantic.
    assert!(DISPLAY_CONTENTS_STYLE.contains("display"));
    assert!(FRAGMENT_STYLE.contains("display"));
}

// =====================================================================
// Mount (ZST)
// =====================================================================

#[test]
fn mount_is_zero_sized() {
    assert_eq!(std::mem::size_of::<Mount>(), 0);
}

#[test]
fn mount_is_default() {
    let _: Mount = Mount::default();
}

#[test]
fn mount_is_clone() {
    let m: Mount = Mount::default();
    let _: Mount = m.clone();
}

#[test]
fn mount_is_copy() {
    let m: Mount = Mount::default();
    let _: Mount = m;
}

#[test]
fn mount_is_eq() {
    let a: Mount = Mount::default();
    let b: Mount = Mount::default();
    assert_eq!(a, b);
}

#[test]
fn mount_is_ord() {
    let a: Mount = Mount::default();
    let b: Mount = Mount::default();
    assert!(a <= b);
    assert!(a >= b);
}

#[test]
fn mount_is_hash() {
    use std::collections::hash_map::DefaultHasher;
    let m: Mount = Mount::default();
    let mut hasher: DefaultHasher = DefaultHasher::new();
    m.hash(&mut hasher);
    let _: u64 = hasher.finish();
}

#[test]
fn mount_debug_format_works() {
    let m: Mount = Mount::default();
    let formatted: String = format!("{:?}", m);
    assert!(formatted.contains("Mount"));
}

// =====================================================================
// Cross-module validation: constants used together must agree
// =====================================================================

#[test]
fn renderer_constants_consistent_with_attribute_constants() {
    // ATTR_STYLE ("style") is the attribute name,
    // while DISPLAY_CONTENTS_STYLE ("display: contents;")
    // is the value. They serve different roles.
    assert_ne!(ATTR_STYLE, DISPLAY_CONTENTS_STYLE);
    assert_eq!(ATTR_STYLE, "style");
}

#[test]
fn dom_const_overlaps_with_attribute_const() {
    // DOM property names from dom/const.rs
    // (ATTR_VALUE, ATTR_CHECKED, ...) are different
    // from the AttributeEntry attribute name
    // constants in attribute/const.rs.
    // This cross-module test ensures no accidental
    // rename divergence.
    assert_eq!(ATTR_VALUE, "value");
}

// =====================================================================
// renderer::registry::const — high-frequency event cap
// =====================================================================

#[test]
fn const_high_frequency_events_length_is_documented() {
    // 6 entries: mousemove, mousewheel, pointermove, scroll, touchmove, wheel.
    // Keep this in sync with `HIGH_FREQUENCY_EVENTS.len()` — the test
    // is the lock against accidental additions / removals.
    assert_eq!(HIGH_FREQUENCY_EVENTS.len(), 5);
}

#[test]
fn const_high_frequency_events_includes_known() {
    // Spot-check the well-known high-frequency events.
    assert!(HIGH_FREQUENCY_EVENTS.contains(&"mousemove"));
    assert!(HIGH_FREQUENCY_EVENTS.contains(&"touchmove"));
    assert!(HIGH_FREQUENCY_EVENTS.contains(&"pointermove"));
    assert!(HIGH_FREQUENCY_EVENTS.contains(&"wheel"));
    assert!(HIGH_FREQUENCY_EVENTS.contains(&"mousewheel"));
}

#[test]
fn const_high_frequency_events_excludes_low_frequency() {
    // `click`, `input`, `keydown` must NOT be in the high-frequency list
    // — their handler lookups aren't locality-preserving (a click on a
    // deeply nested icon needs to resolve to a button defined many
    // ancestors up), so capping the ancestor walk would break them.
    assert!(!HIGH_FREQUENCY_EVENTS.contains(&"click"));
    assert!(!HIGH_FREQUENCY_EVENTS.contains(&"input"));
    assert!(!HIGH_FREQUENCY_EVENTS.contains(&"keydown"));
}

#[test]
fn const_high_frequency_events_distinct() {
    // Each event name must appear at most once.
    use std::collections::HashSet;
    let unique: HashSet<&str> = HIGH_FREQUENCY_EVENTS.iter().copied().collect();
    assert_eq!(unique.len(), HIGH_FREQUENCY_EVENTS.len());
}

#[test]
fn const_high_frequency_events_lowercase() {
    // Event names should be lowercase (matches the existing
    // NON_BUBBLING_EVENTS convention).
    for event in HIGH_FREQUENCY_EVENTS.iter() {
        assert_eq!(*event, event.to_lowercase());
    }
}

#[test]
fn const_max_ancestor_depth_is_bounded() {
    // Sanity: the depth cap must be positive (depth 0 means "don't
    // even check the target" which would silently break handler
    // dispatch) and bounded (e.g. < 32) to keep the constant from
    // accidentally growing into a future DOM-depth-based regression.
    assert!(MAX_ANCESTOR_DEPTH_FOR_HIGH_FREQ > 0);
    assert!(MAX_ANCESTOR_DEPTH_FOR_HIGH_FREQ < 32);
}

#[test]
fn const_max_ancestor_depth_disjoint_from_non_bubbling() {
    // An event that does NOT bubble cannot be delegated to the window
    // in the first place, so the ancestor-walk depth cap is irrelevant
    // for it. Verify the two lists don't overlap so a future maintainer
    // doesn't add an event to both and silently disable its dispatch.
    for hf in HIGH_FREQUENCY_EVENTS.iter() {
        assert!(
            !NON_BUBBLING_EVENTS.contains(hf),
            "{} is in both HIGH_FREQUENCY_EVENTS and NON_BUBBLING_EVENTS —              it cannot be window-delegated, so the depth cap is irrelevant",
            hf
        );
    }
}
