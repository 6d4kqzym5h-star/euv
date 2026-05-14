//! euv Example
//!
//! A demonstration application showcasing the euv component system,
//! reactive signals, routing, and RSX macros.

mod app;
mod component;
mod page;
mod router;
mod style;

use {app::*, component::*, page::*, router::*, style::*};

use {euv::*, wasm_bindgen::prelude::*, web_sys::*};

use wasm_bindgen::JsCast;

/// Entry point for the euv example application.
#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    mount("#app", app);
}
