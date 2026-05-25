mod r#fn;
mod r#impl;
mod r#struct;

pub(crate) use r#fn::{
    countdown_on_input, countdown_on_pause, countdown_on_reset, countdown_on_start,
    stopwatch_on_pause, stopwatch_on_reset, stopwatch_on_start, use_countdown, use_stopwatch,
};

pub(crate) use r#fn::use_interval;

pub(crate) use r#struct::*;
