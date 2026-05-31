use crate::*;

/// Represents a single attribute on a virtual DOM node.
///
/// Combines an attribute name with its corresponding value.
#[derive(Clone, CustomDebug, Data, New)]
pub struct AttributeEntry {
    /// The name of the attribute.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: String,
    /// The value of the attribute.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) value: AttributeValue,
}

/// Represents a CSS pseudo-class or pseudo-element rule attached to a class.
///
/// Each rule has a selector suffix (e.g., ":hover", "::before", ":focus")
/// and a style declaration string. When injected into the DOM, it produces
/// a rule like `.class-name:hover { background: red; }`.
#[derive(Clone, Data, Debug, Default, Eq, New, PartialEq)]
pub struct PseudoRule {
    /// The CSS pseudo selector suffix appended to the class name
    /// (e.g., ":hover", ":focus", ":active", ":disabled", "::before", "::after",
    /// ":first-child", ":last-child", ":nth-child(2n)", etc.).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    selector: String,
    /// The CSS style declarations for this pseudo rule
    /// (e.g., "background: rgba(79, 70, 229, 0.04); color: #4f46e5;").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    style: String,
}

/// Represents a CSS class with a name, its style declarations, and optional pseudo rules.
///
/// Created by the `class!` macro and used in `html!` via the `class:` attribute.
/// When the renderer encounters a `Css`, it injects the styles into the
/// DOM's `<style>` element on first use and applies the class name to the element.
#[derive(Clone, Data, Debug, Default, New)]
pub struct Css {
    /// The CSS class name used in the DOM.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    name: String,
    /// The CSS style declarations (e.g., "max-width: 800px; margin: 0 auto;").
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    style: String,
    /// The pseudo-class and pseudo-element rules for this class
    /// (e.g., ":hover", ":focus", ":active", "::before", etc.).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pseudo_rules: Vec<PseudoRule>,
    /// The media query rules for this class.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    media_rules: Vec<MediaRule>,
}

/// Represents a CSS @media rule attached to a class.
///
/// Each media rule has a query string (e.g., "(max-width: 767px)")
/// and a style declaration string. When injected into the DOM, it produces
/// a rule like `@media (max-width: 767px) { .class-name { font-size: 14px; } }`.
#[derive(Clone, Data, Debug, Default, Eq, New, PartialEq)]
pub struct MediaRule {
    /// The media query condition string (e.g., "(max-width: 767px)").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    query: String,
    /// The CSS style declarations inside this media rule
    /// (e.g., "font-size: 14px; padding: 8px;").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    style: String,
}

/// Adapts various event value types into an `AttributeValue` for event attributes.
///
/// The `html!` macro generates `EventAdapter::new(expr).into_attribute(event_name)`
/// instead of inline trait dispatch boilerplate. This eliminates the per-attribute-site
/// generation of `__EventWrapper`, `__IsClosure`, `__ClosurePicker`, `__ValuePicker`,
/// `__FallbackHelper`, and `__dispatch` types, significantly reducing macro output size.
///
/// The adapter pattern handles three cases:
/// - `FnMut(NativeEvent)` closure → `AttributeValue::Event` via `NativeEventHandler`
/// - `NativeEventHandler` directly → `AttributeValue::Event` as-is
/// - `Option<NativeEventHandler>` → `AttributeValue::Event` or `AttributeValue::Text`
#[derive(Data, New)]
pub struct EventAdapter<T> {
    /// The wrapped value to be adapted into an attribute.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner: T,
}

/// Adapts an arbitrary attribute value expression into an `AttributeValue`.
///
/// Handles the dispatch between event closures and reactive values without
/// requiring the macro to generate inline trait hierarchies. The macro emits
/// `AttrValueAdapter::new(expr).into_attribute_value()` instead of the
/// `__IsClosure` / `__ClosurePicker` / `__ValuePicker` / `__FallbackHelper`
/// / `__dispatch` boilerplate.
///
/// For event attributes (key starts with "on"), event closures are wrapped
/// into `AttributeValue::Event`. For non-event attributes, values are
/// converted via `IntoReactiveValue`.
#[derive(Data, Debug, New)]
pub struct AttrValueAdapter<T> {
    /// The wrapped value to be adapted into an attribute.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner: T,
}

/// A `Sync` wrapper for single-threaded global `HashSet` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct InjectedClassesCell(
    /// Interior-mutable storage for the injected classes set.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) UnsafeCell<Option<HashSet<String>>>,
);
