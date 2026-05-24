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
    cell::{Ref, RefCell, RefMut, UnsafeCell},
    collections::{HashMap, HashSet},
    mem::{swap, take},
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use {js_sys::*, lombok_macros::*, wasm_bindgen::prelude::*, wasm_bindgen::*, web_sys::*};

#[cfg(test)]
use std::cell::Cell;
