//! euv
//!
//! A declarative, cross-platform UI framework for Rust with virtual DOM,
//! reactive signals, and HTML macros for WebAssembly.

mod app;
mod event;
mod noderef;
mod reactive;
mod renderer;
pub mod vdom;

#[cfg(test)]
mod tests;

pub use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    panic::{AssertUnwindSafe, catch_unwind},
};
pub use {app::*, event::*, noderef::*, reactive::*, vdom::*};

pub(crate) use renderer::*;

use std::{
    any::Any,
    borrow::Cow,
    cell::{Cell, Ref, RefCell, UnsafeCell},
    collections::{HashMap, HashSet},
    fmt::{self, Display, Formatter},
    future::Future,
    iter::Iterator,
    marker::PhantomData,
    mem::{swap, take},
    num::ParseIntError,
    ops::Deref,
    rc::Rc,
    sync::{
        LazyLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    vec::Vec,
};

use {js_sys::*, lombok_macros::*, wasm_bindgen::prelude::*, web_sys::*};
