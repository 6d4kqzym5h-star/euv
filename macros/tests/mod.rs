mod class;
mod component;
mod computed;
mod r#fn;
mod unsafe_no_inline;
mod var;
mod vars;
mod watch;

use euv::*;

// Proc-macros defined in this crate (e.g. `class!`,
// `vars!`, `watch!`, `computed!`, `unsafe_no_inline!`,
// `#[component]`) are resolved by their absolute path
// within the integration test root — no explicit `use`
// is needed. The previous `use euv_macros::*;` was
// rejected by clippy as an unused import.

pub(crate) use r#fn::*;
