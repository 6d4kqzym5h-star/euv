mod dom;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;
mod r#type;

pub use {r#fn::*, r#struct::*};

pub(crate) use {dom::*, r#static::*, r#type::*};
