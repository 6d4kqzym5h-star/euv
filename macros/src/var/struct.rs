use crate::*;

/// A parameter definition in a parameterized vars block.
///
/// Each parameter has a name (identifier), an explicit type annotation,
/// and is used as a placeholder in vars variable values.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct VarsParam {
    /// The parameter name identifier.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: Ident,
    /// The explicit type annotation for this parameter (e.g., `&str`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) param_type: Type,
}

/// A single vars block definition parsed from the `vars!` macro.
///
/// Contains visibility, name, optional parameters, and variable definitions.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct VarsDef {
    /// The visibility modifier (e.g., `pub`, `pub(crate)`, `pub(super)`, or none).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) visibility: Visibility,
    /// The CSS variables block name identifier.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: Ident,
    /// Optional parameter list for a parameterized vars block.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) params: Option<Vec<VarsParam>>,
    /// The variable definitions (CSS key with -- prefix, value).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) vars: Vec<(String, VarsValue)>,
}

/// The entire `vars!` macro input, containing multiple vars block definitions.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct VarsInput {
    /// The list of vars block definitions.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) defs: Vec<VarsDef>,
}
