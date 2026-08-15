use super::*;

/// A 2D camera that defines the viewport into the game world.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Camera2D {
    /// The world-space position of the camera center.
    #[get(type(copy))]
    pub(crate) position: Vector2D,
    /// The zoom factor (1.0 = no zoom, 2.0 = 2x magnification).
    #[get(type(copy))]
    pub(crate) zoom: f64,
    /// The rotation angle in radians.
    #[get(type(copy))]
    pub(crate) rotation: f64,
    /// The viewport width in screen pixels.
    #[get(type(copy))]
    pub(crate) viewport_width: f64,
    /// The viewport height in screen pixels.
    #[get(type(copy))]
    pub(crate) viewport_height: f64,
}

/// A 3D camera that defines the viewport into a 3D world using perspective
/// or orthographic projection.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Camera3D {
    /// The world-space position of the camera (eye).
    #[get(type(copy))]
    pub(crate) position: Vector3D,
    /// The point the camera is looking at (target).
    #[get(type(copy))]
    pub(crate) target: Vector3D,
    /// The up direction for the camera.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) up: Vector3D,
    /// The vertical field of view in radians.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) fov: f64,
    /// The near clipping plane distance.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) near: f64,
    /// The far clipping plane distance.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) far: f64,
    /// The viewport width in pixels.
    #[get(type(copy))]
    pub(crate) viewport_width: f64,
    /// The viewport height in pixels.
    #[get(type(copy))]
    pub(crate) viewport_height: f64,
}

/// A wrapper around `CanvasRenderingContext2d` providing convenience
/// drawing methods and camera management for the game engine.
#[derive(Clone, Data, New)]
pub struct CanvasRenderer {
    /// The underlying canvas 2D rendering context.
    pub(crate) context: CanvasRenderingContext2d,
    /// The active camera controlling the viewport.
    #[get(type(copy))]
    pub(crate) camera: Camera2D,
    /// The active rendering quality preset.
    ///
    /// Controls `imageSmoothingEnabled`, `imageSmoothingQuality`, and
    /// `textRendering` on the underlying context. Defaults to `Medium`.
    #[get(type(copy))]
    pub(crate) quality: RenderQuality,
}

/// A linear gradient defined by two endpoints and a list of color stops.
///
/// Used to create smooth color transitions along a straight line
/// for fill or stroke operations on the canvas.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct LinearGradient {
    /// The starting point of the gradient in world space.
    #[get(type(copy))]
    pub(crate) start: Vector2D,
    /// The ending point of the gradient in world space.
    #[get(type(copy))]
    pub(crate) end: Vector2D,
    /// The ordered list of color stops, each containing a position (0.0 to 1.0) and a CSS color string.
    pub(crate) stops: Vec<(f64, String)>,
}

/// A radial gradient defined by inner and outer circles and a list of color stops.
///
/// Used to create smooth color transitions radiating outward from a center point
/// for fill or stroke operations on the canvas.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct RadialGradient {
    /// The center of the inner circle of the gradient.
    #[get(type(copy))]
    pub(crate) inner_center: Vector2D,
    /// The radius of the inner circle.
    #[get(type(copy))]
    pub(crate) inner_radius: f64,
    /// The center of the outer circle of the gradient.
    #[get(type(copy))]
    pub(crate) outer_center: Vector2D,
    /// The radius of the outer circle.
    #[get(type(copy))]
    pub(crate) outer_radius: f64,
    /// The ordered list of color stops, each containing a position (0.0 to 1.0) and a CSS color string.
    pub(crate) stops: Vec<(f64, String)>,
}

/// Shadow rendering configuration for drop shadow effects on canvas primitives.
///
/// When applied, all subsequent fill, stroke, and draw operations will cast
/// a shadow with the specified color, blur radius, and offset.
#[derive(Clone, Data, Debug, New, PartialEq, PartialOrd)]
pub struct ShadowConfig {
    /// The CSS color string of the shadow (e.g., `"rgba(0,0,0,0.5)"`).
    #[get(type(clone))]
    pub(crate) color: String,
    /// The blur radius of the shadow in pixels.
    #[get(type(copy))]
    pub(crate) blur: f64,
    /// The horizontal offset of the shadow in pixels.
    #[get(type(copy))]
    pub(crate) offset_x: f64,
    /// The vertical offset of the shadow in pixels.
    #[get(type(copy))]
    pub(crate) offset_y: f64,
}

/// Represents the rendering priority layer for draw call ordering.
///
/// Higher z-index values are drawn on top of lower values,
/// enabling correct visual layering of game objects.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct RenderLayer {
    /// The z-index determining draw order. Higher values draw later (on top).
    #[get(type(copy))]
    pub(crate) z_index: i32,
    /// Whether objects in this layer should be rendered.
    #[get(type(copy))]
    pub(crate) visible: bool,
}

/// An ordered buffer of deferred draw commands recorded during a frame.
///
/// Scenes and components push `DrawCommand`s into the list during `on_render`
/// instead of drawing immediately. The engine then replays the whole list once
/// per frame via `CanvasRenderer::replay`, which batches consecutive same-style
/// shapes into a single path and skips redundant canvas state changes. The
/// backing `Vec` is reused across frames via `clear()` to avoid reallocation.
#[derive(Clone, Data, Debug, Default, New)]
pub struct DrawList {
    /// The recorded draw commands for the current frame.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) commands: Vec<DrawCommand>,
}

/// A supersampling anti-aliasing (SSAA) canvas wrapper that renders at a higher
/// resolution on an offscreen canvas and downscales to the display canvas for
/// smoother polygon edges in software-rendered 3D scenes.
///
/// The offscreen context is scaled by `scale_factor` so that all drawing
/// code can use logical pixel coordinates without modification. After
/// rendering, call `present()` to draw the high-resolution buffer onto the
/// visible canvas with high-quality image smoothing.
#[derive(Clone, Data, New)]
pub struct SsaaCanvas {
    /// The display canvas element visible to the user.
    pub(crate) display_canvas: HtmlCanvasElement,
    /// The 2D rendering context of the display canvas used for final presentation.
    pub(crate) display_context: CanvasRenderingContext2d,
    /// The offscreen canvas used for high-resolution rendering.
    pub(crate) offscreen_canvas: HtmlCanvasElement,
    /// The 2D rendering context of the offscreen canvas, pre-scaled by `scale_factor`.
    pub(crate) offscreen_context: CanvasRenderingContext2d,
    /// The supersampling scale factor (e.g., 2.0 means 4x SSAA).
    #[get(type(copy))]
    pub(crate) scale_factor: f64,
    /// The rendering quality preset for the downscaling present step.
    ///
    /// Controls the smoothing strategy when the offscreen buffer is
    /// downscaled onto the display canvas. Defaults to `Medium`.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) quality: RenderQuality,
    /// The logical display width in CSS pixels.
    #[get(type(copy))]
    pub(crate) width: f64,
    /// The logical display height in CSS pixels.
    #[get(type(copy))]
    pub(crate) height: f64,
}

/// A WebGPU rendering backend wrapping the GPU device, queue, and canvas context
/// for GPU-accelerated rendering on the web.
///
/// Created asynchronously via `WebGpuRenderer::init` because adapter and
/// device acquisition returns JavaScript Promises that must be awaited.
/// Once initialized, the renderer provides methods to create GPU resources
/// (buffers, shader modules, command encoders) and execute render passes.
///
/// WebGPU types are stored as `JsValue` to avoid feature-gated import issues
/// with `web_sys`. Method calls are performed via `Reflect` and `JsCast`.
#[derive(Clone, Getter, GetterMut, Setter)]
pub struct WebGpuRenderer {
    /// The WebGPU device (`GpuDevice`) used to create GPU resources.
    pub(crate) device: JsValue,
    /// The device's command queue (`GpuQueue`) for submitting command buffers.
    pub(crate) queue: JsValue,
    /// The WebGPU canvas rendering context (`GpuCanvasContext`).
    pub(crate) context: JsValue,
    /// The HTML canvas element backing the WebGPU context.
    pub(crate) canvas: HtmlCanvasElement,
    /// The texture format string used by the canvas's swap chain (e.g., `"bgra8unorm"`).
    #[get(type(clone))]
    pub(crate) format: String,
    /// The physical pixel width of the canvas backing store.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// The physical pixel height of the canvas backing store.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// Whether MSAA anti-aliasing is enabled for render pipelines.
    ///
    /// When `true`, the renderer allocates a multisampled intermediate texture
    /// (`sampleCount: 4`) and resolves into the swap chain each frame; when
    /// `false`, render passes attach directly to the swap chain view at
    /// `sampleCount: 1`.
    #[get(type(copy))]
    pub(crate) antialias: bool,
    /// The multisampled color texture used when `antialias` is `true`.
    ///
    /// `None` when MSAA is disabled. Rebuilt on every resize because the
    /// `width`/`height` are immutable for a given `GpuTexture`.
    #[get(type(clone))]
    pub(crate) multisample_texture: Option<JsValue>,
    /// The default `GpuTextureView` into `multisample_texture`.
    ///
    /// Cached at texture-create time so `begin_render_pass` does not have to
    /// recreate the view each frame. `None` when MSAA is disabled.
    #[get(type(clone))]
    pub(crate) multisample_view: Option<JsValue>,
    /// The depth-stencil texture used for depth-tested passes.
    ///
    /// Created lazily on the first call to [`WebGpuRenderer::begin_render_pass`]
    /// that includes a `depthStencil` attachment. Rebuilt on every resize
    /// because the dimensions are immutable for a given `GpuTexture`. The
    /// matching default view is cached in `depth_view`.
    ///
    /// `None` until the first depth-tested render pass is opened.
    #[get(type(clone))]
    pub(crate) depth_texture: Option<JsValue>,
    /// The default `GpuTextureView` into `depth_texture`.
    ///
    /// `None` when no depth texture has been allocated.
    #[get(type(clone))]
    pub(crate) depth_view: Option<JsValue>,
    /// The depth-stencil format used for `depth_texture`.
    ///
    /// Stored so subsequent render-pass openers can pass the same format
    /// to the pipeline layout without having to remember it externally.
    /// `None` until the first depth texture is allocated.
    #[get(type(clone))]
    pub(crate) depth_format: Option<String>,
    /// User-supplied closure fired when the underlying `GpuDevice` enters
    /// the `lost` state (browser-initiated context loss, OS driver crash,
    /// `device.destroy()`, ...).
    ///
    /// `None` until the caller calls [`WebGpuRenderer::on_device_lost`].
    /// The renderer also stores a separate `device_lost_handle` that
    /// forwards the `GPUDeviceLostInfo` JS value into this callback.
    #[get(type(clone))]
    pub(crate) device_lost_callback: Option<js_sys::Function>,
    /// Whether the device is currently in the `lost` state.
    ///
    /// Once flipped to `true`, every GPU operation returns
    /// `Err(WebGpuError::RendererDisposed)` until the caller destroys the
    /// renderer and creates a new one (WebGPU has no "recover from lost
    /// device" API).
    #[get(type(copy))]
    pub(crate) device_lost: bool,
    /// Shared slot for the most recent popped error-scope value.
    ///
    /// `device.popErrorScope()` returns a `Promise<GPUError?>`; we
    /// cannot `.await` it from a sync call site. Instead, every
    /// `push_error_scope` + `pop_error_scope` pair registers a
    /// microtask via `wasm_bindgen_futures::spawn_local` that stores
    /// the resolved value here. Callers that want the error
    /// synchronously call [`WebGpuRenderer::take_last_error`] to
    /// drain the slot.
    ///
    /// Holding a `Rc<RefCell<...>>` lets the spawn_local future own
    /// its own handle independently of `&self`, so the renderer's
    /// borrow checker stays happy. The slot is empty (`None`) by
    /// default and after each successful take.
    pub(crate) pending_error: std::rc::Rc<std::cell::RefCell<Option<JsValue>>>,
    /// The currently-open `GpuCommandEncoder`, if any.
    ///
    /// WebGPU expects the application to encode all work for a
    /// frame (clear, render passes, compute passes, copy ops) into
    /// a single command encoder, then call `encoder.finish()` to
    /// produce a `GpuCommandBuffer` and submit it to the queue.
    /// The encoder is `None` after `submit()` finishes and must
    /// be re-acquired via `device.createCommandEncoder()` before
    /// the next frame.
    #[get(type(clone))]
    pub(crate) command_encoder: Option<JsValue>,
}

/// A WebGL 2 rendering backend wrapping the `WebGl2RenderingContext`.
///
/// Unlike `WebGpuRenderer`, which stores all GPU handles as opaque `JsValue`s,
/// WebGL exposes concrete `web_sys` types, so the context and canvas are kept
/// as strongly typed values. Shader programs created via
/// [`WebGlRenderer::create_program`] are managed by the caller.
///
/// Construct via [`WebGlRenderer::init`], which resolves the canvas from the
/// [`RenderConfig`], applies device-pixel-ratio scaling to the backing store,
/// and acquires the `webgl2` context.
#[derive(Clone, Data)]
pub struct WebGlRenderer {
    /// The WebGL 2 rendering context used for all GL calls.
    pub(crate) context: WebGl2RenderingContext,
    /// The HTML canvas element backing the WebGL context.
    pub(crate) canvas: HtmlCanvasElement,
    /// The physical pixel width of the canvas backing store.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// The physical pixel height of the canvas backing store.
    #[get(type(copy))]
    pub(crate) height: u32,
}

// =====================================================================
// WebGPU: descriptor & data structs (consumed by the WebGpuRenderer API)
// =====================================================================

/// A single vertex attribute within a vertex buffer layout.
///
/// Mirrors the fields of `GPUVertexAttribute` exactly. The shader location
/// is the `@location(N)` qualifier in the WGSL source. The offset is in
/// bytes from the start of the vertex, and `format` is one of the
/// WGSL vertex format strings (e.g. `"float32x4"`, `"unorm8x4"`).
#[derive(Clone, Copy, Debug, New, PartialEq, Eq, Hash, Getter)]
pub struct VertexAttribute {
    /// The shader location the attribute maps to.
    #[get(type(copy))]
    pub(crate) shader_location: u32,
    /// The byte offset from the start of the vertex.
    #[get(type(copy))]
    pub(crate) offset: u64,
    /// The WGSL vertex format (e.g. `"float32x4"`).
    #[get(type(clone))]
    pub(crate) format: &'static str,
}

/// The layout of a single vertex buffer, expressed as an array stride plus
/// a list of attributes.
///
/// Mirrors `GPUVertexBufferLayout` from the WebGPU spec. The renderer
/// passes the assembled descriptor straight to `createRenderPipeline` via
/// `Reflect`.
#[derive(Clone, Debug, New, Getter)]
pub struct VertexBufferLayout {
    /// The byte stride of one vertex in the buffer.
    #[get(type(copy))]
    pub(crate) array_stride: u64,
    /// Whether the buffer should be advanced per-instance (`true`) or
    /// per-vertex (`false`).
    #[get(type(copy))]
    pub(crate) step_mode: VertexStepMode,
    /// The attributes that describe how to interpret the bytes of one
    /// vertex.
    pub(crate) attributes: Vec<VertexAttribute>,
}

/// Whether a vertex buffer is consumed per-vertex or per-instance.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum VertexStepMode {
    /// Advance the buffer one vertex at a time.
    #[default]
    Vertex,
    /// Advance the buffer one entry at a time, for all vertices of an
    /// instance.
    Instance,
}

impl VertexStepMode {
    /// Returns the WGSL / WebGPU string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertex => "vertex",
            Self::Instance => "instance",
        }
    }
}

/// A single binding entry inside a `BindGroupDescriptor`.
#[derive(Clone, Debug)]
pub enum BindGroupEntry {
    /// A uniform/storage buffer binding.
    Buffer {
        /// The binding slot (matches `@binding(N)` in the shader).
        binding: u32,
        /// The `GpuBuffer` handle.
        buffer: JsValue,
        /// The byte offset into the buffer where the binding starts.
        offset: u64,
        /// The size in bytes of the binding. `None` means "until the end
        /// of the buffer".
        size: Option<u64>,
    },
    /// A sampled texture binding.
    Texture {
        /// The binding slot.
        binding: u32,
        /// The `GpuTextureView` handle.
        view: JsValue,
    },
    /// A sampler binding.
    Sampler {
        /// The binding slot.
        binding: u32,
        /// The `GpuSampler` handle.
        sampler: JsValue,
    },
}

impl BindGroupEntry {
    /// Returns the `@binding(N)` slot this entry occupies. The renderer
    /// uses this when assembling the bind-group descriptor so the
    /// caller does not need to know the JS-side `binding` field name.
    pub(crate) fn binding(&self) -> u32 {
        match self {
            Self::Buffer { binding, .. }
            | Self::Texture { binding, .. }
            | Self::Sampler { binding, .. } => *binding,
        }
    }
}

/// A 2D texture descriptor for `create_texture_2d`.
///
/// Defaults produce a 1x1 RGBA8 texture with `TEXTURE_BINDING | COPY_DST
/// | COPY_SRC` usage, which is the right baseline for a sampled color
/// texture that is uploaded to via `queue.writeTexture`. Override fields
/// after constructing to set `mip_level_count`, `sample_count`, or
/// different `usage` flags.
#[derive(Clone, Debug, New, Getter)]
pub struct Texture2DDescriptor {
    /// The texture width in pixels. Must be > 0.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// The texture height in pixels. Must be > 0.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// The WGSL texture format (e.g. `"rgba8unorm"`, `"bgra8unorm"`,
    /// `"rgba16float"`, `"depth24plus-stencil8"`).
    #[get(type(clone))]
    pub(crate) format: &'static str,
    /// The number of mip levels. `0` is treated as `1`.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) mip_level_count: u32,
    /// The number of samples per texel (`1` for non-MSAA, `4` for MSAA).
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) sample_count: u32,
    /// The WGSL usage flags (e.g. `"RENDER_ATTACHMENT | TEXTURE_BINDING |
    /// COPY_DST | COPY_SRC"`).
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) usage: &'static str,
}

impl Texture2DDescriptor {
    /// Returns a descriptor with the most common defaults applied.
    ///
    /// This is the same as calling the generated `new` constructor and
    /// then explicitly setting the defaults; we provide it so callers
    /// can do `Texture2DDescriptor::default_for(w, h, format)` instead of
    /// having to remember which fields to set.
    ///
    /// # Arguments
    ///
    /// - `width` - The texture width in pixels.
    /// - `height` - The texture height in pixels.
    /// - `format` - The WGSL texture format.
    ///
    /// # Returns
    ///
    /// - A new descriptor with `mip_level_count = 1`, `sample_count = 1`,
    ///   and usage `"TEXTURE_BINDING | COPY_DST | COPY_SRC"`.
    pub fn default_for(width: u32, height: u32, format: &'static str) -> Self {
        Self {
            width,
            height,
            format,
            mip_level_count: 1,
            sample_count: 1,
            usage: "TEXTURE_BINDING | COPY_DST | COPY_SRC",
        }
    }
}

/// A sampler descriptor for `create_sampler`.
///
/// Defaults produce a non-filtering clamp-to-edge sampler. Override
/// fields after constructing to enable linear filtering, repeat
/// addressing, or depth comparison.
#[derive(Clone, Debug, New, Getter)]
pub struct GpuSamplerDescriptor {
    /// Minification filter.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) mag_filter: &'static str,
    /// Magnification filter.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) min_filter: &'static str,
    /// Mipmap filter.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) mipmap_filter: &'static str,
    /// U address mode.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) address_mode_u: &'static str,
    /// V address mode.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) address_mode_v: &'static str,
    /// W address mode.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) address_mode_w: &'static str,
    /// Whether the sampler is a comparison sampler.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) compare: bool,
}

impl GpuSamplerDescriptor {
    /// Returns a descriptor with the most common defaults applied:
    /// nearest filtering and clamp-to-edge addressing on all axes.
    pub fn default_sampler() -> Self {
        Self {
            mag_filter: WEBGPU_FILTER_MODE_NEAREST,
            min_filter: WEBGPU_FILTER_MODE_NEAREST,
            mipmap_filter: WEBGPU_FILTER_MODE_NEAREST,
            address_mode_u: WEBGPU_ADDRESS_MODE_CLAMP_TO_EDGE,
            address_mode_v: WEBGPU_ADDRESS_MODE_CLAMP_TO_EDGE,
            address_mode_w: WEBGPU_ADDRESS_MODE_CLAMP_TO_EDGE,
            compare: false,
        }
    }
}

/// The descriptor for a single (color or depth-stencil) render pass
/// attachment, used as input to `begin_render_pass` / `begin_render_pass_to_texture`.
#[derive(Clone, Debug)]
pub struct RenderPassColorAttachment {
    /// The texture view to draw into.
    ///
    /// When `None`, the renderer uses the swap-chain view (or the MSAA
    /// intermediate view if `antialias == true`).
    pub(crate) view: Option<JsValue>,
    /// An optional resolve target for MSAA.
    ///
    /// `None` when MSAA is disabled. The renderer fills in the default
    /// resolve target (the swap-chain view) when MSAA is enabled and the
    /// caller leaves this as `None`.
    pub(crate) resolve_target: Option<JsValue>,
    /// The clear color as `(r, g, b, a)` in `0.0..=1.0`. `None` means
    /// `"load"` (keep the previous contents).
    pub(crate) clear_value: Option<(f64, f64, f64, f64)>,
    /// The load operation. `None` → `"clear"` when `clear_value` is
    /// `Some`, otherwise `"load"`.
    pub(crate) load_op: Option<&'static str>,
    /// The store operation. `None` → `"store"`.
    pub(crate) store_op: Option<&'static str>,
}

impl RenderPassColorAttachment {
    /// Returns the load op that the renderer should use.
    pub(crate) fn effective_load_op(&self) -> &'static str {
        match (self.load_op, self.clear_value) {
            (Some(op), _) => op,
            (None, Some(_)) => WEBGPU_LOAD_OP_CLEAR,
            (None, None) => WEBGPU_LOAD_OP_LOAD,
        }
    }

    /// Returns the store op that the renderer should use.
    pub(crate) fn effective_store_op(&self) -> &'static str {
        self.store_op.unwrap_or(WEBGPU_STORE_OP_STORE)
    }
}

/// The depth-stencil portion of a `RenderPassDescriptor`, used as input to
/// `begin_render_pass` / `begin_render_pass_to_texture`.
#[derive(Clone, Debug)]
pub struct RenderPassDepthStencilAttachment {
    /// The depth-stencil texture view to use.
    ///
    /// When `None`, the renderer uses the default view into its
    /// `depth_texture` field, allocating the depth texture lazily if
    /// needed.
    pub(crate) view: Option<JsValue>,
    /// The depth clear value in `0.0..=1.0`. `None` means
    /// `"load"` (keep previous depth).
    pub(crate) depth_clear_value: Option<f32>,
    /// The depth load op. `None` → `"clear"` when
    /// `depth_clear_value` is `Some`, otherwise `"load"`.
    pub(crate) depth_load_op: Option<&'static str>,
    /// The depth store op. `None` → `"store"`.
    pub(crate) depth_store_op: Option<&'static str>,
    /// Whether depth reads should be enabled. `None` → `false`.
    pub(crate) depth_read_only: Option<bool>,
}

impl RenderPassDepthStencilAttachment {
    /// Returns the depth load op that the renderer should use.
    pub(crate) fn effective_depth_load_op(&self) -> &'static str {
        match (self.depth_load_op, self.depth_clear_value) {
            (Some(op), _) => op,
            (None, Some(_)) => WEBGPU_LOAD_OP_CLEAR,
            (None, None) => WEBGPU_LOAD_OP_LOAD,
        }
    }

    /// Returns the depth store op that the renderer should use.
    pub(crate) fn effective_depth_store_op(&self) -> &'static str {
        self.depth_store_op.unwrap_or(WEBGPU_STORE_OP_STORE)
    }
}

/// Descriptor for `GpuTexture.createView(descriptor)`.
///
/// Sub-selects a single cube face / mip / array slice / depth-aspect of a
/// texture. When you need the full texture as a 2D view (the common case),
/// just call `create_view` without a descriptor; the new method accepts an
/// `Option<&TextureViewDescriptor>` for callers that need the full
/// flexibility of the WebGPU spec.
#[derive(Clone, Debug, New, Getter)]
pub struct TextureViewDescriptor {
    /// View format override, or `None` to use the texture's own format.
    #[get(type(clone))]
    #[new(value = "None")]
    pub(crate) format: Option<&'static str>,
    /// View dimension (`"2d"`, `"2d-array"`, `"cube"`, `"cube-array"`, ...).
    /// `None` means the dimension is inferred from the texture.
    #[get(type(clone))]
    #[new(value = "None")]
    pub(crate) dimension: Option<&'static str>,
    /// Most significant mip level (inclusive). `None` → `0`.
    #[get(type(copy))]
    #[new(value = "0")]
    pub(crate) base_mip_level: u32,
    /// Number of mip levels in the view. `0` → all the way to the top.
    #[get(type(copy))]
    #[new(value = "0")]
    pub(crate) mip_level_count: u32,
    /// First array layer (inclusive). `None` → `0`. Only meaningful for
    /// `2d-array` / `cube` / `cube-array` views.
    #[get(type(copy))]
    #[new(value = "0")]
    pub(crate) base_array_layer: u32,
    /// Number of array layers. `0` → all remaining layers.
    #[get(type(copy))]
    #[new(value = "0")]
    pub(crate) array_layer_count: u32,
    /// Which aspect of the texture to expose. One of:
    /// `"all"`, `"depth-only"`, `"stencil-only"`. `None` → `"all"`.
    #[get(type(clone))]
    #[new(value = "None")]
    pub(crate) aspect: Option<&'static str>,
}

impl TextureViewDescriptor {
    /// Returns a descriptor that selects the full texture as a 2D view.
    /// This is the cheapest view you can make; equivalent to calling
    /// `texture.createView()` with no argument.
    pub fn full() -> Self {
        Self {
            format: None,
            dimension: None,
            base_mip_level: 0,
            mip_level_count: 0,
            base_array_layer: 0,
            array_layer_count: 0,
            aspect: None,
        }
    }

    /// The dimension string the renderer will send to `createView`.
    ///
    /// We default `None` to `"2d"` instead of omitting the key, because
    /// every other descriptor in the engine uses the explicit-string
    /// form, and a few browsers reject `dimension: undefined`.
    pub(crate) fn effective_dimension(&self) -> &'static str {
        self.dimension.unwrap_or(WEBGPU_TEXTURE_VIEW_DIMENSION_2D)
    }

    /// The aspect string the renderer will send to `createView`.
    ///
    /// Defaults to `"all"`, which is the spec's "expose every channel"
    /// option and the only correct choice for color textures.
    pub(crate) fn effective_aspect(&self) -> &'static str {
        self.aspect.unwrap_or(WEBGPU_TEXTURE_ASPECT_ALL)
    }

    /// Returns a descriptor that selects a single mip level of the texture.
    /// Useful when you want to read back a specific mip (e.g. the half-res
    /// blur output of a downsampling pass) without exposing the rest.
    pub fn mip(level: u32) -> Self {
        Self {
            format: None,
            dimension: None,
            base_mip_level: level,
            mip_level_count: 1,
            base_array_layer: 0,
            array_layer_count: 0,
            aspect: None,
        }
    }

    /// Returns a descriptor that selects the depth-only aspect of a
    /// depth-stencil texture. Required when sampling depth in a shader
    /// (`textureSample(t, s, uv)` where `t` is a depth texture).
    pub fn depth_only() -> Self {
        Self {
            format: None,
            dimension: None,
            base_mip_level: 0,
            mip_level_count: 0,
            base_array_layer: 0,
            array_layer_count: 0,
            aspect: Some(WEBGPU_TEXTURE_ASPECT_DEPTH_ONLY),
        }
    }
}

/// Descriptor for `queue.writeTexture(destination, data, dataLayout, size)`.
///
/// WebGPU's `writeTexture` lets you upload CPU-side pixel data directly to a
/// texture without staging through a buffer. Use it for: ImGui font atlases,
/// procedural noise textures, sprite sheets, `ImageBitmap` pixels, etc.
#[derive(Clone, Debug, New, Getter)]
pub struct TextureWriteDescriptor {
    /// The pixel data to upload. Bytes are laid out according to
    /// `bytes_per_row` / `rows_per_image`.
    #[get(type(clone))]
    pub(crate) data: Vec<u8>,
    /// Bytes per row of the source data. Must be a multiple of 256.
    #[get(type(copy))]
    pub(crate) bytes_per_row: u32,
    /// Number of rows per image. `0` for 2D textures without mip chains.
    #[get(type(copy))]
    pub(crate) rows_per_image: u32,
    /// Destination mip level to write into.
    #[get(type(copy))]
    pub(crate) mip_level: u32,
    /// Destination texture to write into.
    #[get(type(clone))]
    pub(crate) texture: JsValue,
    /// Origin within the destination texture. `None` → `(0, 0, 0)`.
    #[get(type(clone))]
    #[new(value = "None")]
    pub(crate) origin: Option<JsValue>,
    /// Whether to flip the source data vertically before writing.
    /// `true` is essential when uploading from `<img>` / `<canvas>` whose
    /// rows are top-to-bottom but WebGPU textures are bottom-to-top.
    #[get(type(copy))]
    #[new(value = "false")]
    pub(crate) flip_y: bool,
}

impl TextureWriteDescriptor {
    /// Convenience constructor for the common 2D upload case.
    ///
    /// - `data`: packed pixel bytes (format-dependent).
    /// - `bytes_per_row`: row stride of `data`, must be a multiple of 256.
    /// - `texture`: the destination `GpuTexture` handle.
    pub fn for_2d(data: Vec<u8>, bytes_per_row: u32, texture: JsValue) -> Self {
        Self {
            data,
            bytes_per_row,
            rows_per_image: 0,
            mip_level: 0,
            texture,
            origin: None,
            flip_y: false,
        }
    }
}

