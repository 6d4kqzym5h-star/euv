mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;

pub(crate) use {r#fn::*, r#static::*};
