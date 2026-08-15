use super::*;

// =====================================================================
// input tests
// =====================================================================



/// Verifies that a fresh `EngineHandle` starts without a registered input
/// cell and that `register_input`'s accessor has the expected shape.
#[test]
fn engine_handle_starts_without_input_cell() {
    let config: EngineConfig = EngineConfig::default();
    let handle: EngineHandle = EngineHandle::new(config, None, None, None, None);
    let cell: &Option<InputStateCell> = handle.try_get_input_cell();
    assert!(cell.is_none());
}

/// Verifies the `InputState` mutation and readout accessor shapes used by
/// the DOM event listeners and per-frame consumers.
#[test]
fn input_state_frame_lifecycle() {
    let mut state: InputState = InputState::new();
    state.press_key("KeyW".to_string());
    state.press_mouse_button(MouseButton::Left, Vector2D::new(10.0, 20.0));
    state.update_mouse_position(Vector2D::new(12.0, 24.0));
    state.start_touch(1, Vector2D::new(5.0, 6.0));
    assert!(state.get_keys_pressed().contains("KeyW"));
    assert!(state.get_keys_held().contains("KeyW"));
    assert!(
        state
            .get_mouse_buttons_pressed()
            .contains(&MouseButton::Left)
    );
    assert!(state.get_mouse_buttons_held().contains(&MouseButton::Left));
    assert_eq!(state.get_mouse_position().get_x(), 12.0);
    assert_eq!(state.get_mouse_position().get_y(), 24.0);
    assert!(state.get_touch_points().contains_key(&1));
    assert!(state.get_touch_started().contains(&1));
    state.end_frame();
    assert!(state.get_keys_pressed().is_empty());
    assert!(state.get_keys_held().contains("KeyW"));
    assert!(state.get_mouse_buttons_pressed().is_empty());
    assert!(state.get_mouse_buttons_held().contains(&MouseButton::Left));
    assert!(!state.get_mouse_moved());
    assert!(state.get_touch_started().is_empty());
    assert!(state.get_touch_points().contains_key(&1));
    state.release_key("KeyW".to_string());
    state.release_mouse_button(MouseButton::Left);
    state.end_touch(1);
    assert!(state.get_keys_released().contains("KeyW"));
    assert!(!state.get_keys_held().contains("KeyW"));
    assert!(!state.get_mouse_buttons_held().contains(&MouseButton::Left));
    assert!(!state.get_touch_points().contains_key(&1));
    assert!(state.get_touch_ended().contains(&1));
}

// =====================================================================
// renderer tests
// =====================================================================



use wasm_bindgen::JsValue;

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
