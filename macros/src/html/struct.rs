use crate::*;

/// Represents the root of an `html!` macro invocation.
///
/// Contains zero or more top-level HTML nodes. The generated output depends
/// on the number of children:
/// - 0 nodes → `VirtualNode::Empty`
/// - 1 node → the node's token stream directly
/// - N nodes → `VirtualNode::Fragment(vec![...])`
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlRoot {
    /// The top-level nodes parsed from the macro input.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) children: Vec<HtmlNode>,
}

/// Represents a `for` loop in HTML.
///
/// Syntax:
/// - Reactive: `for pattern in {expr} { children }`
///   The iterable expression in braces is treated as a signal that triggers re-rendering.
/// - Inline: `for pattern in expr { children }`
///   The iterable is a plain Rust expression, evaluated once at render time.
///
/// The pattern is a Rust binding pattern (e.g., `item` or `(index, item)`).
/// Each iteration's body is rendered as HTML and collected into a
/// `VirtualNode::Fragment`.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlFor {
    /// Whether this for loop is reactive (iterable wrapped in braces as a signal).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_reactive: bool,
    /// The binding pattern for loop variables (e.g., `item` or `(index, item)`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pattern: proc_macro2::TokenStream,
    /// The iterable expression (from the braces after `in`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) iterable: Expr,
    /// The HTML nodes rendered for each iteration.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) body: Vec<HtmlNode>,
}

/// Represents a conditional in HTML.
///
/// Each branch condition is independently either reactive (braced `{expr}`)
/// or inline (plain expression). The `is_reactive` flag is `true` if any
/// branch has a braced condition, causing the entire if-chain to be wrapped
/// in a `DynamicNode` for reactive re-rendering.
///
/// Supported combinations include:
/// - `if {a} {} else if {b} {}` — all reactive
/// - `if a {} else if b {}` — all inline
/// - `if {a} {} else if b {}` — mixed (first reactive, second inline)
/// - `if a {} else if {b} {}` — mixed (first inline, second reactive)
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlIf {
    /// Whether this conditional has at least one reactive (braced) branch.
    ///
    /// When `true`, the entire if-chain is wrapped in a `DynamicNode` that
    /// re-evaluates when any braced signal changes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_reactive: bool,
    /// The list of condition-branch tuples.
    ///
    /// Each entry is `(condition, body, is_condition_reactive)` where:
    /// - `condition` is the parsed Rust expression, or `None` for the
    ///   trailing `else` branch.
    /// - `body` is the list of HTML child nodes rendered when this branch
    ///   is selected.
    /// - `is_condition_reactive` is `true` when the condition was written
    ///   inside `{}` (reactive). In that case a single-segment identifier
    ///   expression is auto-`.get()`-ed during codegen.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) branches: Vec<(Option<Expr>, Vec<HtmlNode>, bool)>,
}

/// Represents a reactive or inline `if` conditional in attribute value position.
///
/// Each branch condition is independently either reactive (braced `{expr}`)
/// or inline (plain expression). The `is_inline` flag is `true` only when
/// all branches are inline; if any branch is reactive, the entire if-chain
/// is wrapped in a reactive `AttributeValue`.
///
/// Supported combinations include:
/// - `if {a} { v } else if {b} { v }` — all reactive
/// - `if a { v } else if b { v }` — all inline
/// - `if {a} { v } else if b { v }` — mixed (first reactive, second inline)
/// - `if a { v } else if {b} { v }` — mixed (first inline, second reactive)
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlAttrIf {
    /// Whether this conditional is entirely inline (no braced conditions).
    ///
    /// When `false`, at least one branch is reactive and the entire if-chain
    /// is wrapped in a reactive `AttributeValue`.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_inline: bool,
    /// The list of condition-branch tuples.
    ///
    /// Each entry is `(condition, body, is_condition_reactive)`. The last
    /// entry may have `None` as condition (representing `else`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) branches: Vec<(Option<Expr>, Expr, bool)>,
    /// The default token stream for the implicit else branch when no explicit else exists.
    ///
    /// For `class` attributes, this is an empty string. For `style` attributes,
    /// this is also an empty string. For other attributes, an empty string is used.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) else_default: proc_macro2::TokenStream,
}

/// Represents a reactive or inline `match` expression in attribute value position.
///
/// Supports two syntaxes:
/// - Reactive: `match {expr} { pattern => value, ... }`
///   The expression in braces is treated as a signal that triggers re-rendering.
/// - Inline: `match expr { pattern => value, ... }`
///   The expression is a plain Rust expression, evaluated once at render time.
///   This form is typically used inside `for` loops where the scrutinee depends on loop variables.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlAttrMatch {
    /// Whether this match is inline (scrutinee is a plain expression, not a braced signal).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_inline: bool,
    /// The expression to match against.
    ///
    /// For reactive match, this is a braced signal expression.
    /// For inline match, this is a plain Rust expression.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scrutinee: Expr,
    /// The match arms: each arm has a pattern as a raw token stream and a body expression.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) arms: Vec<(proc_macro2::TokenStream, Expr)>,
}

/// Represents a `match` expression in HTML.
///
/// Syntax:
/// - Reactive: `match {expr} { pattern => { children } ... }`
///   The scrutinee expression in braces is treated as a signal that triggers re-rendering.
/// - Inline: `match expr { pattern => { children } ... }`
///   The scrutinee is a plain Rust expression, evaluated once at render time.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlMatch {
    /// Whether this match is reactive (scrutinee wrapped in braces as a signal).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_reactive: bool,
    /// The expression to match against (from the braces after `match` for reactive, or bare expression for inline).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scrutinee: Expr,
    /// The match arms: each arm has a pattern as a raw token stream and a body of HTML nodes.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) arms: Vec<(proc_macro2::TokenStream, Vec<HtmlNode>)>,
}

/// Represents an HTML element with a tag name, attributes, and children.
///
/// Stores the parsed structure of an HTML element for token generation.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlElement {
    /// The tag identifier (used for function calls on Ident tags).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tag: Ident,
    /// The actual tag name string, which may differ from the identifier
    /// when using string literal tags for custom HTML5 elements.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tag_name: String,
    /// Whether the tag was parsed from an identifier (not a string literal).
    /// Ident tags are treated as function calls; string literal tags produce
    /// native HTML elements directly.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_ident_tag: bool,
    /// The attributes of this element.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) attributes: HtmlAttrs,
    /// The child nodes.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) children: Vec<HtmlNode>,
}

/// Represents a dynamic tag in HTML.
///
/// Syntax: `{tag_expr} { attr: value, ... children ... }`.
///
/// The expression in braces evaluates to a tag name string at runtime.
/// If the tag name matches a registered user component function, that
/// component is called with the attributes and children. Otherwise,
/// a native HTML element is created.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlDynamicTag {
    /// The expression that evaluates to a tag name string.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tag_expr: Expr,
    /// The attributes passed to the dynamic element.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) attributes: HtmlAttrs,
    /// The child nodes of the dynamic element.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) children: Vec<HtmlNode>,
}

/// Stores metadata about a registered component function.
///
/// Contains the Props type name, the list of field names, and a map of
/// field name to type string declared in the Props struct, used for
/// attribute filtering and type-aware code generation in dynamic tags.
#[derive(Clone, Data, Debug, New, serde::Deserialize, serde::Serialize)]
pub(crate) struct ComponentInfo {
    /// The Props type name (e.g., "PrimaryButtonProps").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) props_type: String,
    /// The field names declared in the Props struct.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) props_fields: Vec<String>,
    /// Maps field name to its type string (e.g., "children" -> "VirtualNode").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) props_field_types: HashMap<String, String>,
}

/// Bundle of parameters for emitting an attribute-level `if` chain as tokens.
///
/// Groups the conditional AST, the implicit `else` default, and the wrapping
/// mode (`Reactive` / `Raw`) so the helper takes a single argument instead
/// of three positional ones, and so callers don't have to thread the same
/// three values through multiple helpers.
#[derive(Clone, Copy, Data, Debug, New)]
pub(crate) struct AttrIfContext<'a> {
    /// The parsed attribute-level `if` conditional.
    #[get(pub(crate))]
    pub(crate) html_attr_if: &'a HtmlAttrIf,
    /// The default else-branch token stream (used when no explicit `else`).
    #[get(pub(crate))]
    pub(crate) else_default: &'a proc_macro2::TokenStream,
    /// The wrapping mode (`Reactive` or `Raw`).
    #[get(pub(crate), type(copy))]
    pub(crate) mode: AttrIfMode,
}

/// Bundle of parameters for emitting an `HtmlAttrValue` into an `AttributeValue`.
///
/// Combines the attribute value AST, the attribute key string, and the
/// component flag into a single struct so callers don't have to pass three
/// positional arguments to the same helper from multiple sites.
#[derive(Clone, Copy, Data, Debug, New)]
pub(crate) struct AttrValueContext<'a> {
    /// The attribute value AST.
    #[get(pub(crate))]
    pub(crate) value: &'a HtmlAttrValue,
    /// The attribute key (e.g., `"class"`, `"onclick"`).
    #[get(pub(crate))]
    pub(crate) key_str: &'a str,
    /// Whether the attribute belongs to a component (affects event adapter
    /// selection and the resulting `AttributeValue` variant).
    #[get(pub(crate), type(copy))]
    pub(crate) is_component: bool,
}

/// Bundle of parameters for emitting an `HtmlAttrValue` into an
/// `AttributeEntry::new(...)` value (used by element / dynamic tag emission).
#[derive(Clone, Copy, Data, Debug, New)]
pub(crate) struct AttrEntryContext<'a> {
    /// The attribute value AST.
    #[get(pub(crate))]
    pub(crate) value: &'a HtmlAttrValue,
    /// The attribute key (e.g., `"class"`, `"onclick"`).
    #[get(pub(crate))]
    pub(crate) key_str: &'a str,
}
