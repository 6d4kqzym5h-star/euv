/// Environment variable name for the project manifest directory.
pub(crate) const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

/// Source directory name within a Cargo project.
pub(crate) const SRC_DIR: &str = "src";

/// Rust source file extension.
pub(crate) const RUST_FILE_EXTENSION: &str = "rs";

/// Attribute name for marking component functions.
pub(crate) const COMPONENT_ATTR: &str = "component";

/// Attribute key name for CSS class bindings.
pub(crate) const ATTR_KEY_CLASS: &str = "class";

/// Attribute key name for inline style bindings.
pub(crate) const ATTR_KEY_STYLE: &str = "style";

/// Attribute key name for children slot bindings.
pub(crate) const ATTR_KEY_CHILDREN: &str = "children";

/// Prefix for event handler attribute keys (e.g., `onclick`, `onchange`).
pub(crate) const EVENT_ATTR_PREFIX: &str = "on";

/// The Rust raw identifier prefix.
pub(crate) const RAW_IDENT_PREFIX: &str = "r#";

/// The hyphen character used in kebab-case names and CSS property conversion.
pub(crate) const CHAR_HYPHEN: char = '-';

/// The hyphen string used for replacing underscores in kebab-case conversion.
pub(crate) const STR_HYPHEN: &str = "-";

/// The underscore character, part of Rust identifiers.
pub(crate) const CHAR_UNDERSCORE: char = '_';

/// The underscore string used for replacing hyphens in Rust identifier conversion.
pub(crate) const STR_UNDERSCORE: &str = "_";

/// The space character used in CSS string formatting.
pub(crate) const CHAR_SPACE: char = ' ';

/// The CSS declaration terminator character.
pub(crate) const CHAR_CSS_DECL_TERMINATOR: char = ';';

/// Error message when HTML root content is not a valid element or expression.
pub(crate) const ERR_EXPECTED_ELEMENT: &str =
    "expected an element, string literal, if, match, for, or expression";

/// Error message when an unexpected token is encountered inside an HTML element.
pub(crate) const ERR_UNEXPECTED_TOKEN_IN_ELEMENT: &str = "unexpected token in HTML element";

/// Error message when an unexpected token is encountered in HTML children.
pub(crate) const ERR_UNEXPECTED_TOKEN_IN_HTML: &str = "unexpected token in HTML";

/// Error message when an unexpected token is encountered inside a dynamic component.
pub(crate) const ERR_UNEXPECTED_TOKEN_IN_DYNAMIC_COMPONENT: &str =
    "unexpected token in dynamic component";

/// Type name for `VirtualNode`, used to determine the else default value in component props if-chains.
pub(crate) const TYPE_VIRTUAL_NODE: &str = "VirtualNode";

/// Type name identifier for `VirtualNode`, used to detect `VirtualNode<T>` parameter types.
pub(crate) const VIRTUAL_NODE_TYPE: &str = "VirtualNode";
