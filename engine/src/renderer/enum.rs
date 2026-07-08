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
