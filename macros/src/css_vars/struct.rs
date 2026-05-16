use crate::*;

/// A parameter definition in a parameterized CSS variables block.
///
/// Each parameter has a name (identifier), an explicit type annotation,
/// and is used as a placeholder in CSS variable values.
pub(crate) struct CssVarParam {
    /// The parameter name identifier.
    pub(crate) name: Ident,
    /// The explicit type annotation for this parameter (e.g., `&str`).
    pub(crate) ty: SynType,
}

/// A single CSS variables block definition parsed from the `css_vars!` macro.
///
/// Contains visibility, name, optional parameters, and variable definitions.
pub(crate) struct CssVarDef {
    /// The visibility modifier (e.g., `pub`, `pub(crate)`, `pub(super)`, or none).
    pub(crate) visibility: Visibility,
    /// The CSS variables block name identifier.
    pub(crate) name: Ident,
    /// Optional parameter list for a parameterized CSS variables block.
    pub(crate) params: Option<Vec<CssVarParam>>,
    /// The CSS variable definitions (CSS key with -- prefix, value).
    pub(crate) vars: Vec<(String, CssVarValue)>,
}

/// The entire `css_vars!` macro input, containing multiple CSS variable block definitions.
pub(crate) struct CssVarInput {
    /// The list of CSS variable block definitions.
    pub(crate) defs: Vec<CssVarDef>,
}
