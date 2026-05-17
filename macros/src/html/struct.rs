use crate::*;

/// Represents the root of an `html!` macro invocation.
///
/// Contains zero or more top-level HTML nodes. The generated output depends
/// on the number of children:
/// - 0 nodes → `VirtualNode::Empty`
/// - 1 node → the node's token stream directly
/// - N nodes → `VirtualNode::Fragment(vec![...])`
pub(crate) struct HtmlRoot {
    /// The top-level nodes parsed from the macro input.
    pub(crate) children: Vec<HtmlNode>,
}

/// Represents a reactive `for` loop in HTML.
///
/// Syntax: `for pattern in {expr} { children }`
///
/// The pattern is a Rust binding pattern (e.g., `item` or `(index, item)`).
/// The expression in braces must evaluate to an iterable. Each iteration's
/// body is rendered as HTML and collected into a `VirtualNode::Fragment`.
pub(crate) struct HtmlFor {
    /// The binding pattern for loop variables (e.g., `item` or `(index, item)`).
    pub(crate) pattern: proc_macro2::TokenStream,
    /// The iterable expression (from the braces after `in`).
    pub(crate) iterable: Expr,
    /// The HTML nodes rendered for each iteration.
    pub(crate) body: Vec<HtmlNode>,
}

/// Represents a reactive `if` conditional in HTML.
///
/// Syntax: `if {expr} { children } [else if {expr} { children }]* [else { children }]`
pub(crate) struct HtmlIf {
    /// The list of condition-branch pairs. Each condition is a braced expression.
    /// The last entry may have `None` as condition (representing `else`).
    pub(crate) branches: Vec<(Option<Expr>, Vec<HtmlNode>)>,
}

/// Represents a reactive `if` conditional in attribute value position.
///
/// Syntax: `if {expr} { value } [else if {expr} { value }]* [else { value }]`
pub(crate) struct HtmlAttrIf {
    /// The list of condition-branch pairs. Each condition is a braced expression.
    /// The last entry may have `None` as condition (representing `else`).
    pub(crate) branches: Vec<(Option<Expr>, Expr)>,
}

/// Represents a reactive `match` expression in HTML.
///
/// Syntax: `match {expr} { pattern => { children } ... }`
pub(crate) struct HtmlMatch {
    /// The expression to match against (from the braces after `match`).
    pub(crate) scrutinee: Expr,
    /// The match arms: each arm has a pattern as a raw token stream and a body of HTML nodes.
    pub(crate) arms: Vec<(proc_macro2::TokenStream, Vec<HtmlNode>)>,
}

/// Represents an HTML element with a tag name, attributes, and children.
///
/// Stores the parsed structure of an HTML element for token generation.
pub(crate) struct HtmlElement {
    /// The tag name or component name.
    pub(crate) tag: Ident,
    /// The attributes of this element.
    pub(crate) attributes: Vec<(Ident, HtmlAttrValue)>,
    /// The child nodes.
    pub(crate) children: Vec<HtmlNode>,
    /// Whether this is a component (contains underscore in tag name).
    pub(crate) is_component: bool,
}
