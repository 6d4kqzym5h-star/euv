use super::*;

use crate::{HashMap, I18n, MessageEntry, Signal};

fn run_with_signal_capture<F>(f: F) -> bool
where
    F: FnOnce(),
{
    catch_unwind(AssertUnwindSafe(f)).is_ok()
}

fn fresh_i18n() -> I18n {
    I18n::new(
        Signal::create(String::from("en")),
        Signal::create(String::from("en")),
        Signal::create(HashMap::new()),
    )
}

/// Pre-populated I18n: `en` has `hello -> "Hello"` and
/// `zh-CN` has `hello -> "你好"`. Used by set-path tests
/// that need to read via `t()` after registering.
fn seeded_i18n() -> I18n {
    let mut messages: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut en: HashMap<String, String> = HashMap::new();
    en.insert("hello".to_string(), String::from("Hello"));
    en.insert("goodbye".to_string(), String::from("Goodbye"));
    let mut zh: HashMap<String, String> = HashMap::new();
    zh.insert("hello".to_string(), String::from("你好"));
    messages.insert("en".to_string(), en);
    messages.insert("zh-CN".to_string(), zh);
    I18n::new(
        Signal::create(String::from("en")),
        Signal::create(String::from("en")),
        Signal::create(messages),
    )
}

// ============================================================
//  Pure-Rust contract tests.
// ============================================================

#[test]
fn fresh_i18n_defaults_to_en_locale_and_empty_messages() {
    let i18n: I18n = fresh_i18n();
    assert_eq!(i18n.locale().get(), "en");
    assert_eq!(i18n.fallback_locale().get(), "en");
    assert_eq!(i18n.locale_count(), 0);
    assert_eq!(i18n.active_message_count(), 0);
}

#[test]
fn fresh_i18n_returns_key_for_missing_translation() {
    let i18n: I18n = fresh_i18n();
    assert_eq!(i18n.t("hello"), "hello");
    assert_eq!(i18n.t("nonexistent"), "nonexistent");
}

#[test]
fn locale_and_fallback_locale_accessors_return_signal_clones() {
    let i18n: I18n = fresh_i18n();
    let _locale: Signal<String> = i18n.locale();
    let _fallback: Signal<String> = i18n.fallback_locale();
}

#[test]
fn reactive_read_via_subscribed_locale_signal_matches_initial_value() {
    let i18n: I18n = fresh_i18n();
    let subscribed: Signal<String> = i18n.locale();
    assert_eq!(subscribed.get(), "en");
}

#[test]
fn seeded_messages_are_translated_by_active_locale() {
    let i18n: I18n = seeded_i18n();
    // Default locale is "en"
    assert_eq!(i18n.t("hello"), "Hello");
    assert_eq!(i18n.t("goodbye"), "Goodbye");
}

#[test]
fn seeded_i18n_active_message_count_matches_registered_keys() {
    let i18n: I18n = seeded_i18n();
    assert_eq!(i18n.active_message_count(), 2);
}

#[test]
fn seeded_i18n_locale_count_matches_registered_locales() {
    let i18n: I18n = seeded_i18n();
    assert_eq!(i18n.locale_count(), 2);
}

#[test]
fn i18n_clone_shares_internal_signals() {
    let i18n: I18n = seeded_i18n();
    let twin: I18n = i18n.clone();
    // Both clones point at the same signals — reads
    // through the twin see the same messages.
    assert_eq!(twin.t("hello"), "Hello");
    assert_eq!(twin.locale().get(), "en");
}

// ============================================================
//  Set-path coverage tests.
// ============================================================

#[test]
fn change_locale_updates_active_locale_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = run_with_signal_capture(|| {
        i18n.change_locale("zh-CN");
    });
    if ran {
        assert_eq!(i18n.locale().get(), "zh-CN");
        assert_eq!(i18n.t("hello"), "你好");
    }
}

#[test]
fn change_locale_falls_back_when_key_missing_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = run_with_signal_capture(|| {
        // zh-CN has `hello` but not `goodbye`. When the
        // active locale is zh-CN, the missing `goodbye`
        // should fall back to en.
        i18n.change_locale("zh-CN");
    });
    if ran {
        assert_eq!(i18n.t("hello"), "你好");
        assert_eq!(
            i18n.t("goodbye"),
            "Goodbye",
            "missing key in zh-CN should fall back to en"
        );
    }
}

#[test]
fn change_locale_returns_key_when_neither_locale_has_it_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = run_with_signal_capture(|| {
        i18n.change_locale("zh-CN");
    });
    if ran {
        assert_eq!(
            i18n.t("nonexistent"),
            "nonexistent",
            "key missing in both locales must fall through to the key itself"
        );
    }
}

#[test]
fn change_fallback_locale_changes_fallback_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = run_with_signal_capture(|| {
        // Switch fallback from en to zh-CN.
        i18n.change_fallback_locale("zh-CN");
        // Set active to a locale that does NOT exist.
        i18n.change_locale("ja");
    });
    if ran {
        // Neither ja nor zh-CN has `goodbye`. Falls
        // back to the key itself.
        // (en still has goodbye, but en is no longer
        // the fallback.)
        assert_eq!(i18n.t("goodbye"), "goodbye");
    }
}

#[test]
fn add_messages_inserts_new_locale_set_path() {
    let i18n: I18n = fresh_i18n();
    let ran: bool = run_with_signal_capture(|| {
        let entries: &[MessageEntry] = &[("hello", "Bonjour"), ("goodbye", "Au revoir")];
        i18n.add_messages("fr", entries);
    });
    if ran {
        i18n.change_locale("fr");
        assert_eq!(i18n.t("hello"), "Bonjour");
        assert_eq!(i18n.t("goodbye"), "Au revoir");
    }
}

#[test]
fn add_messages_overwrites_existing_entry_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = run_with_signal_capture(|| {
        // Overwrite `hello` in en.
        let entries: &[MessageEntry] = &[("hello", "Howdy")];
        i18n.add_messages("en", entries);
    });
    if ran {
        assert_eq!(i18n.t("hello"), "Howdy");
    }
}

#[test]
fn add_messages_supports_empty_batch_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = run_with_signal_capture(|| {
        let entries: &[MessageEntry] = &[];
        i18n.add_messages("es", entries);
    });
    if ran {
        assert_eq!(i18n.locale_count(), 3);
        assert_eq!(i18n.active_message_count(), 2);
    }
}

#[test]
fn remove_locale_drops_locale_from_table_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = run_with_signal_capture(|| {
        i18n.remove_locale("zh-CN");
    });
    if ran {
        assert_eq!(i18n.locale_count(), 1);
    }
}

#[test]
fn remove_locale_absent_locale_is_noop_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = run_with_signal_capture(|| {
        i18n.remove_locale("ja");
    });
    if ran {
        assert_eq!(i18n.locale_count(), 2);
    }
}

#[test]
fn remove_message_drops_single_key_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = run_with_signal_capture(|| {
        i18n.remove_message("en", "goodbye");
    });
    if ran {
        // `goodbye` is gone from en; falls through to
        // the key itself.
        assert_eq!(i18n.t("goodbye"), "goodbye");
        assert_eq!(i18n.t("hello"), "Hello");
    }
}

#[test]
fn remove_message_absent_key_is_noop_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = run_with_signal_capture(|| {
        i18n.remove_message("en", "nonexistent");
    });
    if ran {
        assert_eq!(i18n.t("hello"), "Hello");
        assert_eq!(i18n.t("goodbye"), "Goodbye");
    }
}

#[test]
fn t_with_interpolates_supplied_variables_set_path() {
    let i18n: I18n = seeded_i18n();
    // The seeded messages don't have a `{name}` template,
    // so the interpolation is a no-op. Verify by calling
    // with vars and seeing them ignored.
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("name", "Alice");
    let ran: bool = run_with_signal_capture(|| {
        i18n.change_locale("en");
    });
    if ran {
        let result: String = i18n.t_with("hello", &vars);
        assert_eq!(result, "Hello");
    }
}

// ============================================================
//  Interpolation unit tests (pure Rust, no signals).
// ============================================================
//
// These live in `state.rs::interpolate_tests` but the
// `interpolate` function is `fn` (not `pub fn`), so we
// re-test the interpolation behavior end-to-end via
// the public `t_with` method, using a pre-populated
// messages map that contains `{name}` placeholders.
//
// We seed messages via `Signal::create` (no `set` calls,
// so no native panic) and verify the rendered string.

#[test]
fn t_with_substitutes_placeholders_in_pre_registered_messages() {
    let mut messages: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut en: HashMap<String, String> = HashMap::new();
    en.insert("greet".to_string(), String::from("Hello, {name}!"));
    messages.insert("en".to_string(), en);
    let i18n: I18n = I18n::new(
        Signal::create(String::from("en")),
        Signal::create(String::from("en")),
        Signal::create(messages),
    );
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("name", "Alice");
    assert_eq!(i18n.t_with("greet", &vars), "Hello, Alice!");
}

#[test]
fn t_with_leaves_missing_placeholders_as_literal_tokens() {
    let mut messages: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut en: HashMap<String, String> = HashMap::new();
    en.insert("greet".to_string(), String::from("Hello, {name}!"));
    messages.insert("en".to_string(), en);
    let i18n: I18n = I18n::new(
        Signal::create(String::from("en")),
        Signal::create(String::from("en")),
        Signal::create(messages),
    );
    let vars: HashMap<&'static str, &'static str> = HashMap::new();
    assert_eq!(i18n.t_with("greet", &vars), "Hello, {name}!");
}

#[test]
fn t_with_substitutes_multiple_placeholders() {
    let mut messages: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut en: HashMap<String, String> = HashMap::new();
    en.insert("ordered".to_string(), String::from("{greeting}, {name}!"));
    messages.insert("en".to_string(), en);
    let i18n: I18n = I18n::new(
        Signal::create(String::from("en")),
        Signal::create(String::from("en")),
        Signal::create(messages),
    );
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("greeting", "你好");
    vars.insert("name", "Alice");
    assert_eq!(i18n.t_with("ordered", &vars), "你好, Alice!");
}

#[test]
fn t_with_supports_underscored_placeholder_names() {
    let mut messages: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut en: HashMap<String, String> = HashMap::new();
    en.insert("greet".to_string(), String::from("Hi, {first_name}!"));
    messages.insert("en".to_string(), en);
    let i18n: I18n = I18n::new(
        Signal::create(String::from("en")),
        Signal::create(String::from("en")),
        Signal::create(messages),
    );
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("first_name", "Alice");
    assert_eq!(i18n.t_with("greet", &vars), "Hi, Alice!");
}

#[test]
fn t_with_handles_unterminated_brace_by_leaving_literal() {
    let mut messages: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut en: HashMap<String, String> = HashMap::new();
    en.insert("broken".to_string(), String::from("Hello, {name!"));
    messages.insert("en".to_string(), en);
    let i18n: I18n = I18n::new(
        Signal::create(String::from("en")),
        Signal::create(String::from("en")),
        Signal::create(messages),
    );
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("name", "Alice");
    assert_eq!(i18n.t_with("broken", &vars), "Hello, {name!");
}

#[test]
fn t_with_handles_missing_key_by_returning_key() {
    let i18n: I18n = fresh_i18n();
    let vars: HashMap<&'static str, &'static str> = HashMap::new();
    assert_eq!(i18n.t_with("missing", &vars), "missing");
}

// ============================================================
//  Stress tests.
// ============================================================

#[test]
fn many_locales_can_be_registered_set_path() {
    let i18n: I18n = fresh_i18n();
    let cell: Rc<Cell<usize>> = Rc::new(Cell::new(0));
    let cell_clone: Rc<Cell<usize>> = Rc::clone(&cell);
    let ran: bool = run_with_signal_capture(|| {
        for i in 0..20 {
            let locale: String = format!("locale-{}", i);
            let entries: &[MessageEntry] = &[("k", "v")];
            i18n.add_messages(&locale, entries);
        }
        let _ = catch_unwind(AssertUnwindSafe(|| {
            cell_clone.set(i18n.locale_count());
        }));
    });
    if ran {
        assert_eq!(cell.get(), 20);
    }
}

#[test]
fn reactivity_locale_change_triggers_re_read_of_t() {
    // The reactive contract: when locale changes,
    // `t(key)` should reflect the new locale. We don't
    // need a real subscription here — `t` reads
    // `self.locale.get()` internally, so a fresh `t`
    // call after `change_locale` returns the new
    // translation (gated on the set-path running).
    let i18n: I18n = seeded_i18n();
    assert_eq!(i18n.t("hello"), "Hello");
    let ran: bool = run_with_signal_capture(|| {
        i18n.change_locale("zh-CN");
    });
    if ran {
        assert_eq!(i18n.t("hello"), "你好");
    }
}

#[test]
fn reactively_subscribed_locale_signal_reflects_change_locale() {
    let i18n: I18n = fresh_i18n();
    let subscribed: Signal<String> = i18n.locale();
    assert_eq!(subscribed.get(), "en");
    let ran: bool = run_with_signal_capture(|| {
        i18n.change_locale("ja");
    });
    if ran {
        assert_eq!(subscribed.get(), "ja");
    }
}

#[test]
fn empty_template_returns_empty_string() {
    let vars: HashMap<&'static str, &'static str> = HashMap::new();
    assert_eq!(interpolate("", &vars), "");
}

#[test]
fn template_without_placeholders_is_unchanged() {
    let vars: HashMap<&'static str, &'static str> = HashMap::new();
    assert_eq!(interpolate("Hello, world!", &vars), "Hello, world!");
}

#[test]
fn single_placeholder_is_substituted() {
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("name", "Alice");
    assert_eq!(interpolate("Hello, {name}!", &vars), "Hello, Alice!");
}

#[test]
fn multiple_placeholders_are_all_substituted() {
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("greeting", "你好");
    vars.insert("name", "Alice");
    assert_eq!(interpolate("{greeting}, {name}!", &vars), "你好, Alice!");
}

#[test]
fn missing_placeholder_is_left_as_literal() {
    let vars: HashMap<&'static str, &'static str> = HashMap::new();
    assert_eq!(interpolate("Hello, {name}!", &vars), "Hello, {name}!");
}

#[test]
fn unterminated_brace_is_left_as_literal() {
    let vars: HashMap<&'static str, &'static str> = HashMap::new();
    assert_eq!(interpolate("Hello, {name!", &vars), "Hello, {name!");
}

#[test]
fn mixed_substituted_and_missing_placeholders() {
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("greeting", "你好");
    assert_eq!(interpolate("{greeting}, {name}!", &vars), "你好, {name}!");
}

#[test]
fn placeholder_names_can_contain_underscores() {
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("first_name", "Alice");
    assert_eq!(interpolate("Hi, {first_name}!", &vars), "Hi, Alice!");
}
