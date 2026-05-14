use crate::*;

/// Represents a reactive `if` conditional in RSX.
///
/// Syntax: `if {expr} { children } [else if {expr} { children }]* [else { children }]`
pub(crate) struct RsxIf {
    /// The list of condition-branch pairs. Each condition is a braced expression.
    /// The last entry may have `None` as condition (representing `else`).
    pub(crate) branches: Vec<(Option<Expr>, Vec<RsxNode>)>,
}

/// Represents a reactive `match` expression in RSX.
///
/// Syntax: `match {expr} { pattern => { children } ... }`
pub(crate) struct RsxMatch {
    /// The expression to match against (from the braces after `match`).
    pub(crate) scrutinee: Expr,
    /// The match arms: each arm has a pattern as a raw token stream and a body of RSX nodes.
    pub(crate) arms: Vec<(TokenStream2, Vec<RsxNode>)>,
}

/// Represents an RSX element with a tag name, attributes, and children.
///
/// Stores the parsed structure of an RSX element for token generation.
pub(crate) struct RsxElement {
    /// The tag name or component name.
    pub(crate) tag: Ident,
    /// The attributes of this element.
    pub(crate) attributes: Vec<(Ident, RsxAttrValue)>,
    /// The child nodes.
    pub(crate) children: Vec<RsxNode>,
    /// Whether this is a component (contains underscore in tag name).
    pub(crate) is_component: bool,
}
