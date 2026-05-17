use crate::*;

/// Represents a CSS style property.
///
/// A single key-value pair representing a CSS declaration.
#[derive(Data, Default, New)]
pub struct StyleProperty {
    /// The CSS property name (e.g., "margin", "padding").
    #[get(pub)]
    #[set(pub)]
    name: String,
    /// The CSS property value.
    #[get(pub)]
    #[set(pub)]
    value: String,
}

/// A collection of CSS style properties that can be converted to a style string.
#[derive(Data, New)]
pub struct Style {
    /// The list of style properties.
    #[get(pub)]
    #[set(pub)]
    properties: Vec<StyleProperty>,
}

/// Represents a single attribute on a virtual DOM node.
///
/// Combines an attribute name with its corresponding value.
#[derive(Clone, Data, New)]
pub struct AttributeEntry {
    /// The name of the attribute.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: String,
    /// The value of the attribute.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) value: AttributeValue,
}

/// Represents a text node in the virtual DOM.
///
/// Text nodes may optionally be bound to a reactive signal for automatic updates.
#[derive(Clone, Data, New)]
pub struct TextNode {
    /// The text content.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content: String,
    /// An optional signal that drives reactive text updates.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) signal: Option<Signal<String>>,
}

/// A closure-based dynamic node that re-renders when its dependency signals change.
///
/// Holds a boxed closure that produces a fresh `VirtualNode` on each evaluation.
/// The renderer subscribes to the closure's signals and patches the DOM automatically.
/// Contains a `HookContext` that persists hook state (like `use_signal`) across
/// re-renders, ensuring that signal values are not reset when the render function
/// is called again.
#[derive(Data)]
pub struct DynamicNode {
    /// The closure that generates the dynamic virtual node tree.
    #[get(pub)]
    #[set(pub)]
    pub(crate) render_fn: Rc<RefCell<dyn FnMut() -> VirtualNode>>,
    /// Persistent hook context for this dynamic node, storing signal
    /// state and other hook values across render cycles.
    ///
    /// Implements `Copy`; all copies share the same underlying state.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) hook_context: HookContext,
    /// A stable identifier used to distinguish different dynamic nodes.
    ///
    /// When two `Dynamic` variants occupy the same slot in the virtual DOM tree
    /// (e.g. different arms of a `match` expression), equal ids indicate the
    /// same logical node (no DOM replacement needed) while different ids signal
    /// a structural swap (DOM must be rebuilt).
    #[get(pub)]
    #[set(pub)]
    pub(crate) id: u64,
}

/// Represents a CSS class with a name and its style declarations.
///
/// Created by the `class!` macro and used in `html!` via the `class:` attribute.
/// When the renderer encounters a `CssClass`, it injects the styles into the
/// DOM's `<style>` element on first use and applies the class name to the element.
#[derive(Clone, Data, Default)]
pub struct CssClass {
    /// The CSS class name used in the DOM.
    #[get(pub)]
    #[set(pub)]
    name: String,
    /// The CSS style declarations (e.g., "max-width: 800px; margin: 0 auto;").
    #[get(pub)]
    #[set(pub)]
    style: String,
}
