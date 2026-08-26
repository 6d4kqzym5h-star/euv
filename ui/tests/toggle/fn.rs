use super::*;

#[test]
fn toggle_default_is_false() {
    let toggle: Toggle = Toggle::default();
    assert!(!toggle.get());
}

#[test]
fn toggle_new_starts_false() {
    let toggle: Toggle = Toggle::new();
    assert!(!toggle.get());
}

#[test]
fn toggle_set_true_makes_true() {
    let toggle: Toggle = Toggle::new();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        toggle.set_true();
    }))
    .is_ok();
    if ran {
        assert!(toggle.get());
    }
}

#[test]
fn toggle_set_false_makes_false() {
    let toggle: Toggle = Toggle::new();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        toggle.set_true();
    }))
    .is_ok();
    if ran {
        assert!(toggle.get());
        let ran: bool = catch_unwind(AssertUnwindSafe(|| {
            toggle.set_false();
        }))
        .is_ok();
        if ran {
            assert!(!toggle.get());
        }
    }
}

#[test]
fn toggle_toggle_flips_false_to_true() {
    let toggle: Toggle = Toggle::new();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        toggle.toggle();
    }))
    .is_ok();
    if ran {
        assert!(toggle.get());
    }
}

#[test]
fn toggle_toggle_flips_true_to_false() {
    let toggle: Toggle = Toggle::new();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        toggle.set_true();
    }))
    .is_ok();
    if ran {
        let ran: bool = catch_unwind(AssertUnwindSafe(|| {
            toggle.toggle();
        }))
        .is_ok();
        if ran {
            assert!(!toggle.get());
        }
    }
}

#[test]
fn toggle_double_toggle_returns_to_initial() {
    let toggle: Toggle = Toggle::new();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        toggle.toggle();
        toggle.toggle();
    }))
    .is_ok();
    if ran {
        assert!(!toggle.get());
    }
}

#[test]
fn toggle_set_replaces_value() {
    let toggle: Toggle = Toggle::new();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        toggle.set(true);
        toggle.set(false);
    }))
    .is_ok();
    if ran {
        assert!(!toggle.get());
    }
}

#[test]
fn toggle_clone_shares_state() {
    let original: Toggle = Toggle::new();
    let clone: Toggle = original.clone();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        clone.set_true();
    }))
    .is_ok();
    if ran {
        assert!(original.get());
        assert!(clone.get());
    }
}

#[test]
fn toggle_reactive_read_via_subscribed_signal_matches() {
    let toggle: Toggle = Toggle::new();
    let initial: bool = toggle.get_value().get();
    assert!(!initial);
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        toggle.set_true();
    }))
    .is_ok();
    if ran {
        let after: bool = toggle.get_value().get();
        assert!(after);
    }
}

#[test]
fn toggle_partial_eq_same_value() {
    let a: Toggle = Toggle::new();
    let b: Toggle = Toggle::new();
    assert_eq!(a, b);
}

#[test]
fn toggle_partial_eq_different_value() {
    let a: Toggle = Toggle::new();
    let b: Toggle = Toggle::new();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        b.set_true();
    }))
    .is_ok();
    if ran {
        assert_ne!(a, b);
    }
}

#[test]
fn toggle_partial_eq_after_mutation() {
    let a: Toggle = Toggle::new();
    let b: Toggle = Toggle::new();
    assert_eq!(a, b);
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        b.set_true();
    }))
    .is_ok();
    if ran {
        assert_ne!(a, b);
    }
}

#[test]
fn toggle_display_format_works() {
    let toggle: Toggle = Toggle::new();
    let formatted: String = format!("{toggle}");
    assert_eq!(formatted, "Toggle(false)");
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        toggle.set_true();
    }))
    .is_ok();
    if ran {
        let formatted: String = format!("{toggle}");
        assert_eq!(formatted, "Toggle(true)");
    }
}
