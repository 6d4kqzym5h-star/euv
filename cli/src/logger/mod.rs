mod r#const;
mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;

pub use {color_output::*, log};

pub(crate) use {r#const::*, r#static::*};
