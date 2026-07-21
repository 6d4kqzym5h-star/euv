use super::*;

/// The parsed input of the `watch!` macro.
///
/// Contains a list of signal expressions and a closure body with parameter names and optional types.
/// Parameter names are `None` when the closure uses `_` (anonymous) parameters.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct WatchInput {
    /// The signal expressions to watch.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) signals: Vec<Expr>,
    /// The parameter names of the closure. `None` for anonymous `_` parameters.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) param_names: Vec<Option<Ident>>,
    /// The optional parameter types of the closure.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) param_types: Vec<Option<Type>>,
    /// The closure body statements.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) body: Vec<Stmt>,
}
