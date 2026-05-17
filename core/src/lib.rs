//! euv
//!
//! A declarative, cross-platform UI framework for Rust with virtual DOM,
//! reactive signals, and HTML macros for WebAssembly.

mod event;
mod reactive;
mod renderer;
mod vdom;

pub use {event::*, reactive::*, renderer::*, vdom::*};

use std::{
    any::Any,
    borrow::Cow,
    cell::UnsafeCell,
    collections::HashMap,
    mem::take,
    ops::{Deref, DerefMut},
    ptr::null_mut,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
#[cfg(test)]
use std::{cell::Cell, rc::Rc};

#[cfg(target_arch = "wasm32")]
use {
    js_sys::{Function, Reflect},
    wasm_bindgen::closure,
};
use {lombok_macros::*, wasm_bindgen::JsCast, wasm_bindgen::prelude::*, web_sys::*};
