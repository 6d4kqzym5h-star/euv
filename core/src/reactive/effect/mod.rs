pub(crate) mod r#const;
pub(crate) mod r#fn;
pub(crate) mod r#impl;
pub(crate) mod r#static;
mod r#struct;

pub use {r#fn::*, r#struct::*};

pub(crate) use {r#const::*, r#static::*};
