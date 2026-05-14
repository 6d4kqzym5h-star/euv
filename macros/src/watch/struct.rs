use crate::*;

/// The parsed input of the `watch!` macro.
///
/// Contains a list of signal expressions and a closure body with parameter names.
pub(crate) struct WatchInput {
    /// The signal expressions to watch.
    pub(crate) signals: Vec<Expr>,
    /// The parameter names of the closure.
    pub(crate) param_names: Vec<Ident>,
    /// The closure body statements.
    pub(crate) body: Vec<syn::Stmt>,
}
