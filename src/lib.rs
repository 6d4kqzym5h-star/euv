//! euv
//!
//! A declarative, cross-platform UI framework for Rust with virtual DOM,
//! reactive signals, and HTML macros for WebAssembly.

pub use {euv_core::*, euv_macros::*};

pub mod vdom {
    pub use ::euv_core::vdom::*;
}

pub use {console_error_panic_hook, js_sys, wasm_bindgen, wasm_bindgen_futures, web_sys};
