mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;
mod r#type;

pub use r#fn::*;

pub(crate) use {r#static::*, r#struct::*, r#type::*};
