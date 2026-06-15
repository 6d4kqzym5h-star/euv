//! euv Example
//!
//! A demonstration application showcasing the euv component system,
//! reactive signals, routing, and HTML macros.

mod component;
mod page;
mod style;

use {component::*, page::*, style::*};

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use euv::{js_sys::*, wasm_bindgen::prelude::*, wasm_bindgen_futures::*, web_sys::*, *};

use {
    lombok_macros::*,
    qrcode::{QrCode, render::svg},
    serde::Deserialize,
};

/// Entry point for the euv example application.
#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    inject_app_global_css();
    mount("#app", app);
}
