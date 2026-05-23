use crate::*;

/// The parsed input of the `watch!` macro.
///
/// Contains a list of signal expressions and a closure body with parameter names.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct WatchInput {
    /// The signal expressions to watch.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) signals: Vec<Expr>,
    /// The parameter names of the closure.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) param_names: Vec<Ident>,
    /// The closure body statements.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) body: Vec<syn::Stmt>,
}
