use crate::*;

/// Represents a CSS style property.
///
/// A single key-value pair representing a CSS declaration.
#[derive(Data, Default)]
pub struct StyleProperty {
    /// The CSS property name (e.g., "margin", "padding").
    name: String,
    /// The CSS property value.
    value: String,
}

/// A collection of CSS style properties that can be converted to a style string.
#[derive(Data, New)]
pub struct Style {
    /// The list of style properties.
    properties: Vec<StyleProperty>,
}

/// Represents a single attribute on a virtual DOM node.
///
/// Combines an attribute name with its corresponding value.
#[derive(Clone, Data, New)]
pub struct AttributeEntry {
    /// The name of the attribute.
    pub(crate) name: String,
    /// The value of the attribute.
    pub(crate) value: AttributeValue,
}

/// Represents a text node in the virtual DOM.
///
/// Text nodes may optionally be bound to a reactive signal for automatic updates.
#[derive(Clone, Data, New)]
pub struct TextNode {
    /// The text content.
    pub(crate) content: String,
    /// An optional signal that drives reactive text updates.
    pub(crate) signal: Option<Signal<String>>,
}

/// A closure-based dynamic node that re-renders when its dependency signals change.
///
/// Holds a boxed closure that produces a fresh `VirtualNode` on each evaluation.
/// The renderer subscribes to the closure's signals and patches the DOM automatically.
/// Contains a `HookContext` that persists hook state (like `use_signal`) across
/// re-renders, ensuring that signal values are not reset when the render function
/// is called again.
pub struct DynamicNode {
    /// The closure that generates the dynamic virtual node tree.
    pub render_fn: Rc<RefCell<dyn FnMut() -> VirtualNode>>,
    /// Persistent hook context for this dynamic node, storing signal
    /// state and other hook values across render cycles.
    ///
    /// Implements `Copy`; all copies share the same underlying state.
    pub hook_context: HookContext,
}

/// Represents a CSS class with a name and its style declarations.
///
/// Created by the `class!` macro and used in `rsx!` via the `class:` attribute.
/// When the renderer encounters a `CssClass`, it injects the styles into the
/// DOM's `<style>` element on first use and applies the class name to the element.
#[derive(Clone, Data, Default)]
pub struct CssClass {
    /// The CSS class name used in the DOM.
    name: String,
    /// The CSS style declarations (e.g., "max-width: 800px; margin: 0 auto;").
    style: String,
}
