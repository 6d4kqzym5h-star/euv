mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;
#[cfg(test)]
mod test;

pub use r#struct::*;

pub(crate) use {r#fn::*, r#static::*};
