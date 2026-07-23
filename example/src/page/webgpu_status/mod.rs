//! Shared diagnostic helpers for the WebGPU demo tabs.
//!
//! Both the 2D and 3D game pages render a "Status:" banner whose text
//! is decided by `webgpu_status_text`. The strings the banner may
//! show live in [`const`] so they stay aligned across pages.
mod r#const;
mod r#fn;

pub(crate) use r#fn::*;
