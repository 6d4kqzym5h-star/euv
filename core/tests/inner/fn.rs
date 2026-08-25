use super::*;

#[test]
fn inner_html_static_carries_string_payload() {
    let attr: AttributeValue = AttributeValue::InnerHtml(String::from("<svg/>"));
    match &attr {
        AttributeValue::InnerHtml(s) => assert_eq!(s, "<svg/>"),
        _ => panic!("expected AttributeValue::InnerHtml"),
    }
    let cloned: AttributeValue = attr.clone();
    match cloned {
        AttributeValue::InnerHtml(s) => assert_eq!(s, "<svg/>"),
        _ => panic!("cloned value lost its InnerHtml variant"),
    }
}

#[test]
fn inner_html_signal_carries_signal_payload() {
    let signal: Signal<String> = Signal::create(String::from("<b>hi</b>"));
    let attr: AttributeValue = AttributeValue::InnerHtmlSignal(signal);
    match &attr {
        AttributeValue::InnerHtmlSignal(s) => {
            assert_eq!(s.get(), "<b>hi</b>");
        }
        _ => panic!("expected AttributeValue::InnerHtmlSignal"),
    }
}

#[test]
fn debug_names_inner_html_variant() {
    let static_attr: AttributeValue = AttributeValue::InnerHtml(String::from("payload"));
    let formatted: String = format!("{static_attr:?}");
    assert!(
        formatted.contains("InnerHtml"),
        "Debug output must name the variant, got: {formatted}",
    );
    let signal: Signal<String> = Signal::create(String::from("hidden-signal-value"));
    let reactive_attr: AttributeValue = AttributeValue::InnerHtmlSignal(signal);
    let formatted: String = format!("{reactive_attr:?}");
    assert!(
        formatted.contains("InnerHtmlSignal"),
        "Debug output must name the variant, got: {formatted}",
    );
    assert!(
        !formatted.contains("hidden-signal-value"),
        "Debug output leaked the signal payload: {formatted}",
    );
}

#[test]
fn inner_html_adapter_from_str_copies_payload() {
    let adapter: InnerHtmlAdapter<&str> = InnerHtmlAdapter::new("<i>copied</i>");
    let attr: AttributeValue = adapter.into();
    match attr {
        AttributeValue::InnerHtml(s) => assert_eq!(s, "<i>copied</i>"),
        _ => panic!("InnerHtmlAdapter<&str> did not produce InnerHtml"),
    }
}

#[test]
fn inner_html_adapter_from_string_passes_through() {
    let payload: String = String::from("<b>kept</b>");
    let adapter: InnerHtmlAdapter<String> = InnerHtmlAdapter::new(payload);
    let attr: AttributeValue = adapter.into();
    match attr {
        AttributeValue::InnerHtml(s) => assert_eq!(s, "<b>kept</b>"),
        _ => panic!("InnerHtmlAdapter<String> did not produce InnerHtml"),
    }
}

#[test]
fn inner_html_adapter_from_signal_produces_signal_variant() {
    let signal: Signal<String> = Signal::create(String::from("<div/>"));
    let adapter: InnerHtmlAdapter<Signal<String>> = InnerHtmlAdapter::new(signal);
    let attr: AttributeValue = adapter.into();
    match attr {
        AttributeValue::InnerHtmlSignal(s) => {
            assert_eq!(s.get(), "<div/>");
        }
        _ => panic!("InnerHtmlAdapter<Signal<String>> did not produce InnerHtmlSignal"),
    }
}

#[test]
fn existing_variants_still_construct() {
    let _text: AttributeValue = AttributeValue::Text(String::from("ok"));
    let _signal: AttributeValue = AttributeValue::Signal(Signal::create(String::from("ok")));
    let _dynamic: AttributeValue = AttributeValue::Dynamic(String::from("ok"));
}
