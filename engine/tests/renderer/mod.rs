//! Renderer integration tests.
//!
//! The `euv-engine` integration tests are organised by renderer family so
//! the renderer-specific smoke tests live together under `renderer/`. This
//! module is the conventional Rust test-module entry point: it re-exports
//! every `#[test]` defined in the sibling test files so `cargo test` runs
//! them all without the test binary having to know the internal layout.
//!
//! Currently the `renderer/` tree covers the WebGPU render path
//! (descriptor defaults, dynamic offsets, async readback, error scope,
//! mipmaps, `writeTexture`). As other renderer backends (Canvas 2D,
//! WebGL fallback) land, their compile-shape tests belong here too.
//!
//! Sub-modules:
//! - [`fn`] — Compile-shape and pure-logic smoke tests for the
//!   `WebGpuRenderer` API surface.
//!
//! Tests that exercise the full engine pipeline (e.g. `input_state_shape.rs`)
//! continue to live at the `tests/` root because they are engine-wide
//! rather than renderer-specific.

mod r#fn;
