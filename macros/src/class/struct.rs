use crate::*;

/// A parameter definition in a parameterized class.
///
/// Each parameter has a name (identifier), an explicit type annotation,
/// and is used as a placeholder in CSS property values.
pub(crate) struct ClassParam {
    /// The parameter name identifier.
    pub(crate) name: Ident,
    /// The explicit type annotation for this parameter (e.g., `&str`).
    pub(crate) ty: syn::Type,
}

/// A single class definition parsed from the `class!` macro.
///
/// Contains visibility, name, optional parameters, and style properties.
pub(crate) struct ClassDef {
    /// The visibility modifier (e.g., `pub`, `pub(crate)`, `pub(super)`, or none).
    pub(crate) visibility: syn::Visibility,
    /// The class name identifier.
    pub(crate) name: Ident,
    /// Optional parameter list for a parameterized class.
    pub(crate) params: Option<Vec<ClassParam>>,
    /// The style properties for this class.
    pub(crate) properties: Vec<(String, ClassPropValue)>,
}

/// The entire `class!` macro input, containing multiple class definitions.
pub(crate) struct ClassInput {
    /// The list of class definitions.
    pub(crate) classes: Vec<ClassDef>,
}
