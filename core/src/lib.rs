//! euv
//!
//! A declarative, cross-platform UI framework for Rust with virtual DOM,
//! reactive signals, and HTML macros for WebAssembly.

pub mod component;
pub mod event;
pub mod platform;
pub mod reactive;
pub mod renderer;
pub mod vdom;

pub use {component::*, event::*, reactive::*, renderer::*, vdom::*};

#[cfg(test)]
use std::cell::Cell;
use std::{
    any::Any,
    cell::{RefCell, RefMut, UnsafeCell},
    collections::HashMap,
    ops::{Deref, DerefMut},
    ptr::null_mut,
    rc::Rc,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use {
    lombok_macros::*,
    wasm_bindgen::JsCast,
    wasm_bindgen::prelude::*,
    web_sys::{
        ClipboardEvent, Document, DragEvent, Element, Event, HtmlButtonElement, HtmlElement,
        HtmlInputElement, HtmlOptionElement, HtmlSelectElement, HtmlTextAreaElement, InputEvent,
        KeyboardEvent, MouseEvent, Node, SubmitEvent, Text, TouchEvent, WheelEvent, Window, window,
    },
};
