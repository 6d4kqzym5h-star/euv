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
    cell::{Ref, RefCell, UnsafeCell},
    collections::{HashMap, HashSet},
    fmt::{self, Display, Formatter},
    marker::PhantomData,
    mem::{swap, take},
    num::ParseIntError,
    rc::Rc,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use {js_sys::*, lombok_macros::*, wasm_bindgen::prelude::*, web_sys::*};
