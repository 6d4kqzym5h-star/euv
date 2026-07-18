use crate::*;

/// Defines how new pixels are composited with existing pixels on the canvas.
///
/// Maps directly to the CSS `globalCompositeOperation` property.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BlendMode {
    /// The source is drawn over the destination (default alpha blending).
    #[default]
    Normal,
    /// The source color is multiplied with the destination, producing a darker result.
    Multiply,
    /// The source and destination are inverted, multiplied, then inverted again.
    Screen,
    /// The source and destination colors are added together, clamped to maximum brightness.
    Lighter,
    /// Combines `Multiply` and `Screen` based on the destination color.
    Overlay,
    /// Keeps the darker of the source and destination per channel.
    Darken,
    /// Keeps the lighter of the source and destination per channel.
    Lighten,
    /// Dodges the destination color brightening it based on the source.
    ColorDodge,
    /// Burns the destination color darkening it based on the source.
    ColorBurn,
    /// A harsher version of `Overlay` using the source color as the filter.
    HardLight,
    /// A softer version of `Overlay` using the source color as the filter.
    SoftLight,
    /// Subtracts the darker color from the lighter color per channel.
    Difference,
    /// Similar to `Difference` but with lower contrast.
    Exclusion,
    /// Uses the hue of the source with the saturation and luminosity of the destination.
    Hue,
    /// Uses the saturation of the source with the hue and luminosity of the destination.
    Saturation,
    /// Uses the hue and saturation of the source with the luminosity of the destination.
    Color,
    /// Uses the luminosity of the source with the hue and saturation of the destination.
    Luminosity,
}

/// Rendering quality preset controlling anti-aliasing smoothing strategy.
///
/// Maps to the canvas `imageSmoothingQuality` value plus an explicit
/// `imageSmoothingEnabled` toggle. Combined with a CSS `image-rendering:
/// pixelated` rule on the consumer side, `Low` produces crisp pixel-art
/// rendering while `High` produces smooth vector-style rendering.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum RenderQuality {
    /// Fastest rendering, pixelated scaling.
    ///
    /// Disables `imageSmoothingEnabled` on the canvas context and sets
    /// `imageSmoothingQuality = "low"`. Pair with CSS `image-rendering:
    /// pixelated` for sharp nearest-neighbour scaling.
    Low,
    /// Balanced rendering with default smoothing quality.
    ///
    /// Sets `imageSmoothingQuality = "medium"`.
    Medium,
    /// Highest fidelity rendering with smooth edges and high-quality scaling.
    ///
    /// Sets `imageSmoothingQuality = "high"`. Best for vector-style content
    /// on HiDPI displays. This is the default — when no explicit quality is
    /// requested, the engine errs on the side of visual fidelity rather than
    /// performance, since users typically notice aliasing artifacts before
    /// they notice a few extra milliseconds of GPU time.
    #[default]
    High,
}

/// Errors that can occur while asynchronously initializing a `WebGpuRenderer`.
///
/// Each variant maps to one specific failure mode that the WebGPU init
/// pipeline can encounter when calling into the browser's GPU API. The
/// underlying JS error (when available) is carried as a `JsValue` so callers
/// can surface the exact diagnostic string without losing fidelity.
///
/// Instead of logging diagnostics inside the engine, `WebGpuRenderer::init`
/// returns `Result<WebGpuRenderer, WebGpuInitError>` and lets the caller
/// decide how to react — typically via `Console::error` on the example side
/// or by falling back to the Canvas 2D backend.
#[derive(Clone, Debug)]
pub enum WebGpuInitError {
    /// `Reflect::get(navigator, "webgpu")` threw an exception.
    ///
    /// Surfaced when the JavaScript binding lookup itself fails rather than
    /// simply returning `undefined`/`null`. Carries the original JS error.
    NavigatorLookup(JsValue),
    /// `navigator.gpu` is `undefined` or `null`.
    ///
    /// The browser does not expose WebGPU on the current origin. The most
    /// common causes are serving over an insecure origin (must be HTTPS or
    /// `localhost`) or running in a browser that lacks the WebGPU feature.
    NavigatorGpuMissing,
    /// `Reflect::get(gpu, "requestAdapter")` threw an exception.
    ///
    /// Carries the original JS error returned by the reflect call.
    RequestAdapterLookup(JsValue),
    /// `gpu.requestAdapter()` threw an exception synchronously.
    ///
    /// Carries the thrown JS error or value.
    RequestAdapterCall(JsValue),
    /// The adapter promise rejected, or the `INIT_PROMISE_TIMEOUT_MILLIS`
    /// race timer fired before the adapter was produced.
    ///
    /// Carries the rejection value, which may be a string, an error object,
    /// or `undefined` when the timeout won the race.
    AdapterPromise(JsValue),
    /// `requestAdapter()` resolved to `null` or `undefined`.
    ///
    /// No compatible GPU adapter exists for the requested `powerPreference`.
    AdapterUnavailable,
    /// `Reflect::get(adapter, "requestDevice")` threw an exception.
    RequestDeviceLookup(JsValue),
    /// `adapter.requestDevice()` threw an exception synchronously.
    RequestDeviceCall(JsValue),
    /// The device promise rejected, or the `INIT_PROMISE_TIMEOUT_MILLIS`
    /// race timer fired before the device was produced.
    DevicePromise(JsValue),
    /// `requestDevice()` resolved to `null` or `undefined`.
    ///
    /// The adapter could not allocate a device, typically because the
    /// adapter is in a `device-lost` state.
    DeviceUnavailable,
    /// `document.querySelector(canvas_selector)` returned `None`.
    ///
    /// The canvas element is not in the DOM yet (or its selector is wrong).
    /// Carries the selector string that was queried.
    CanvasNotFound(String),
    /// `document.querySelector(canvas_selector)` threw an exception.
    CanvasQuery(JsValue),
    /// `canvas.get_context("webgpu")` returned `None`.
    ///
    /// The canvas is already using a different context type, or WebGPU is
    /// disabled for this canvas.
    CanvasContextUnavailable,
    /// `Reflect::get(gpu, "getPreferredCanvasFormat")` threw an exception.
    PreferredFormatLookup(JsValue),
    /// `gpu.getPreferredCanvasFormat()` threw an exception synchronously.
    PreferredFormatCall(JsValue),
    /// `getPreferredCanvasFormat()` resolved to a value that is not a string.
    ///
    /// Carries the offending JS value so callers can log its type/name.
    PreferredFormatType(JsValue),
    /// `Reflect::get(context, "configure")` threw an exception.
    ConfigureLookup(JsValue),
    /// `Reflect::get(device, "queue")` threw an exception.
    QueueLookup(JsValue),
}
