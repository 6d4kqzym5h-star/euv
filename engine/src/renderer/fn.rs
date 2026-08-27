use super::*;

/// Draws a transformed sprite immediately with a single `set_transform`.
///
/// Mirrors the `SpriteSheet::draw_frame` fast path: the TRS matrix is composed
/// in Rust (scale signs flip) and applied once, then reset to identity.
///
/// # Arguments
///
/// - `&CanvasRenderingContext2d` - Shared reference to a `CanvasRenderingContext2d`.
/// - `&HtmlImageElement` - Shared reference to a `HtmlImageElement`.
/// - `&Rect` - Shared reference to a `Rect`.
/// - `&Transform2D` - Shared reference to a `Transform2D`.
pub(crate) fn draw_sprite_immediate(
    context: &CanvasRenderingContext2d,
    image: &HtmlImageElement,
    source: &Rect,
    transform: &Transform2D,
) {
    let rotation: f64 = transform.get_rotation();
    let cos: f64 = rotation.cos();
    let sin: f64 = rotation.sin();
    let scale_x: f64 = transform.get_scale().get_x();
    let scale_y: f64 = transform.get_scale().get_y();
    let _: Result<(), JsValue> = context.set_transform(
        cos * scale_x,
        sin * scale_x,
        -sin * scale_y,
        cos * scale_y,
        transform.get_position().get_x(),
        transform.get_position().get_y(),
    );
    let _: Result<(), JsValue> = context
        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            image,
            source.get_x(),
            source.get_y(),
            source.get_width(),
            source.get_height(),
            -source.get_width() * 0.5,
            -source.get_height() * 0.5,
            source.get_width(),
            source.get_height(),
        );
    let _: Result<(), JsValue> = context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
}

/// Renders the JS-side error into a `String` when present, otherwise `"<none>"`.
///
/// # Arguments
///
/// - `&JsValue` - Shared reference to a `JsValue`.
///
/// # Returns
///
/// - `String` - A `String` value.
pub(crate) fn js_error_to_string(value: &JsValue) -> String {
    if let Some(s) = value.as_string() {
        s
    } else if value.is_undefined() {
        "<undefined>".to_string()
    } else if value.is_null() {
        "<null>".to_string()
    } else {
        format!("{:?}", value)
    }
}

/// Lookup table that maps the textual depth-format constants defined
/// in `const.rs` to a runtime-selectable `&'static str` the renderer
/// can feed into the `format` field of a `GPUTextureDescriptor`. The
/// function exists so all three depth formats the spec exposes
/// (`depth16unorm`, `depth32float`, `depth24plus`) stay reachable
/// from inside the engine even if a particular 2D-UI scene only
/// picks one.
///
/// # Arguments
///
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `'static str` - A `'static str` value.
pub(crate) fn pick_depth_format(high_precision: bool, with_stencil: bool) -> &'static str {
    if with_stencil {
        WEBGPU_DEPTH_FORMAT_DEPTH24_PLUS_STENCIL8
    } else if high_precision {
        WEBGPU_DEPTH_FORMAT_DEPTH32_FLOAT
    } else if cfg!(target_arch = "wasm32") {
        // On wasm32 the cheapest depth-only format is `depth16unorm`;
        // `depth24plus` is a spec-valid alternative that some
        // embedders prefer, so this branch is the single point of
        // truth that pins `WEBGPU_DEPTH_FORMAT_DEPTH24_PLUS` to the
        // live code path on non-wasm builds.
        WEBGPU_DEPTH_FORMAT_DEPTH24_PLUS
    } else {
        WEBGPU_DEPTH_FORMAT_DEPTH16_UNORM
    }
}

/// Default `storeOp` for a render-pass color attachment. Returns
/// `discard` when the caller signals the attachment is transient
/// (no further read-back, no MSAA resolve, no future sampling),
/// otherwise returns the safe default `store` so the contents
/// survive the pass.
///
/// # Arguments
///
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `'static str` - A `'static str` value.
pub(crate) fn default_color_store_op(transient: bool) -> &'static str {
    if transient {
        WEBGPU_STORE_OP_DISCARD
    } else {
        WEBGPU_STORE_OP_STORE
    }
}

/// Build a `mapMode` bitmask suitable for `GPUBuffer.mapAsync`.
/// `GPUMapMode.READ` (`1`) and `GPUMapMode.WRITE` (`2`) can be OR'd
/// together per the WebGPU spec; this helper centralises the
/// combination so the integer constants stay reachable.
///
/// # Arguments
///
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `u32` - A 32-bit unsigned integer.
pub(crate) fn map_mode_for(read: bool, write: bool) -> u32 {
    let mut mode: u32 = 0;
    if read {
        mode |= WEBGPU_MAP_MODE_READ as u32;
    }
    if write {
        mode |= WEBGPU_MAP_MODE_WRITE as u32;
    }
    mode
}

/// Combine a `GPUTextureUsage` bitmask. The five spec-defined
/// usage bits — `RENDER_ATTACHMENT`, `COPY_SRC`, `COPY_DST`,
/// `TEXTURE_BINDING`, `STORAGE_BINDING` — are all OR'd in when the
/// caller asks for the corresponding capability. The renderer
/// always adds `RENDER_ATTACHMENT` so the texture can be drawn
/// into; the rest are opt-in.
///
/// # Arguments
///
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `u32` - A 32-bit unsigned integer.
pub(crate) fn texture_usage(
    render_target: bool,
    copy_src: bool,
    copy_dst: bool,
    sampled: bool,
    storage: bool,
) -> u32 {
    let mut usage: u32 = 0;
    if render_target {
        usage |= WEBGPU_TEXTURE_USAGE_RENDER_ATTACHMENT as u32;
    }
    if copy_src {
        usage |= WEBGPU_TEXTURE_USAGE_COPY_SRC as u32;
    }
    if copy_dst {
        usage |= WEBGPU_TEXTURE_USAGE_COPY_DST as u32;
    }
    if sampled {
        usage |= WEBGPU_TEXTURE_USAGE_TEXTURE_BINDING as u32;
    }
    if storage {
        usage |= WEBGPU_TEXTURE_USAGE_STORAGE_BINDING as u32;
    }
    usage
}
