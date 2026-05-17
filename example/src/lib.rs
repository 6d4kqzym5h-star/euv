//! euv Example
//!
//! A demonstration application showcasing the euv component system,
//! reactive signals, routing, and HTML macros.

mod app;
mod component;
mod page;
mod router;
mod style;
mod theme;

use {app::*, component::*, page::*, router::*, style::*, theme::*};

use {euv_core::*, euv_macros::*};

use {js_sys::*, lombok_macros::*, wasm_bindgen::prelude::*, wasm_bindgen_futures::*, web_sys::*};

/// Entry point for the euv example application.
#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    mount("#app", app);
}
