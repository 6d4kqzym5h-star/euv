/// The placeholder text for the dynamic attribute key input.
pub(crate) const DYNAMIC_KEY_PLACEHOLDER: &str = "Enter attr key (e.g. data-custom)";

/// The placeholder text for the dynamic attribute value input.
pub(crate) const DYNAMIC_VALUE_PLACEHOLDER: &str = "Enter attr value";

/// The autocomplete attribute value for the dynamic key input.
pub(crate) const ATTRS_AUTOCOMPLETE_OFF: &str = "off";

/// The HTML id for the dynamic attribute key input element.
pub(crate) const DYNAMIC_KEY_INPUT_ID: &str = "attrs-dynamic-key";

/// The HTML id for the dynamic attribute value input element.
pub(crate) const DYNAMIC_VALUE_INPUT_ID: &str = "attrs-dynamic-value";

/// The default CSS property key for the class macro dynamic key demo.
///
/// `color` (instead of `background-color`) is the default so that the demo
/// block stays readable at first paint: setting `background-color` to
/// `#000000` while the inherited text colour is also black makes the body
/// text disappear until the user manually types a contrasting value. With
/// `color` as the default, the page is immediately legible and any property
/// the user types afterwards (including `background-color`) is applied at
/// runtime to demonstrate the dynamic-class behaviour.
pub(crate) const CLASS_DYNAMIC_PROP_KEY: &str = "color";

/// The default CSS property value for the class macro dynamic key demo.
///
/// Pairs with `CLASS_DYNAMIC_PROP_KEY`: the demo opens with `color: <theme
/// foreground>`, which resolves to the same colour the rest of the page
/// uses and keeps the demo body visible.
pub(crate) const CLASS_DYNAMIC_PROP_VALUE: &str = "var(--foreground)";

/// The placeholder text for the CSS property key input.
pub(crate) const CLASS_KEY_PLACEHOLDER: &str = "Enter CSS prop key (e.g. background-color)";

/// The placeholder text for the CSS property value input.
pub(crate) const CLASS_VALUE_PLACEHOLDER: &str = "Enter CSS prop value (e.g. #000000)";

/// The HTML id for the CSS property key input element.
pub(crate) const CLASS_KEY_INPUT_ID: &str = "attrs-class-key";

/// The HTML id for the CSS property value input element.
pub(crate) const CLASS_VALUE_INPUT_ID: &str = "attrs-class-value";
