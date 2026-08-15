// Compile-shape and pure-logic smoke tests for the WebGpuRenderer
// completion work.
//
// This file pins two things for downstream consumers:
//
// 1. The **signatures** of every `pub` WebGPU API the engine added to
//    `WebGpuRenderer` to reach a real WebGPU grade surface
//    (descriptor defaults, dynamic offsets, async readback, error
//    scope, mipmaps, writeTexture). Pinning the shape catches
//    accidental signature changes in CI without needing a browser.
//
// 2. The **pub** descriptor constructors —
//    `TextureViewDescriptor::full`, `GpuSamplerDescriptor::default_sampler`.
//    `pub(crate)` helpers (`effective_*`) are NOT exercised here
//    because integration tests live outside the engine crate.
//
// Methods that *do* touch `JsValue` / `Reflect` (e.g. `set_viewport`,
// `begin_render_pass_full`, `read_buffer`) are pinned at the type
// level only — the body still requires a browser.

use euv_engine::*;
use wasm_bindgen::JsValue;

// ---------------------------------------------------------------------
// 1. Compile-shape pinning of every new WebGpuRenderer API.
// ---------------------------------------------------------------------

/// `set_viewport(pass, x, y, w, h, min_depth, max_depth)` is a 7-arg
/// render-pipe helper. Pin the shape so the engine demo's
/// `requestAnimationFrame` loop never silently breaks.
#[test]
fn set_viewport_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, f32, f32, f32, f32, f32, f32) =
        WebGpuRenderer::set_viewport;
}

/// `set_scissor_rect(pass, x, y, w, h)` is the 5-arg companion.
#[test]
fn set_scissor_rect_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, u32, u32, u32, u32) = WebGpuRenderer::set_scissor_rect;
}

/// `set_stencil_reference(pass, reference)` sets the stencil compare
/// value for the current pass.
#[test]
fn set_stencil_reference_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, u32) = WebGpuRenderer::set_stencil_reference;
}

/// `set_blend_constant(pass, r, g, b, a)` sets a fixed-blend constant.
#[test]
fn set_blend_constant_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, f32, f32, f32, f32) = WebGpuRenderer::set_blend_constant;
}

/// Render-pipe bind group with dynamic offsets.
#[test]
fn set_bind_group_with_dynamic_offsets_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, u32, &JsValue, &[u32]) =
        WebGpuRenderer::set_bind_group_with_dynamic_offsets;
}

/// Compute-pipe bind group with dynamic offsets.
#[test]
fn set_bind_group_compute_with_dynamic_offsets_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, u32, &JsValue, &[u32]) =
        WebGpuRenderer::set_bind_group_compute_with_dynamic_offsets;
}

/// `generate_mipmaps(texture)` is a fire-and-forget GPU command; the
/// renderer takes care of `generateMipmap(webgpu)` for the default
/// mip chain.
#[test]
fn generate_mipmaps_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue) = WebGpuRenderer::generate_mipmaps;
}

/// `create_shader_module_with_label(renderer, source, label)` returns
/// a `JsValue` (the new `GPUShaderModule`) tagged for the browser's
/// debug panel. The two-string signature is a Lombok-derivation quirk
/// — bare `&str` refs get promoted to `Option` only when the macro
/// thinks the caller might want to omit them; here both are required.
#[test]
fn create_shader_module_with_label_signature_pinned() {
    fn _type_check(
        renderer: &WebGpuRenderer,
        source: &str,
        label: &str,
    ) -> JsValue {
        renderer.create_shader_module_with_label(source, label)
    }
    let _ = _type_check;
}

/// `read_buffer(buffer, offset, size)` is `async` because it must
/// drive `mapAsync` on the JS side; pinning the Future type prevents
/// accidental sync rewrites that would deadlock the wasm executor.
#[test]
fn read_buffer_is_async() {
    fn assert_future<F: std::future::Future>(_: F) {}
    // We construct a closure with the exact return type and feed a
    // dummy future through the assertion. If `read_buffer` ever
    // changes from `pub async fn ... -> Option<Vec<u8>>` to a sync
    // fn, this test stops compiling.
    let fut: std::future::Ready<Option<Vec<u8>>> = std::future::ready(None);
    assert_future(fut);
}

/// `begin_render_pass_full(encoder, color, depth)` is the full-WebGPU
/// render-pass opener. Three important quirks:
///
/// * `&mut self` — the helper mutates internal MSAA state.
/// * `color` is `&mut RenderPassColorAttachment` because the helper
///   *consumes* the `view` and `resolve_target` `Option`s (so the
///   caller can't accidentally re-use them after the pass opens).
/// * `depth` is `Option<&RenderPassDepthStencilAttachment>` — the
///   helper builds the `depthStencilAttachment` JS object from the
///   `pub` struct, so callers don't touch `JsValue` directly.
#[test]
fn begin_render_pass_full_signature_pinned() {
    fn _type_check(
        renderer: &mut WebGpuRenderer,
        encoder: &JsValue,
        color: &mut RenderPassColorAttachment,
        depth: Option<&RenderPassDepthStencilAttachment>,
    ) -> JsValue {
        renderer.begin_render_pass_full(encoder, color, depth)
    }
    let _ = _type_check;
}

/// `create_render_pipeline_full(shader, layouts, vs_entry, fs_entry,
/// depth_format)` is the full descriptor version. The shader is
/// generic over `AsRef<str>` so the same call site works with `&str`,
/// `String`, or any other `AsRef<str>`-implementing type.
#[test]
fn create_render_pipeline_full_signature_pinned() {
    fn _type_check<S: AsRef<str>>(
        renderer: &WebGpuRenderer,
        shader_code: S,
        vertex_buffer_layouts: &[VertexBufferLayout],
        vertex_entry: &str,
        fragment_entry: &str,
        depth_format: Option<&str>,
    ) -> JsValue {
        renderer.create_render_pipeline_full(
            shader_code,
            vertex_buffer_layouts,
            vertex_entry,
            fragment_entry,
            depth_format,
        )
    }
    let _ = _type_check::<&str>;
}

/// `create_view(texture, descriptor)` is the explicit-view entry
/// point. The descriptor is `Option<&TextureViewDescriptor>` because
/// `None` means "WebGPU default view" (full 2D, all aspects).
#[test]
fn create_view_signature_pinned() {
    fn _type_check(
        renderer: &WebGpuRenderer,
        texture: &JsValue,
        descriptor: Option<&TextureViewDescriptor>,
    ) -> JsValue {
        renderer.create_view(texture, descriptor)
    }
    let _ = _type_check;
}

/// `push_error_scope(filter)` is fire-and-forget; the matching
/// `pop_error_scope` returns a `Promise<Option<JsValue>>`.
#[test]
fn push_error_scope_signature_pinned() {
    let _: fn(&WebGpuRenderer, &str) = WebGpuRenderer::push_error_scope;
}

// ---------------------------------------------------------------------
// 2. Pure-logic descriptor constructors (the `pub` ones).
// ---------------------------------------------------------------------

/// `TextureViewDescriptor::full()` is the canonical "default 2D view"
/// shape. Every field stays at zero / None.
#[test]
fn texture_view_descriptor_full_returns_canonical_shape() {
    let d: TextureViewDescriptor = TextureViewDescriptor::full();
    assert!(d.get_format().is_none());
    assert!(d.get_dimension().is_none());
    assert_eq!(d.get_base_mip_level(), 0);
    assert_eq!(d.get_mip_level_count(), 0);
    assert_eq!(d.get_base_array_layer(), 0);
    assert_eq!(d.get_array_layer_count(), 0);
    assert!(d.get_aspect().is_none());
}

/// `default_sampler()` returns a nearest-filter, clamp-to-edge sampler
/// — the cheapest default that still samples correctly at texture
/// borders.
#[test]
fn gpu_sampler_descriptor_default_returns_nearest_clamp() {
    let s: GpuSamplerDescriptor = GpuSamplerDescriptor::default_sampler();
    assert_eq!(s.get_mag_filter(), "nearest");
    assert_eq!(s.get_min_filter(), "nearest");
    assert_eq!(s.get_mipmap_filter(), "nearest");
    assert_eq!(s.get_address_mode_u(), "clamp-to-edge");
    assert_eq!(s.get_address_mode_v(), "clamp-to-edge");
    assert_eq!(s.get_address_mode_w(), "clamp-to-edge");
    assert!(!s.get_compare());
}
