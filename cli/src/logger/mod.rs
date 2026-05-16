mod r#const;
mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;

pub use {color_output::*, log};

use {r#const::*, r#static::*};
