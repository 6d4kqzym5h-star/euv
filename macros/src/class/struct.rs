use crate::*;

/// A parameter definition in a parameterized class.
///
/// Each parameter has a name (identifier), an explicit type annotation,
/// and is used as a placeholder in CSS property values.
pub(crate) struct ClassParam {
    /// The parameter name identifier.
    pub(crate) name: Ident,
    /// The explicit type annotation for this parameter (e.g., `&str`).
    pub(crate) ty: Type,
}

/// A pseudo-class or pseudo-element block parsed from the `class!` macro.
///
/// Represents a block like `hover { background: "red"; }` or
/// `focus { outline: "2px solid blue"; }`.
pub(crate) struct PseudoBlock {
    /// The pseudo selector string (e.g., ":hover", ":focus", "::before").
    pub(crate) selector: String,
    /// The style properties inside this pseudo block.
    pub(crate) properties: Vec<(String, ClassPropValue)>,
}

/// A media query block parsed from the `class!` macro.
///
/// Represents a block like `media "(max-width: 767px)" { font-size: "14px"; }`.
pub(crate) struct MediaBlock {
    /// The media query condition string (e.g., "(max-width: 767px)").
    pub(crate) query: String,
    /// The style properties inside this media block.
    pub(crate) properties: Vec<(String, ClassPropValue)>,
}

/// A single class definition parsed from the `class!` macro.
///
/// Contains visibility, name, optional parameters, style properties,
/// optional pseudo-class/pseudo-element blocks, and optional media query blocks.
pub(crate) struct ClassDef {
    /// The visibility modifier (e.g., `pub`, `pub(crate)`, `pub(super)`, or none).
    pub(crate) visibility: Visibility,
    /// The class name identifier.
    pub(crate) name: Ident,
    /// Optional parameter list for a parameterized class.
    pub(crate) params: Option<Vec<ClassParam>>,
    /// The style properties for this class.
    pub(crate) properties: Vec<(String, ClassPropValue)>,
    /// The pseudo-class and pseudo-element blocks for this class.
    pub(crate) pseudo_blocks: Vec<PseudoBlock>,
    /// The media query blocks for this class.
    pub(crate) media_blocks: Vec<MediaBlock>,
}

/// The entire `class!` macro input, containing multiple class definitions.
pub(crate) struct ClassInput {
    /// The list of class definitions.
    pub(crate) classes: Vec<ClassDef>,
}
