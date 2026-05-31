use crate::*;

/// A parameter definition in a parameterized class.
///
/// Each parameter has a name (identifier), an explicit type annotation,
/// and is used as a placeholder in CSS property values.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct ClassParam {
    /// The parameter name identifier.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: Ident,
    /// The explicit type annotation for this parameter (e.g., `&str`).
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) param_type: Type,
}

/// A pseudo-class or pseudo-element block parsed from the `class!` macro.
///
/// Represents a block like `hover { background: "red"; }` or
/// `focus { outline: "2px solid blue"; }`.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct PseudoBlock {
    /// The pseudo selector string (e.g., ":hover", ":focus", "::before").
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) selector: String,
    /// The style properties inside this pseudo block.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) properties: Vec<(ClassPropKey, ClassPropValue)>,
}

/// A media query block parsed from the `class!` macro.
///
/// Represents a block like `media "(max-width: 767px)" { font-size: "14px"; }`.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct MediaBlock {
    /// The media query condition string (e.g., "(max-width: 767px)").
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) query: String,
    /// The style properties inside this media block.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) properties: Vec<(ClassPropKey, ClassPropValue)>,
}

/// A parent class reference in an extends clause.
///
/// Represents a class inheritance call like `c_primary_button(1)` or `c_base()`.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct ClassExtend {
    /// The parent class name identifier.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: Ident,
    /// The arguments passed to the parent class function (as raw token streams).
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) args: Vec<proc_macro2::TokenStream>,
}

/// A single class definition parsed from the `class!` macro.
///
/// Contains visibility, name, optional parameters, style properties,
/// optional pseudo-class/pseudo-element blocks, and optional media query blocks.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct ClassDef {
    /// The visibility modifier (e.g., `pub`, `pub(crate)`, `pub(super)`, or none).
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) visibility: Visibility,
    /// The class name identifier.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: Ident,
    /// Optional parameter list for a parameterized class.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) params: Option<Vec<ClassParam>>,
    /// Parent classes to inherit properties from (e.g., `c_primary_button(1)`).
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) extends: Vec<ClassExtend>,
    /// The style properties for this class.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) properties: Vec<(ClassPropKey, ClassPropValue)>,
    /// The pseudo-class and pseudo-element blocks for this class.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pseudo_blocks: Vec<PseudoBlock>,
    /// The media query blocks for this class.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) media_blocks: Vec<MediaBlock>,
}

/// The entire `class!` macro input, containing multiple class definitions.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct ClassInput {
    /// The list of class definitions.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) classes: Vec<ClassDef>,
}

/// Parameters for `emit_once_lock_fn`.
///
/// Bundles the values needed to generate a `OnceLock`-based static function body
/// for a no-param class definition.
pub(crate) struct OnceLockParams<'a> {
    /// The visibility modifier for the generated function.
    pub(crate) visibility: &'a Visibility,
    /// The function name token stream.
    pub(crate) fn_name_token: &'a proc_macro2::TokenStream,
    /// The `OnceLock` constant name token stream.
    pub(crate) const_name_token: &'a proc_macro2::TokenStream,
    /// The class name as a string literal.
    pub(crate) class_name_str: &'a str,
    /// Token stream that evaluates to the CSS style string.
    pub(crate) style_expr: &'a proc_macro2::TokenStream,
    /// Token stream that evaluates to the pseudo rules vector.
    pub(crate) pseudo_expr: &'a proc_macro2::TokenStream,
    /// Token stream that evaluates to the media rules vector.
    pub(crate) media_expr: &'a proc_macro2::TokenStream,
}
