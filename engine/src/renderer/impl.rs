use super::*;

/// Implements camera transformation methods for `Camera2D`.
impl Camera2D {
    /// Creates a new camera centered at the origin with default zoom and no rotation.
    ///
    /// # Arguments
    ///
    /// - `f64` - The viewport width in pixels.
    /// - `f64` - The viewport height in pixels.
    ///
    /// # Returns
    ///
    /// - `Camera2D` - The new camera.
    pub fn create(viewport_width: f64, viewport_height: f64) -> Camera2D {
        Camera2D::new(
            Vector2D::zero(),
            RENDERER_DEFAULT_CAMERA_ZOOM,
            RENDERER_DEFAULT_CAMERA_ROTATION,
            viewport_width,
            viewport_height,
        )
    }

    /// Converts a world-space point to screen-space coordinates.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The world-space point.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The screen-space point.
    pub fn world_to_screen(&self, world: Vector2D) -> Vector2D {
        let relative: Vector2D = world - self.get_position();
        let rotated: Vector2D = relative.rotated(-self.get_rotation());
        Vector2D::new(
            rotated.get_x() * self.get_zoom() + self.get_viewport_width() * 0.5,
            rotated.get_y() * self.get_zoom() + self.get_viewport_height() * 0.5,
        )
    }

    /// Converts a screen-space point to world-space coordinates.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The screen-space point.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The world-space point.
    pub fn screen_to_world(&self, screen: Vector2D) -> Vector2D {
        let relative: Vector2D = Vector2D::new(
            (screen.get_x() - self.get_viewport_width() * 0.5) / self.get_zoom(),
            (screen.get_y() - self.get_viewport_height() * 0.5) / self.get_zoom(),
        );
        let rotated: Vector2D = relative.rotated(self.get_rotation());
        rotated + self.get_position()
    }

    /// Moves the camera position by the given offset.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The translation offset in world space.
    pub fn translate(&mut self, offset: Vector2D) {
        self.set_position(self.get_position() + offset);
    }

    /// Adjusts the zoom by the given factor, clamped to a minimum of `EPSILON`.
    ///
    /// # Arguments
    ///
    /// - `f64` - The zoom multiplier.
    pub fn zoom_by(&mut self, factor: f64) {
        self.set_zoom((self.get_zoom() * factor).max(EPSILON));
    }
}

/// Implements `Default` for `Camera2D` as a camera at the origin with 800x600 viewport.
impl Default for Camera2D {
    fn default() -> Camera2D {
        Camera2D::create(800.0, 600.0)
    }
}

/// Implements static font and color utility methods for `CanvasRenderer`.
impl CanvasRenderer {
    /// Builds a CSS font string from font size and family.
    ///
    /// # Arguments
    ///
    /// - `f64` - The font size in pixels.
    /// - `F: AsRef<str>` - The font family name.
    ///
    /// # Returns
    ///
    /// - `String` - The CSS font string (e.g., `"16px sans-serif"`).
    pub fn font<F>(size: f64, family: F) -> String
    where
        F: AsRef<str>,
    {
        let family: &str = family.as_ref();
        format!("{size}px {family}")
    }

    /// Creates a default font string using the default font size and family.
    ///
    /// # Returns
    ///
    /// - `String` - The default CSS font string.
    pub fn default_font() -> String {
        Self::font(RENDERER_DEFAULT_FONT_SIZE, RENDERER_DEFAULT_FONT_FAMILY)
    }

    /// Enables high-quality anti-aliasing on an arbitrary canvas 2D context.
    ///
    /// Applies the `High` rendering quality preset via `apply_quality`,
    /// which sets `imageSmoothingEnabled`, `imageSmoothingQuality = "high"`,
    /// and `textRendering = "geometricPrecision"` on the given context.
    ///
    /// Use this static helper when you manage your own `CanvasRenderingContext2d`
    /// and don't hold a `CanvasRenderer` instance. For instances, call
    /// `renderer.enable_smoothing()` instead.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas context to configure.
    pub fn enable_smoothing_on(context: &CanvasRenderingContext2d) {
        Self::apply_quality(context, RenderQuality::High);
    }

    /// Detects the host device pixel ratio (HiDPI scale factor) via reflection.
    ///
    /// Reads `window.devicePixelRatio` using `Reflect::get` because the
    /// `web-sys` `Window` features currently in use do not expose a native
    /// getter for this property. Falls back to
    /// `RENDERER_DEFAULT_DEVICE_PIXEL_RATIO` (1.0) when the value is missing,
    /// not a finite number, or below 1.0.
    ///
    /// # Returns
    ///
    /// - `f64` - The detected device pixel ratio (clamped to `>= 1.0`).
    pub fn detect_dpr() -> f64 {
        let window_value: Window = window().expect("no global window exists");
        let raw: Option<f64> = Reflect::get(
            window_value.as_ref(),
            &JsValue::from_str(RENDERER_PROPERTY_DEVICE_PIXEL_RATIO),
        )
        .ok()
        .and_then(|value: JsValue| value.as_f64());
        raw.filter(|value: &f64| value.is_finite() && *value >= 1.0)
            .unwrap_or(RENDERER_DEFAULT_DEVICE_PIXEL_RATIO)
    }

    /// Applies the given `RenderQuality` preset to an arbitrary canvas context.
    ///
    /// Sets `imageSmoothingEnabled`, `imageSmoothingQuality`, and
    /// `textRendering` according to the supplied quality. `Low` disables
    /// smoothing (intended for use with CSS `image-rendering: pixelated`),
    /// `Medium` and `High` enable it with the matching quality level.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The target context.
    /// - `RenderQuality` - The quality preset to apply.
    pub(crate) fn apply_quality(context: &CanvasRenderingContext2d, quality: RenderQuality) {
        let smoothing_enabled: bool = !matches!(quality, RenderQuality::Low);
        context.set_image_smoothing_enabled(smoothing_enabled);
        let quality_value: &str = match quality {
            RenderQuality::Low => RENDERER_IMAGE_SMOOTHING_QUALITY_LOW,
            RenderQuality::Medium => RENDERER_IMAGE_SMOOTHING_QUALITY_MEDIUM,
            RenderQuality::High => RENDERER_IMAGE_SMOOTHING_QUALITY_HIGH,
        };
        let _ = Reflect::set(
            context,
            &JsValue::from_str(RENDERER_PROPERTY_IMAGE_SMOOTHING_QUALITY),
            &JsValue::from_str(quality_value),
        );
        let _ = Reflect::set(
            context,
            &JsValue::from_str(RENDERER_PROPERTY_TEXT_RENDERING),
            &JsValue::from_str(RENDERER_TEXT_RENDERING_GEOMETRIC_PRECISION),
        );
    }
}

/// Implements static CSS conversion for `Color`.
impl Color {
    /// Converts a `Color` to a CSS `rgba()` string suitable for canvas fill or stroke styles.
    ///
    /// # Arguments
    ///
    /// - `&Color` - The color to convert.
    ///
    /// # Returns
    ///
    /// - `String` - The CSS `rgba()` color string.
    pub fn to_css(color: &Color) -> String {
        color.to_css_rgba()
    }
}

/// Returns the command slice of a `DrawList` for replay iteration.
fn self_commands(list: &DrawList) -> &[DrawCommand] {
    list.get_commands().as_slice()
}

/// Draws a transformed sprite immediately with a single `set_transform`.
///
/// Mirrors the `SpriteSheet::draw_frame` fast path: the TRS matrix is composed
/// in Rust (scale signs flip) and applied once, then reset to identity.
fn draw_sprite_immediate(
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
    let _ = context.set_transform(
        cos * scale_x,
        sin * scale_x,
        -sin * scale_y,
        cos * scale_y,
        transform.get_position().get_x(),
        transform.get_position().get_y(),
    );
    let _ = context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
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
    let _ = context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
}

/// Implements drawing and camera management methods for `CanvasRenderer`.
/// Implements recording and replay for `DrawList`.
impl DrawList {
    /// Creates an empty draw list.
    ///
    /// # Returns
    ///
    /// - `DrawList` - The new empty draw list.
    pub fn create() -> DrawList {
        DrawList::new(Vec::new())
    }

    /// Returns whether the list contains no commands.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if there are no recorded commands.
    pub fn is_empty(&self) -> bool {
        self.get_commands().is_empty()
    }

    /// Returns the number of recorded commands.
    ///
    /// # Returns
    ///
    /// - `usize` - The command count.
    pub fn len(&self) -> usize {
        self.get_commands().len()
    }

    /// Removes all recorded commands, keeping the allocated capacity for reuse
    /// on the next frame.
    pub fn clear(&mut self) {
        self.get_mut_commands().clear();
    }

    /// Records a fill-rectangle command.
    pub fn fill_rect(&mut self, position: Vector2D, width: f64, height: f64, color: Color) {
        self.get_mut_commands().push(DrawCommand::FillRect {
            position,
            width,
            height,
            color,
        });
    }

    /// Records a stroke-rectangle command.
    pub fn stroke_rect(
        &mut self,
        position: Vector2D,
        width: f64,
        height: f64,
        color: Color,
        line_width: f64,
    ) {
        self.get_mut_commands().push(DrawCommand::StrokeRect {
            position,
            width,
            height,
            color,
            line_width,
        });
    }

    /// Records a fill-circle command.
    pub fn fill_circle(&mut self, center: Vector2D, radius: f64, color: Color) {
        self.get_mut_commands().push(DrawCommand::FillCircle {
            center,
            radius,
            color,
        });
    }

    /// Records a stroke-circle command.
    pub fn stroke_circle(&mut self, center: Vector2D, radius: f64, color: Color, line_width: f64) {
        self.get_mut_commands().push(DrawCommand::StrokeCircle {
            center,
            radius,
            color,
            line_width,
        });
    }

    /// Records a line-segment command.
    pub fn draw_line(&mut self, start: Vector2D, end: Vector2D, color: Color, line_width: f64) {
        self.get_mut_commands().push(DrawCommand::Line {
            start,
            end,
            color,
            line_width,
        });
    }

    /// Records a fill-text command.
    pub fn fill_text<T, F>(&mut self, text: T, position: Vector2D, color: Color, font: F)
    where
        T: AsRef<str>,
        F: AsRef<str>,
    {
        self.get_mut_commands().push(DrawCommand::FillText {
            text: text.as_ref().to_string(),
            position,
            color,
            font: font.as_ref().to_string(),
        });
    }

    /// Records a transformed sprite draw command.
    pub fn draw_sprite(&mut self, image: &HtmlImageElement, source: Rect, transform: Transform2D) {
        self.get_mut_commands().push(DrawCommand::DrawSprite {
            image: image.clone(),
            source,
            transform,
        });
    }

    /// Records an image sub-region draw command (no rotation).
    pub fn draw_image_rect(
        &mut self,
        image: &HtmlImageElement,
        source: Rect,
        dest_position: Vector2D,
        dest_width: f64,
        dest_height: f64,
    ) {
        self.get_mut_commands().push(DrawCommand::DrawImageRect {
            image: image.clone(),
            source,
            dest_position,
            dest_width,
            dest_height,
        });
    }

    /// Records a global-alpha state change.
    pub fn set_global_alpha(&mut self, alpha: f64) {
        self.get_mut_commands()
            .push(DrawCommand::SetGlobalAlpha { alpha });
    }

    /// Records a blend-mode state change.
    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        self.get_mut_commands()
            .push(DrawCommand::SetBlendMode { mode });
    }
}

impl CanvasRenderer {
    /// Creates a new renderer from a canvas element selector and viewport dimensions.
    ///
    /// # Arguments
    ///
    /// - `&str` - The CSS selector for the canvas element.
    /// - `f64` - The viewport width.
    /// - `f64` - The viewport height.
    ///
    /// # Returns
    ///
    /// - `Option<CanvasRenderer>` - The renderer, or `None` if the canvas was not found.
    pub fn from_selector<S>(
        canvas_selector: S,
        viewport_width: f64,
        viewport_height: f64,
    ) -> Option<CanvasRenderer>
    where
        S: AsRef<str>,
    {
        let window_value: Window = window().expect("no global window exists");
        let document_value: Document = window_value.document().expect("should have a document");
        let element: Element = document_value
            .query_selector(canvas_selector.as_ref())
            .ok()
            .flatten()?;
        let canvas_element: HtmlCanvasElement = element.unchecked_into();
        let context_object: Object = canvas_element
            .get_context(RENDERER_CONTEXT_TYPE_2D)
            .ok()
            .flatten()?;
        let context: CanvasRenderingContext2d = context_object.unchecked_into();
        let renderer: CanvasRenderer = CanvasRenderer::new(
            context,
            Camera2D::create(viewport_width, viewport_height),
            RenderQuality::default(),
        );
        renderer.enable_smoothing();
        Some(renderer)
    }

    /// Enables high-quality anti-aliasing on the canvas context by setting
    /// `imageSmoothingEnabled` to `true` and `imageSmoothingQuality` to `"high"`.
    ///
    /// Applies the active `quality` preset via the shared `apply_quality`
    /// helper so that all smoothing-related settings are kept in sync.
    pub fn enable_smoothing(&self) {
        Self::apply_quality(self.get_context(), self.get_quality());
    }

    /// Clears the entire canvas viewport.
    pub fn clear(&self) {
        self.get_context().clear_rect(
            0.0,
            0.0,
            self.get_camera().get_viewport_width(),
            self.get_camera().get_viewport_height(),
        );
    }

    /// Clears the canvas and fills it with the given CSS color string.
    ///
    /// # Arguments
    ///
    /// - `C: AsRef<str>` - The CSS color string (e.g., `"#000000"`).
    pub fn clear_color<C>(&self, color: C)
    where
        C: AsRef<str>,
    {
        self.get_context().set_fill_style_str(color.as_ref());
        self.get_context().fill_rect(
            0.0,
            0.0,
            self.get_camera().get_viewport_width(),
            self.get_camera().get_viewport_height(),
        );
    }

    /// Saves the current canvas state (transform, styles) onto the state stack.
    pub fn save(&self) {
        self.get_context().save();
    }

    /// Restores the most recently saved canvas state.
    pub fn restore(&self) {
        self.get_context().restore();
    }

    /// Replays a recorded `DrawList` onto this renderer's canvas.
    ///
    /// Convenience wrapper around `replay_context` using this renderer's context.
    ///
    /// # Arguments
    ///
    /// - `&DrawList` - The recorded commands to replay.
    pub fn replay(&self, list: &DrawList) {
        Self::replay_context(self.get_context(), list);
    }

    /// Replays a recorded `DrawList` onto an arbitrary canvas 2D context in a
    /// single batched pass.
    ///
    /// Consecutive same-style shapes are merged into one path (one `begin_path`
    /// plus one `fill`/`stroke` per style run), fill/stroke colors and line
    /// widths are only re-applied when they change, and sprites are drawn with a
    /// single `set_transform` rather than a save/restore pair. This collapses
    /// the per-shape canvas state churn of immediate-mode drawing.
    ///
    /// The canvas transform and global alpha are reset to identity / 1.0 when
    /// replay finishes, so callers can sandwich the call between
    /// `save()`/`apply_camera()` and `restore()` without leaking state.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The target canvas 2D context.
    /// - `&DrawList` - The recorded commands to replay.
    pub fn replay_context(context: &CanvasRenderingContext2d, list: &DrawList) {
        let mut current_fill: Option<Color> = None;
        let mut current_stroke: Option<Color> = None;
        let mut current_line_width: f64 = f64::NAN;
        // Whether a same-style path run is currently open.
        let mut run_open: bool = false;
        let mut run_is_fill: bool = true;
        let mut run_key: Option<(u8, Color, f64)> = None;

        // Returns the style key for a path-batchable command, or `None` for
        // commands that break a run (sprites, images, text, state changes).
        fn batch_key(command: &DrawCommand) -> Option<(u8, Color, f64)> {
            match command {
                DrawCommand::FillRect { color, .. } | DrawCommand::FillCircle { color, .. } => {
                    Some((0, *color, 0.0))
                }
                DrawCommand::StrokeRect {
                    color, line_width, ..
                }
                | DrawCommand::StrokeCircle {
                    color, line_width, ..
                }
                | DrawCommand::Line {
                    color, line_width, ..
                } => Some((1, *color, *line_width)),
                _ => None,
            }
        }

        // Emits a single path-batchable command's geometry into the open path.
        fn emit_geometry(context: &CanvasRenderingContext2d, command: &DrawCommand) {
            match command {
                DrawCommand::FillRect {
                    position,
                    width,
                    height,
                    ..
                }
                | DrawCommand::StrokeRect {
                    position,
                    width,
                    height,
                    ..
                } => {
                    context.rect(position.get_x(), position.get_y(), *width, *height);
                }
                DrawCommand::FillCircle { center, radius, .. }
                | DrawCommand::StrokeCircle { center, radius, .. } => {
                    context.move_to(center.get_x() + radius, center.get_y());
                    let _ = context.arc(center.get_x(), center.get_y(), *radius, 0.0, TWO_PI);
                }
                DrawCommand::Line { start, end, .. } => {
                    context.move_to(start.get_x(), start.get_y());
                    context.line_to(end.get_x(), end.get_y());
                }
                _ => {}
            }
        }

        for command in self_commands(list) {
            let key: Option<(u8, Color, f64)> = batch_key(command);
            // Close the open run if this command breaks it or starts a new style.
            if run_open && key != run_key {
                if run_is_fill {
                    context.fill();
                } else {
                    context.stroke();
                }
                run_open = false;
            }
            if let Some(current_key) = key {
                // Begin (or continue) a same-style path run.
                if !run_open {
                    let (kind, color, line_width) = current_key;
                    if kind == 0 {
                        if current_fill != Some(color) {
                            context.set_fill_style_str(&Color::to_css(&color));
                            current_fill = Some(color);
                        }
                        run_is_fill = true;
                    } else {
                        if current_stroke != Some(color) {
                            context.set_stroke_style_str(&Color::to_css(&color));
                            current_stroke = Some(color);
                        }
                        if current_line_width != line_width {
                            context.set_line_width(line_width);
                            current_line_width = line_width;
                        }
                        run_is_fill = false;
                    }
                    context.begin_path();
                    run_open = true;
                    run_key = Some(current_key);
                }
                emit_geometry(context, command);
                continue;
            }
            // Non-batchable command: draw it immediately.
            match command {
                DrawCommand::FillText {
                    text,
                    position,
                    color,
                    font,
                } => {
                    if current_fill != Some(*color) {
                        context.set_fill_style_str(&Color::to_css(color));
                        current_fill = Some(*color);
                    }
                    context.set_font(font);
                    let _ = context.fill_text(text, position.get_x(), position.get_y());
                }
                DrawCommand::DrawSprite {
                    image,
                    source,
                    transform,
                } => {
                    draw_sprite_immediate(context, image, source, transform);
                }
                DrawCommand::DrawImageRect {
                    image,
                    source,
                    dest_position,
                    dest_width,
                    dest_height,
                } => {
                    let _ = context
                        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                            image,
                            source.get_x(),
                            source.get_y(),
                            source.get_width(),
                            source.get_height(),
                            dest_position.get_x(),
                            dest_position.get_y(),
                            *dest_width,
                            *dest_height,
                        );
                }
                DrawCommand::SetGlobalAlpha { alpha } => {
                    context.set_global_alpha(Numeric::clamp(*alpha, 0.0, 1.0));
                }
                DrawCommand::SetBlendMode { mode } => {
                    let _ = context.set_global_composite_operation(mode.to_css());
                }
                _ => {}
            }
        }
        // Flush any trailing open run.
        if run_open {
            if run_is_fill {
                context.fill();
            } else {
                context.stroke();
            }
        }
        let _ = context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        context.set_global_alpha(1.0);
    }

    /// Applies the camera transform to the canvas context.
    ///
    /// Translates to the screen center, applies zoom and rotation,
    /// then offsets by the negative camera position.
    pub fn apply_camera(&self) {
        let camera: Camera2D = self.get_camera();
        let _ = self.get_context().translate(
            camera.get_viewport_width() * 0.5,
            camera.get_viewport_height() * 0.5,
        );
        let _ = self
            .get_context()
            .scale(camera.get_zoom(), camera.get_zoom());
        let _ = self.get_context().rotate(camera.get_rotation());
        let _ = self.get_context().translate(
            -camera.get_position().get_x(),
            -camera.get_position().get_y(),
        );
    }

    /// Sets the fill color for subsequent fill operations.
    ///
    /// # Arguments
    ///
    /// - `C: AsRef<str>` - The CSS color string.
    pub fn set_fill_color<C>(&self, color: C)
    where
        C: AsRef<str>,
    {
        self.get_context().set_fill_style_str(color.as_ref());
    }

    /// Sets the stroke color for subsequent stroke operations.
    ///
    /// # Arguments
    ///
    /// - `C: AsRef<str>` - The CSS color string.
    pub fn set_stroke_color<C>(&self, color: C)
    where
        C: AsRef<str>,
    {
        self.get_context().set_stroke_style_str(color.as_ref());
    }

    /// Sets the line width for subsequent stroke operations.
    ///
    /// # Arguments
    ///
    /// - `f64` - The line width in pixels.
    pub fn set_line_width(&self, width: f64) {
        self.get_context().set_line_width(width);
    }

    /// Sets the global alpha (opacity) for all subsequent drawing operations.
    ///
    /// # Arguments
    ///
    /// - `f64` - The alpha value in the range 0.0 to 1.0.
    pub fn set_global_alpha(&self, alpha: f64) {
        self.get_context()
            .set_global_alpha(Numeric::clamp(alpha, 0.0, 1.0));
    }

    /// Fills a rectangle at the given world-space position and dimensions.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The top-left position in world space.
    /// - `f64` - The width.
    /// - `f64` - The height.
    pub fn fill_rect(&self, position: Vector2D, width: f64, height: f64) {
        self.get_context()
            .fill_rect(position.get_x(), position.get_y(), width, height);
    }

    /// Strokes the outline of a rectangle at the given world-space position and dimensions.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The top-left position in world space.
    /// - `f64` - The width.
    /// - `f64` - The height.
    pub fn stroke_rect(&self, position: Vector2D, width: f64, height: f64) {
        self.get_context()
            .stroke_rect(position.get_x(), position.get_y(), width, height);
    }

    /// Fills a circle at the given world-space center with the specified radius.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The center in world space.
    /// - `f64` - The radius.
    pub fn fill_circle(&self, center: Vector2D, radius: f64) {
        self.get_context().begin_path();
        self.get_context()
            .arc(center.get_x(), center.get_y(), radius, 0.0, TWO_PI)
            .unwrap_or(());
        self.get_context().fill();
    }

    /// Strokes the outline of a circle at the given world-space center.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The center in world space.
    /// - `f64` - The radius.
    pub fn stroke_circle(&self, center: Vector2D, radius: f64) {
        self.get_context().begin_path();
        self.get_context()
            .arc(center.get_x(), center.get_y(), radius, 0.0, TWO_PI)
            .unwrap_or(());
        self.get_context().stroke();
    }

    /// Draws a line segment between two world-space points.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The start point.
    /// - `Vector2D` - The end point.
    pub fn draw_line(&self, start: Vector2D, end: Vector2D) {
        self.get_context().begin_path();
        self.get_context().move_to(start.get_x(), start.get_y());
        self.get_context().line_to(end.get_x(), end.get_y());
        self.get_context().stroke();
    }

    /// Fills text at the given world-space position.
    ///
    /// # Arguments
    ///
    /// - `T: AsRef<str>` - The text to draw.
    /// - `Vector2D` - The position in world space.
    pub fn fill_text<T>(&self, text: T, position: Vector2D)
    where
        T: AsRef<str>,
    {
        self.get_context()
            .fill_text(text.as_ref(), position.get_x(), position.get_y())
            .unwrap_or(());
    }

    /// Sets the font for subsequent text rendering.
    ///
    /// # Arguments
    ///
    /// - `F: AsRef<str>` - The CSS font string (e.g., `"16px sans-serif"`).
    pub fn set_font<F>(&self, font: F)
    where
        F: AsRef<str>,
    {
        self.get_context().set_font(font.as_ref());
    }

    /// Draws an image element at the given world-space position and dimensions.
    ///
    /// # Arguments
    ///
    /// - `&HtmlImageElement` - The image element to draw.
    /// - `Vector2D` - The top-left position in world space.
    /// - `f64` - The destination width.
    /// - `f64` - The destination height.
    pub fn draw_image(
        &self,
        image: &HtmlImageElement,
        position: Vector2D,
        width: f64,
        height: f64,
    ) {
        let _ = self
            .get_context()
            .draw_image_with_html_image_element_and_dw_and_dh(
                image,
                position.get_x(),
                position.get_y(),
                width,
                height,
            );
    }

    /// Draws a sub-region of an image element at the given world-space position.
    ///
    /// # Arguments
    ///
    /// - `&HtmlImageElement` - The image element to draw.
    /// - `Rect` - The source rectangle within the image.
    /// - `Vector2D` - The destination top-left position in world space.
    /// - `f64` - The destination width.
    /// - `f64` - The destination height.
    pub fn draw_image_rect(
        &self,
        image: &HtmlImageElement,
        source: Rect,
        dest_position: Vector2D,
        dest_width: f64,
        dest_height: f64,
    ) {
        let _ = self
            .get_context()
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                source.get_x(),
                source.get_y(),
                source.get_width(),
                source.get_height(),
                dest_position.get_x(),
                dest_position.get_y(),
                dest_width,
                dest_height,
            );
    }
}

/// Implements 3D camera transformation and projection methods for `Camera3D`.
impl Camera3D {
    /// Creates a new 3D camera at the given position looking at the target.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The eye position.
    /// - `Vector3D` - The target position to look at.
    /// - `f64` - The viewport width.
    /// - `f64` - The viewport height.
    ///
    /// # Returns
    ///
    /// - `Camera3D` - The new camera.
    pub fn create(
        position: Vector3D,
        target: Vector3D,
        viewport_width: f64,
        viewport_height: f64,
    ) -> Camera3D {
        let mut camera: Camera3D = Camera3D::new(position, target, viewport_width, viewport_height);
        camera.set_up(Vector3D::up());
        camera.set_fov(DEFAULT_CAMERA_FOV);
        camera.set_near(DEFAULT_CAMERA_NEAR);
        camera.set_far(DEFAULT_CAMERA_FAR);
        camera
    }

    /// Returns the aspect ratio (width / height).
    ///
    /// # Returns
    ///
    /// - `f64` - The aspect ratio.
    pub fn aspect(&self) -> f64 {
        if self.get_viewport_height() < EPSILON {
            return 1.0;
        }
        self.get_viewport_width() / self.get_viewport_height()
    }

    /// Returns the forward direction (from position to target, normalized).
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The forward direction.
    pub fn forward(&self) -> Vector3D {
        (self.get_target() - self.get_position()).normalized()
    }

    /// Returns the right direction (cross product of forward and up).
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The right direction.
    pub fn right(&self) -> Vector3D {
        self.forward().cross(self.get_up()).normalized()
    }

    /// Returns the view matrix for this camera.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The view matrix.
    pub fn view_matrix(&self) -> Matrix4x4 {
        Matrix4x4::look_at(self.get_position(), self.get_target(), self.get_up())
    }

    /// Returns the perspective projection matrix for this camera.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The projection matrix.
    pub fn projection_matrix(&self) -> Matrix4x4 {
        Matrix4x4::perspective(
            self.get_fov(),
            self.aspect(),
            self.get_near(),
            self.get_far(),
        )
    }

    /// Returns the combined view-projection matrix.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The view-projection matrix.
    pub fn view_proj_matrix(&self) -> Matrix4x4 {
        self.projection_matrix().multiply(self.view_matrix())
    }

    /// Converts a 3D world-space point to screen-space (NDC) coordinates.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The world-space point.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The screen-space point where x and y are in [0, 1] and z is the depth.
    pub fn world_to_screen(&self, world: Vector3D) -> Vector3D {
        let clip: Vector3D = self.view_proj_matrix().transform_point(world);
        Vector3D::new(
            (clip.get_x() + 1.0) * 0.5 * self.get_viewport_width(),
            (1.0 - clip.get_y()) * 0.5 * self.get_viewport_height(),
            clip.get_z(),
        )
    }

    /// Projects a world-space point and returns whether it is within the camera frustum.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The world-space point.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the point is within the frustum.
    pub fn in_frustum(&self, world: Vector3D) -> bool {
        let clip: Vector3D = self.view_proj_matrix().transform_point(world);
        clip.get_x() >= -1.0
            && clip.get_x() <= 1.0
            && clip.get_y() >= -1.0
            && clip.get_y() <= 1.0
            && clip.get_z() >= -1.0
            && clip.get_z() <= 1.0
    }

    /// Moves the camera position by the given offset, keeping the target offset by the same amount.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The translation offset.
    pub fn translate(&mut self, offset: Vector3D) {
        self.set_position(self.get_position() + offset);
        self.set_target(self.get_target() + offset);
    }

    /// Moves the camera position towards the target by the given distance.
    ///
    /// # Arguments
    ///
    /// - `f64` - The distance to zoom in (positive) or out (negative).
    pub fn zoom(&mut self, distance: f64) {
        let direction: Vector3D = self.forward();
        self.set_position(self.get_position() + direction.scaled(distance));
    }

    /// Orbits the camera around the target by the given yaw and pitch angles.
    ///
    /// # Arguments
    ///
    /// - `f64` - The yaw delta in radians (horizontal rotation).
    /// - `f64` - The pitch delta in radians (vertical rotation).
    pub fn orbit(&mut self, yaw_delta: f64, pitch_delta: f64) {
        let offset: Vector3D = self.get_position() - self.get_target();
        let current_distance: f64 = offset.magnitude();
        let current_yaw: f64 = offset.get_x().atan2(offset.get_z());
        let horizontal_dist: f64 =
            (offset.get_x() * offset.get_x() + offset.get_z() * offset.get_z()).sqrt();
        let current_pitch: f64 = (offset.get_y() / horizontal_dist.max(EPSILON)).asin();
        let new_yaw: f64 = current_yaw + yaw_delta;
        let new_pitch: f64 = Numeric::clamp(
            current_pitch + pitch_delta,
            -HALF_PI + EPSILON,
            HALF_PI - EPSILON,
        );
        let cos_pitch: f64 = new_pitch.cos();
        self.set_position(
            self.get_target()
                + Vector3D::new(
                    new_yaw.sin() * cos_pitch * current_distance,
                    new_pitch.sin() * current_distance,
                    new_yaw.cos() * cos_pitch * current_distance,
                ),
        );
    }
}

/// Implements `Default` for `Camera3D` as a camera at (0, 0, 5) looking at the origin.
impl Default for Camera3D {
    fn default() -> Camera3D {
        Camera3D::create(Vector3D::new(0.0, 0.0, 5.0), Vector3D::zero(), 800.0, 600.0)
    }
}

/// Implements construction, presentation, and anti-aliasing methods for `SsaaCanvas`.
impl SsaaCanvas {
    /// Creates an `SsaaCanvas` from a CSS selector using the default scale factor.
    ///
    /// # Arguments
    ///
    /// - `S: AsRef<str>` - The CSS selector for the display canvas element.
    /// - `f64` - The logical display width in CSS pixels.
    /// - `f64` - The logical display height in CSS pixels.
    ///
    /// # Returns
    ///
    /// - `Option<SsaaCanvas>` - The SSAA canvas, or `None` if the canvas was not found.
    pub fn from_selector<S>(canvas_selector: S, width: f64, height: f64) -> Option<SsaaCanvas>
    where
        S: AsRef<str>,
    {
        Self::from_selector_with_scale(
            canvas_selector,
            width,
            height,
            RENDERER_DEFAULT_SSAA_SCALE_FACTOR,
        )
    }

    /// Creates an `SsaaCanvas` from a CSS selector with a custom SSAA scale factor.
    ///
    /// The offscreen canvas is created at `width * scale_factor` by `height * scale_factor`
    /// pixels, and its context is pre-scaled so that drawing code uses logical coordinates.
    ///
    /// # Arguments
    ///
    /// - `S: AsRef<str>` - The CSS selector for the display canvas element.
    /// - `f64` - The logical display width in CSS pixels.
    /// - `f64` - The logical display height in CSS pixels.
    /// - `f64` - The supersampling scale factor (e.g., 2.0 for 4x SSAA).
    ///
    /// # Returns
    ///
    /// - `Option<SsaaCanvas>` - The SSAA canvas, or `None` if the canvas was not found.
    pub fn from_selector_with_scale<S>(
        canvas_selector: S,
        width: f64,
        height: f64,
        scale_factor: f64,
    ) -> Option<SsaaCanvas>
    where
        S: AsRef<str>,
    {
        let window_value: Window = window().expect("no global window exists");
        let document_value: Document = window_value.document().expect("should have a document");
        let element: Element = document_value
            .query_selector(canvas_selector.as_ref())
            .ok()
            .flatten()?;
        let display_canvas: HtmlCanvasElement = element.unchecked_into();
        let device_pixel_ratio: f64 = CanvasRenderer::detect_dpr();
        let physical_width: u32 = (width * device_pixel_ratio).round() as u32;
        let physical_height: u32 = (height * device_pixel_ratio).round() as u32;
        display_canvas.set_width(physical_width);
        display_canvas.set_height(physical_height);
        let display_context_object: Object = display_canvas
            .get_context(RENDERER_CONTEXT_TYPE_2D)
            .ok()
            .flatten()?;
        let display_context: CanvasRenderingContext2d = display_context_object.unchecked_into();
        let _ = display_context.scale(device_pixel_ratio, device_pixel_ratio);
        let offscreen_canvas: HtmlCanvasElement = document_value
            .create_element(RENDERER_ELEMENT_CANVAS)
            .ok()?
            .unchecked_into();
        let scaled_width: u32 = (width * scale_factor * device_pixel_ratio).round() as u32;
        let scaled_height: u32 = (height * scale_factor * device_pixel_ratio).round() as u32;
        offscreen_canvas.set_width(scaled_width);
        offscreen_canvas.set_height(scaled_height);
        let offscreen_context_object: Object = offscreen_canvas
            .get_context(RENDERER_CONTEXT_TYPE_2D)
            .ok()
            .flatten()?;
        let offscreen_context: CanvasRenderingContext2d = offscreen_context_object.unchecked_into();
        let _ = offscreen_context.scale(
            scale_factor * device_pixel_ratio,
            scale_factor * device_pixel_ratio,
        );
        let ssaa_canvas: SsaaCanvas = SsaaCanvas::new(
            display_canvas,
            display_context,
            offscreen_canvas,
            offscreen_context,
            scale_factor,
            width,
            height,
        );
        ssaa_canvas.enable_smoothing();
        Some(ssaa_canvas)
    }

    /// Presents the offscreen buffer onto the display canvas with high-quality downscaling.
    ///
    /// Applies the active `quality` preset to the display context, clears the
    /// display canvas, then draws the offscreen canvas scaled down to the
    /// logical display size. This is the core SSAA step that produces smooth
    /// polygon edges.
    pub fn present(&self) {
        CanvasRenderer::apply_quality(self.get_display_context(), self.get_quality());
        self.get_display_context()
            .clear_rect(0.0, 0.0, self.get_width(), self.get_height());
        let _ = self
            .get_display_context()
            .draw_image_with_html_canvas_element_and_dw_and_dh(
                self.get_offscreen_canvas(),
                0.0,
                0.0,
                self.get_width(),
                self.get_height(),
            );
    }

    /// Clears the offscreen buffer to transparent.
    pub fn clear(&self) {
        self.get_offscreen_context()
            .clear_rect(0.0, 0.0, self.get_width(), self.get_height());
    }

    /// Clears the offscreen buffer and fills it with the given CSS color.
    ///
    /// # Arguments
    ///
    /// - `C: AsRef<str>` - The CSS color string.
    pub fn clear_color<C>(&self, color: C)
    where
        C: AsRef<str>,
    {
        self.get_offscreen_context()
            .set_fill_style_str(color.as_ref());
        self.get_offscreen_context()
            .fill_rect(0.0, 0.0, self.get_width(), self.get_height());
    }

    /// Enables high-quality anti-aliasing on both the display and offscreen contexts.
    ///
    /// Applies the active `quality` preset to both contexts via the shared
    /// `apply_quality` helper.
    pub fn enable_smoothing(&self) {
        let quality: RenderQuality = self.get_quality();
        CanvasRenderer::apply_quality(self.get_display_context(), quality);
        CanvasRenderer::apply_quality(self.get_offscreen_context(), quality);
    }
}

/// Implements CSS composite operation string conversion for `BlendMode`.
impl BlendMode {
    /// Returns the CSS `globalCompositeOperation` string for this blend mode.
    ///
    /// # Returns
    ///
    /// - `&str` - The CSS composite operation string.
    pub fn to_css(&self) -> &str {
        match self {
            BlendMode::Normal => BLEND_MODE_NORMAL,
            BlendMode::Multiply => BLEND_MODE_MULTIPLY,
            BlendMode::Screen => BLEND_MODE_SCREEN,
            BlendMode::Lighter => BLEND_MODE_LIGHTER,
            BlendMode::Overlay => BLEND_MODE_OVERLAY,
            BlendMode::Darken => BLEND_MODE_DARKEN,
            BlendMode::Lighten => BLEND_MODE_LIGHTEN,
            BlendMode::ColorDodge => BLEND_MODE_COLOR_DODGE,
            BlendMode::ColorBurn => BLEND_MODE_COLOR_BURN,
            BlendMode::HardLight => BLEND_MODE_HARD_LIGHT,
            BlendMode::SoftLight => BLEND_MODE_SOFT_LIGHT,
            BlendMode::Difference => BLEND_MODE_DIFFERENCE,
            BlendMode::Exclusion => BLEND_MODE_EXCLUSION,
            BlendMode::Hue => BLEND_MODE_HUE,
            BlendMode::Saturation => BLEND_MODE_SATURATION,
            BlendMode::Color => BLEND_MODE_COLOR,
            BlendMode::Luminosity => BLEND_MODE_LUMINOSITY,
        }
    }
}

/// Implements construction and canvas gradient creation for `LinearGradient`.
impl LinearGradient {
    /// Creates a new linear gradient from two points and a list of color stops.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The start point.
    /// - `Vector2D` - The end point.
    /// - `Vec<(f64, String)>` - The color stops as (position, color) pairs.
    ///
    /// # Returns
    ///
    /// - `LinearGradient` - The new gradient.
    pub fn create(start: Vector2D, end: Vector2D, stops: Vec<(f64, String)>) -> LinearGradient {
        LinearGradient::new(start, end, stops)
    }

    /// Creates a `CanvasGradient` from this gradient definition on the given context.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas context.
    ///
    /// # Returns
    ///
    /// - `Option<CanvasGradient>` - The canvas gradient, or `None` if creation failed.
    pub fn to_gradient(&self, context: &CanvasRenderingContext2d) -> Option<CanvasGradient> {
        let canvas_gradient: CanvasGradient = context.create_linear_gradient(
            self.get_start().get_x(),
            self.get_start().get_y(),
            self.get_end().get_x(),
            self.get_end().get_y(),
        );
        for (position, color) in self.get_stops() {
            let _ = canvas_gradient.add_color_stop(*position as f32, color);
        }
        Some(canvas_gradient)
    }
}

/// Implements construction and canvas gradient creation for `RadialGradient`.
impl RadialGradient {
    /// Creates a new radial gradient from inner and outer circles and color stops.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The inner circle center.
    /// - `f64` - The inner circle radius.
    /// - `Vector2D` - The outer circle center.
    /// - `f64` - The outer circle radius.
    /// - `Vec<(f64, String)>` - The color stops as (position, color) pairs.
    ///
    /// # Returns
    ///
    /// - `RadialGradient` - The new gradient.
    pub fn create(
        inner_center: Vector2D,
        inner_radius: f64,
        outer_center: Vector2D,
        outer_radius: f64,
        stops: Vec<(f64, String)>,
    ) -> RadialGradient {
        RadialGradient::new(
            inner_center,
            inner_radius,
            outer_center,
            outer_radius,
            stops,
        )
    }

    /// Creates a `CanvasGradient` from this gradient definition on the given context.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas context.
    ///
    /// # Returns
    ///
    /// - `Option<CanvasGradient>` - The canvas gradient, or `None` if creation failed.
    pub fn to_gradient(&self, context: &CanvasRenderingContext2d) -> Option<CanvasGradient> {
        let canvas_gradient: CanvasGradient = context
            .create_radial_gradient(
                self.get_inner_center().get_x(),
                self.get_inner_center().get_y(),
                self.get_inner_radius(),
                self.get_outer_center().get_x(),
                self.get_outer_center().get_y(),
                self.get_outer_radius(),
            )
            .ok()?;
        for (position, color) in self.get_stops() {
            let _ = canvas_gradient.add_color_stop(*position as f32, color);
        }
        Some(canvas_gradient)
    }
}

/// Implements construction methods for `ShadowConfig`.
impl ShadowConfig {
    /// Creates a shadow configuration with default values.
    ///
    /// # Returns
    ///
    /// - `ShadowConfig` - The default shadow configuration.
    pub fn create() -> ShadowConfig {
        ShadowConfig::new(
            RENDERER_DEFAULT_SHADOW_COLOR.to_string(),
            RENDERER_DEFAULT_SHADOW_BLUR,
            0.0,
            0.0,
        )
    }
}

/// Implements `Default` for `ShadowConfig` with default shadow values.
impl Default for ShadowConfig {
    fn default() -> ShadowConfig {
        ShadowConfig::create()
    }
}

/// Implements construction methods for `RenderLayer`.
impl RenderLayer {
    /// Creates a render layer with the given z-index and visibility.
    ///
    /// # Arguments
    ///
    /// - `i32` - The z-index determining draw order.
    /// - `bool` - Whether the layer is visible.
    ///
    /// # Returns
    ///
    /// - `RenderLayer` - The new render layer.
    pub fn create(z_index: i32, visible: bool) -> RenderLayer {
        RenderLayer::new(z_index, visible)
    }

    /// Creates a background render layer with z-index 0 and visibility enabled.
    ///
    /// # Returns
    ///
    /// - `RenderLayer` - The background layer.
    pub fn background() -> RenderLayer {
        RenderLayer::new(RENDERER_LAYER_BACKGROUND, true)
    }

    /// Creates a foreground render layer with a high z-index and visibility enabled.
    ///
    /// # Returns
    ///
    /// - `RenderLayer` - The foreground layer.
    pub fn foreground() -> RenderLayer {
        RenderLayer::new(RENDERER_LAYER_FOREGROUND, true)
    }

    /// Creates a UI overlay render layer with the highest z-index and visibility enabled.
    ///
    /// # Returns
    ///
    /// - `RenderLayer` - The UI overlay layer.
    pub fn ui() -> RenderLayer {
        RenderLayer::new(RENDERER_LAYER_UI, true)
    }
}

/// Implements blend mode, shadow, and gradient rendering methods for `CanvasRenderer`.
impl CanvasRenderer {
    /// Sets the blend mode for compositing subsequent draw operations.
    ///
    /// # Arguments
    ///
    /// - `BlendMode` - The blend mode to apply.
    pub fn set_blend_mode(&self, mode: BlendMode) {
        let _ = self
            .get_context()
            .set_global_composite_operation(mode.to_css());
    }

    /// Applies a shadow configuration for subsequent draw operations.
    ///
    /// # Arguments
    ///
    /// - `&ShadowConfig` - The shadow configuration to apply.
    pub fn set_shadow(&self, config: &ShadowConfig) {
        self.get_context()
            .set_shadow_color(config.get_color().as_str());
        self.get_context().set_shadow_blur(config.get_blur());
        self.get_context()
            .set_shadow_offset_x(config.get_offset_x());
        self.get_context()
            .set_shadow_offset_y(config.get_offset_y());
    }

    /// Clears any previously applied shadow, disabling shadow rendering.
    pub fn clear_shadow(&self) {
        self.get_context().set_shadow_color("rgba(0, 0, 0, 0)");
        self.get_context().set_shadow_blur(0.0);
        self.get_context().set_shadow_offset_x(0.0);
        self.get_context().set_shadow_offset_y(0.0);
    }

    /// Applies a linear gradient as the fill style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&LinearGradient` - The linear gradient to use as fill style.
    pub fn set_linear_gradient_fill(&self, gradient: &LinearGradient) {
        if let Some(canvas_gradient) = gradient.to_gradient(self.get_context()) {
            self.get_context()
                .set_fill_style_canvas_gradient(&canvas_gradient);
        }
    }

    /// Applies a radial gradient as the fill style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&RadialGradient` - The radial gradient to use as fill style.
    pub fn set_radial_gradient_fill(&self, gradient: &RadialGradient) {
        if let Some(canvas_gradient) = gradient.to_gradient(self.get_context()) {
            self.get_context()
                .set_fill_style_canvas_gradient(&canvas_gradient);
        }
    }

    /// Applies a linear gradient as the stroke style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&LinearGradient` - The linear gradient to use as stroke style.
    pub fn set_linear_gradient_stroke(&self, gradient: &LinearGradient) {
        if let Some(canvas_gradient) = gradient.to_gradient(self.get_context()) {
            self.get_context()
                .set_stroke_style_canvas_gradient(&canvas_gradient);
        }
    }

    /// Applies a radial gradient as the stroke style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&RadialGradient` - The radial gradient to use as stroke style.
    pub fn set_radial_gradient_stroke(&self, gradient: &RadialGradient) {
        if let Some(canvas_gradient) = gradient.to_gradient(self.get_context()) {
            self.get_context()
                .set_stroke_style_canvas_gradient(&canvas_gradient);
        }
    }
}

/// Implements the `RenderBackend` trait for `CanvasRenderer`, providing
/// a backend-agnostic rendering interface.
///
/// Each method forwards to the inherent `CanvasRenderer` method of the
/// same name, so the per-call documentation lives on the trait definition
/// in `engine::renderer::trait` — the inherent method is the source of
/// truth, this impl is the trait bridge.
impl RenderBackend for CanvasRenderer {
    /// Forwards to [`CanvasRenderer::clear`].
    fn clear(&self) {
        self.clear();
    }

    /// Forwards to [`CanvasRenderer::clear_color`].
    fn clear_color<C>(&self, color: C)
    where
        C: AsRef<str>,
    {
        self.clear_color(color);
    }

    /// Forwards to [`CanvasRenderer::save`].
    fn save(&self) {
        self.save();
    }

    /// Forwards to [`CanvasRenderer::restore`].
    fn restore(&self) {
        self.restore();
    }

    /// Forwards to [`CanvasRenderer::set_fill_color`].
    fn set_fill_color(&self, color: &str) {
        self.set_fill_color(color);
    }

    /// Forwards to [`CanvasRenderer::set_stroke_color`].
    fn set_stroke_color(&self, color: &str) {
        self.set_stroke_color(color);
    }

    /// Forwards to [`CanvasRenderer::set_line_width`].
    fn set_line_width(&self, width: f64) {
        self.set_line_width(width);
    }

    /// Forwards to [`CanvasRenderer::set_global_alpha`].
    fn set_global_alpha(&self, alpha: f64) {
        self.set_global_alpha(alpha);
    }

    /// Forwards to [`CanvasRenderer::set_blend_mode`].
    fn set_blend_mode(&self, mode: BlendMode) {
        self.set_blend_mode(mode);
    }

    /// Forwards to [`CanvasRenderer::set_shadow`].
    fn set_shadow(&self, config: &ShadowConfig) {
        self.set_shadow(config);
    }

    /// Forwards to [`CanvasRenderer::clear_shadow`].
    fn clear_shadow(&self) {
        self.clear_shadow();
    }

    /// Forwards to [`CanvasRenderer::fill_rect`].
    fn fill_rect(&self, position: Vector2D, width: f64, height: f64) {
        self.fill_rect(position, width, height);
    }

    /// Forwards to [`CanvasRenderer::stroke_rect`].
    fn stroke_rect(&self, position: Vector2D, width: f64, height: f64) {
        self.stroke_rect(position, width, height);
    }

    /// Forwards to [`CanvasRenderer::fill_circle`].
    fn fill_circle(&self, center: Vector2D, radius: f64) {
        self.fill_circle(center, radius);
    }

    /// Forwards to [`CanvasRenderer::stroke_circle`].
    fn stroke_circle(&self, center: Vector2D, radius: f64) {
        self.stroke_circle(center, radius);
    }

    /// Forwards to [`CanvasRenderer::draw_line`].
    fn draw_line(&self, start: Vector2D, end: Vector2D) {
        self.draw_line(start, end);
    }

    /// Forwards to [`CanvasRenderer::fill_text`].
    fn fill_text(&self, text: &str, position: Vector2D) {
        self.fill_text(text, position);
    }

    /// Forwards to [`CanvasRenderer::set_font`].
    fn set_font(&self, font: &str) {
        self.set_font(font);
    }

    /// Forwards to [`CanvasRenderer::draw_image`].
    fn draw_image(&self, image: &HtmlImageElement, position: Vector2D, width: f64, height: f64) {
        self.draw_image(image, position, width, height);
    }

    /// Forwards to [`CanvasRenderer::set_linear_gradient_fill`].
    fn set_linear_gradient_fill(&self, gradient: &LinearGradient) {
        self.set_linear_gradient_fill(gradient);
    }

    /// Forwards to [`CanvasRenderer::set_radial_gradient_fill`].
    fn set_radial_gradient_fill(&self, gradient: &RadialGradient) {
        self.set_radial_gradient_fill(gradient);
    }
}

/// Implements async initialization and GPU resource creation for `WebGpuRenderer`.
impl WebGpuRenderer {
    /// Asynchronously initializes a WebGPU renderer from the given render configuration.
    ///
    /// Requests a GPU adapter and device, obtains the WebGPU canvas context,
    /// and configures it with the preferred texture format. Returns `None` if
    /// WebGPU is not supported, the adapter/device request fails, or the canvas
    /// element is not found.
    ///
    /// # Arguments
    ///
    /// - `&RenderConfig` - The rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Option<WebGpuRenderer>` - The initialized renderer, or `None` on failure.
    ///   Maximum time in milliseconds to wait for `requestAdapter` and
    ///   `requestDevice` before treating them as failed.
    ///
    /// Some browser GPU states (headless, no GPU, sandboxed, device-lost)
    /// leave the WebGPU adapter/device promises permanently pending instead
    /// of resolving to `null` or rejecting. Without a timeout the
    /// `JsFuture::from(...).await` inside `init` would hang forever and
    /// the UI would stay stuck on `Initializing...`. Wrapping each promise
    /// in `Promise.race` against a timer-rejected sibling forces the
    /// future to resolve so the caller's `let Some(...) = ... else { ... }`
    /// branch can run and report `WebGPU Not Supported`.
    /// Returns a Promise that rejects after `INIT_PROMISE_TIMEOUT_MILLIS`.
    fn timeout_promise() -> Promise {
        let window_value: Window = window().expect("no global window exists");
        Promise::new(&mut |_resolve: Function, reject: Function| {
            let reject_fn: Function = reject.clone();
            let timer: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                let _ = reject_fn.call1(
                    &JsValue::UNDEFINED,
                    &JsValue::from_str(RENDERER_TIMEOUT_ERROR_MESSAGE),
                );
            }));
            let _ = window_value.set_timeout_with_callback_and_timeout_and_arguments_0(
                timer.as_ref().unchecked_ref(),
                INIT_PROMISE_TIMEOUT_MILLIS,
            );
            timer.forget();
        })
    }

    /// Wraps `promise` in `Promise.race([promise, timeout_promise()])` so that
    /// awaiting it never blocks longer than `INIT_PROMISE_TIMEOUT_MILLIS`.
    ///
    /// Calls `Promise.race` via reflection because wasm-bindgen does not
    /// currently expose the static `race` method on `js_sys::Promise`.
    fn race_with_timeout(promise: Promise) -> Promise {
        let array: Array = Array::of2(&promise, &Self::timeout_promise());
        Promise::race(&array)
    }

    /// Asynchronously initializes a WebGPU renderer from the given render configuration.
    ///
    /// Requests a GPU adapter and device, obtains the WebGPU canvas context,
    /// and configures it with the preferred texture format. Returns `Err` if
    /// WebGPU is not supported, the adapter/device request fails, the canvas
    /// element is not found, or the adapter/device request hangs beyond
    /// `INIT_PROMISE_TIMEOUT_MILLIS` (a defensive timeout for browser GPU
    /// states that leave the WebGPU promises permanently pending).
    ///
    /// The engine no longer logs diagnostic output internally; instead each
    /// failure mode is returned as a distinct `WebGpuInitError` variant so
    /// the caller can decide how to surface it (typically via `Console::error`
    /// or by falling back to the Canvas 2D backend).
    ///
    /// # Arguments
    ///
    /// - `&RenderConfig` - The rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Result<WebGpuRenderer, WebGpuInitError>` - The initialized renderer, or
    ///   a typed error describing the specific failure.
    pub async fn init(config: &RenderConfig) -> Result<WebGpuRenderer, WebGpuInitError> {
        let window: Window = window().expect("no global window exists");
        let navigator: Navigator = window.navigator();
        let gpu_result: Result<JsValue, JsValue> =
            Reflect::get(navigator.as_ref(), &JsValue::from_str(WEBGPU_CONTEXT_TYPE));
        let gpu: JsValue = match gpu_result {
            Ok(value) => value,
            Err(err) => return Err(WebGpuInitError::NavigatorLookup(err)),
        };
        if gpu.is_undefined() || gpu.is_null() {
            return Err(WebGpuInitError::NavigatorGpuMissing);
        }
        let adapter_options: Object = Object::new();
        let _ = Reflect::set(
            &adapter_options,
            &JsValue::from_str(WEBGPU_PROPERTY_POWER_PREFERENCE),
            &JsValue::from_str(config.power_preference.to_web_sys_string()),
        );
        let request_adapter_fn: Function =
            match Reflect::get(&gpu, &JsValue::from_str(WEBGPU_METHOD_REQUEST_ADAPTER)) {
                Ok(value) => value.unchecked_into(),
                Err(err) => return Err(WebGpuInitError::RequestAdapterLookup(err)),
            };
        let adapter_promise: Promise = match request_adapter_fn.call1(&gpu, &adapter_options) {
            Ok(value) => value.unchecked_into(),
            Err(err) => return Err(WebGpuInitError::RequestAdapterCall(err)),
        };
        let adapter_value: JsValue =
            match JsFuture::from(Self::race_with_timeout(adapter_promise)).await {
                Ok(value) => value,
                Err(err) => return Err(WebGpuInitError::AdapterPromise(err)),
            };
        if adapter_value.is_null() || adapter_value.is_undefined() {
            return Err(WebGpuInitError::AdapterUnavailable);
        }
        let device_descriptor: Object = Object::new();
        let request_device_fn: Function = match Reflect::get(
            &adapter_value,
            &JsValue::from_str(WEBGPU_METHOD_REQUEST_DEVICE),
        ) {
            Ok(value) => value.unchecked_into(),
            Err(err) => return Err(WebGpuInitError::RequestDeviceLookup(err)),
        };
        let device_promise: Promise =
            match request_device_fn.call1(&adapter_value, &device_descriptor) {
                Ok(value) => value.unchecked_into(),
                Err(err) => return Err(WebGpuInitError::RequestDeviceCall(err)),
            };
        let device_value: JsValue =
            match JsFuture::from(Self::race_with_timeout(device_promise)).await {
                Ok(value) => value,
                Err(err) => return Err(WebGpuInitError::DevicePromise(err)),
            };
        if device_value.is_null() || device_value.is_undefined() {
            return Err(WebGpuInitError::DeviceUnavailable);
        }
        let document: Document = window.document().expect("should have a document");
        let element: Element = match document.query_selector(&config.canvas_selector) {
            Ok(Some(el)) => el,
            Ok(None) => {
                return Err(WebGpuInitError::CanvasNotFound(
                    config.canvas_selector.clone(),
                ));
            }
            Err(err) => return Err(WebGpuInitError::CanvasQuery(err)),
        };
        let canvas: HtmlCanvasElement = element.unchecked_into();
        let context_object: Option<Object> = canvas.get_context(WEBGPU_CONTEXT_TYPE).ok().flatten();
        let context_object: Object = match context_object {
            Some(c) => c,
            None => return Err(WebGpuInitError::CanvasContextUnavailable),
        };
        let context: JsValue = context_object.into();
        let get_format_fn: Function =
            match Reflect::get(&gpu, &JsValue::from_str(WEBGPU_METHOD_GET_PREFERRED_FORMAT)) {
                Ok(value) => value.unchecked_into(),
                Err(err) => return Err(WebGpuInitError::PreferredFormatLookup(err)),
            };
        let format_value: JsValue = match get_format_fn.call0(&gpu) {
            Ok(value) => value,
            Err(err) => return Err(WebGpuInitError::PreferredFormatCall(err)),
        };
        let format: String = match format_value.as_string() {
            Some(s) => s,
            None => return Err(WebGpuInitError::PreferredFormatType(format_value)),
        };
        // WebGPU's `configure` requires the canvas backing-store size to be
        // set BEFORE calling configure, otherwise the swap chain is created
        // at 0x0 and the first getCurrentTexture() returns an error.
        let dpr: f64 = CanvasRenderer::detect_dpr();
        let physical_width: u32 = (config.width * dpr).round() as u32;
        let physical_height: u32 = (config.height * dpr).round() as u32;
        canvas.set_width(physical_width);
        canvas.set_height(physical_height);
        let canvas_config: Object = Object::new();
        let _ = Reflect::set(
            &canvas_config,
            &JsValue::from_str(WEBGPU_PROPERTY_DEVICE),
            &device_value,
        );
        let _ = Reflect::set(
            &canvas_config,
            &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
            &format_value,
        );
        let configure_fn: Function =
            match Reflect::get(&context, &JsValue::from_str(WEBGPU_METHOD_CONFIGURE)) {
                Ok(value) => value.unchecked_into(),
                Err(err) => return Err(WebGpuInitError::ConfigureLookup(err)),
            };
        let _ = configure_fn.call1(&context, &canvas_config);
        let queue: JsValue =
            match Reflect::get(&device_value, &JsValue::from_str(WEBGPU_PROPERTY_QUEUE)) {
                Ok(value) => value,
                Err(err) => return Err(WebGpuInitError::QueueLookup(err)),
            };
        Ok(WebGpuRenderer {
            device: device_value,
            queue,
            context,
            canvas,
            format,
            width: physical_width,
            height: physical_height,
            antialias: config.antialias,
        })
    }

    /// Resizes the canvas backing store and reconfigures the swap chain.
    ///
    /// WebGPU's `GpuCanvasContext.configure` is sticky: it sets the texture
    /// format and device once, but the swap chain tracks the canvas's
    /// `width`/`height` attributes. When the CSS layout size changes (a
    /// window resize, a panel toggle, a DPR change) the canvas keeps its
    /// old physical dimensions unless we explicitly update `width`/`height`
    /// and call `configure` again. Without this, subsequent
    /// `getCurrentTexture()` calls return a texture that no longer matches
    /// the visible region and the frame either stretches or freezes.
    ///
    /// Re-`configure`ing with the same `device` + `format` is the
    /// spec-defined way to swap in a fresh swap chain bound to the new
    /// backing-store size.
    ///
    /// # Arguments
    ///
    /// - `u32` - The new physical pixel width (already multiplied by DPR).
    /// - `u32` - The new physical pixel height.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` on success, `false` if the swap chain or canvas
    ///   handles were missing or `configure` failed.
    pub fn resize(&mut self, physical_width: u32, physical_height: u32) -> bool {
        if self.get_canvas().is_null()
            || self.get_context().is_null()
            || self.get_device().is_undefined()
        {
            return false;
        }
        self.get_canvas().set_width(physical_width);
        self.get_canvas().set_height(physical_height);
        let format_value: JsValue = JsValue::from_str(&self.get_format());
        let canvas_config: Object = Object::new();
        let _ = Reflect::set(
            &canvas_config,
            &JsValue::from_str(WEBGPU_PROPERTY_DEVICE),
            self.get_device(),
        );
        let _ = Reflect::set(
            &canvas_config,
            &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
            &format_value,
        );
        let configure_fn: Function = Reflect::get(
            self.get_context(),
            &JsValue::from_str(WEBGPU_METHOD_CONFIGURE),
        )
        .ok()
        .and_then(|value: JsValue| value.dyn_into::<Function>().ok())
        .unwrap_or_else(|| Function::new_no_args(""));
        if configure_fn
            .call1(self.get_context(), &canvas_config)
            .is_err()
        {
            return false;
        }
        self.set_width(physical_width);
        self.set_height(physical_height);
        true
    }

    /// Creates a shader module from WGSL source code.
    ///
    /// # Arguments
    ///
    /// - `S: AsRef<str>` - The WGSL shader source code.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created shader module as a JavaScript value.
    pub(crate) fn create_shader_module<S>(&self, code: S) -> JsValue
    where
        S: AsRef<str>,
    {
        let descriptor: Object = Object::new();
        let _ = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_CODE),
            &JsValue::from_str(code.as_ref()),
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_SHADER_MODULE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Creates a new command encoder for recording GPU commands.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created command encoder as a JavaScript value.
    pub(crate) fn create_command_encoder(&self) -> JsValue {
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_COMMAND_ENCODER),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call0(self.get_device())
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Returns the current texture view from the canvas swap chain.
    ///
    /// This texture view should be used as the color attachment target for
    /// render passes. The texture is automatically presented to the canvas
    /// when the command buffer is submitted.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The current frame's texture view as a JavaScript value.
    pub(crate) fn get_current_texture_view(&self) -> JsValue {
        let get_texture_fn: Function = Reflect::get(
            self.get_context(),
            &JsValue::from_str(WEBGPU_METHOD_GET_CURRENT_TEXTURE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let texture: JsValue = get_texture_fn
            .call0(self.get_context())
            .unwrap_or(JsValue::UNDEFINED);
        let create_view_fn: Function =
            Reflect::get(&texture, &JsValue::from_str(WEBGPU_METHOD_CREATE_VIEW))
                .unwrap_or(JsValue::UNDEFINED)
                .unchecked_into();
        create_view_fn.call0(&texture).unwrap_or(JsValue::UNDEFINED)
    }

    /// Begins a render pass on the given command encoder with a clear color.
    ///
    /// The render pass targets the canvas's current texture and clears it
    /// to the specified color. The returned `JsValue` is a `GpuRenderPassEncoder`
    /// that can be used to issue draw commands. The pass must be ended (via `end()`)
    /// before the command encoder is finished.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The command encoder to begin the pass on.
    /// - `(f64, f64, f64, f64)` - The clear color as (r, g, b, a) in 0.0–1.0 range.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The active render pass encoder as a JavaScript value.
    pub(crate) fn begin_render_pass(
        &self,
        encoder: &JsValue,
        clear_color: (f64, f64, f64, f64),
    ) -> JsValue {
        let view: JsValue = self.get_current_texture_view();
        let color_dict: Object = Object::new();
        let _ = Reflect::set(
            &color_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_R),
            &JsValue::from_f64(clear_color.0),
        );
        let _ = Reflect::set(
            &color_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_G),
            &JsValue::from_f64(clear_color.1),
        );
        let _ = Reflect::set(
            &color_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_B),
            &JsValue::from_f64(clear_color.2),
        );
        let _ = Reflect::set(
            &color_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_A),
            &JsValue::from_f64(clear_color.3),
        );
        let attachment: Object = Object::new();
        let _ = Reflect::set(&attachment, &JsValue::from_str(WEBGPU_PROPERTY_VIEW), &view);
        let _ = Reflect::set(
            &attachment,
            &JsValue::from_str(WEBGPU_PROPERTY_LOAD_OP),
            &JsValue::from_str(WEBGPU_LOAD_OP_CLEAR),
        );
        let _ = Reflect::set(
            &attachment,
            &JsValue::from_str(WEBGPU_PROPERTY_STORE_OP),
            &JsValue::from_str(WEBGPU_STORE_OP_STORE),
        );
        let _ = Reflect::set(
            &attachment,
            &JsValue::from_str(WEBGPU_PROPERTY_CLEAR_VALUE),
            &color_dict,
        );
        let color_attachments: Array = Array::new();
        color_attachments.push(&attachment);
        let descriptor: Object = Object::new();
        let _ = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_COLOR_ATTACHMENTS),
            &color_attachments,
        );
        let begin_fn: Function =
            Reflect::get(encoder, &JsValue::from_str(WEBGPU_METHOD_BEGIN_RENDER_PASS))
                .unwrap_or(JsValue::UNDEFINED)
                .unchecked_into();
        begin_fn
            .call1(encoder, &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Submits an array of command buffers to the GPU queue for execution.
    ///
    /// # Arguments
    ///
    /// - `&[JsValue]` - The command buffers to submit.
    pub(crate) fn submit(&self, command_buffers: &[JsValue]) {
        let array: Array = Array::new();
        for buffer in command_buffers {
            array.push(buffer);
        }
        let submit_fn: Function =
            Reflect::get(self.get_queue(), &JsValue::from_str(WEBGPU_METHOD_SUBMIT))
                .unwrap_or(JsValue::UNDEFINED)
                .unchecked_into();
        let _ = submit_fn.call1(self.get_queue(), &array);
    }

    /// Creates a simple render pipeline from a single WGSL shader source.
    ///
    /// The shader must contain `@vertex fn vs_main(...)` and
    /// `@fragment fn fs_main(...)` entry points. No vertex buffers are used;
    /// vertex positions should be derived from `@builtin(vertex_index)` in
    /// the shader. The pipeline uses auto-layout (`layout: null`), which works
    /// when the shader has no bind groups.
    ///
    /// # Arguments
    ///
    /// - `S: AsRef<str>` - The WGSL shader source code.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created render pipeline as a JavaScript value.
    pub fn create_render_pipeline<S>(&self, shader_code: S) -> JsValue
    where
        S: AsRef<str>,
    {
        let module: JsValue = self.create_shader_module(shader_code);
        let vertex_state: Object = Object::new();
        let _ = Reflect::set(
            &vertex_state,
            &JsValue::from_str(WEBGPU_PROPERTY_MODULE),
            &module,
        );
        let _ = Reflect::set(
            &vertex_state,
            &JsValue::from_str(WEBGPU_PROPERTY_ENTRY_POINT),
            &JsValue::from_str(WEBGPU_VERTEX_ENTRY_POINT),
        );
        let _ = Reflect::set(
            &vertex_state,
            &JsValue::from_str(WEBGPU_PROPERTY_BUFFERS),
            &Array::new(),
        );
        let target: Object = Object::new();
        let _ = Reflect::set(
            &target,
            &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
            &JsValue::from_str(&self.get_format()),
        );
        let targets: Array = Array::new();
        targets.push(&target);
        let fragment_state: Object = Object::new();
        let _ = Reflect::set(
            &fragment_state,
            &JsValue::from_str(WEBGPU_PROPERTY_MODULE),
            &module,
        );
        let _ = Reflect::set(
            &fragment_state,
            &JsValue::from_str(WEBGPU_PROPERTY_ENTRY_POINT),
            &JsValue::from_str(WEBGPU_FRAGMENT_ENTRY_POINT),
        );
        let _ = Reflect::set(
            &fragment_state,
            &JsValue::from_str(WEBGPU_PROPERTY_TARGETS),
            &targets,
        );
        let primitive: Object = Object::new();
        let _ = Reflect::set(
            &primitive,
            &JsValue::from_str(WEBGPU_PROPERTY_TOPOLOGY),
            &JsValue::from_str(WEBGPU_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST),
        );
        let descriptor: Object = Object::new();
        let _ = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_LAYOUT),
            &JsValue::null(),
        );
        let _ = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_VERTEX),
            &vertex_state,
        );
        let _ = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_FRAGMENT),
            &fragment_state,
        );
        let _ = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_PRIMITIVE),
            &primitive,
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_RENDER_PIPELINE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Sets the render pipeline on a render pass encoder.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder.
    /// - `&JsValue` - The render pipeline to set.
    pub(crate) fn set_pipeline(&self, pass: &JsValue, pipeline: &JsValue) {
        let set_fn: Function = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_SET_PIPELINE))
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into();
        let _ = set_fn.call1(pass, pipeline);
    }

    /// Draws primitives on a render pass encoder.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder.
    /// - `u32` - The number of vertices to draw.
    /// - `u32` - The number of instances to draw.
    pub(crate) fn draw(&self, pass: &JsValue, vertex_count: u32, instance_count: u32) {
        let draw_fn: Function = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_DRAW))
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into();
        let _ = draw_fn.call2(
            pass,
            &JsValue::from_f64(f64::from(vertex_count)),
            &JsValue::from_f64(f64::from(instance_count)),
        );
    }

    /// Ends a render pass on the given pass encoder.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder to end.
    pub(crate) fn end_render_pass(&self, pass: &JsValue) {
        let end_fn: Function = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_END))
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into();
        let _ = end_fn.call0(pass);
    }

    /// Finishes a command encoder and returns the resulting command buffer.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The command encoder to finish.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The finished command buffer.
    pub(crate) fn finish_command_encoder(&self, encoder: &JsValue) -> JsValue {
        let finish_fn: Function = Reflect::get(encoder, &JsValue::from_str(WEBGPU_METHOD_FINISH))
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into();
        finish_fn.call0(encoder).unwrap_or(JsValue::UNDEFINED)
    }

    /// Renders a complete frame with a pipeline and animated clear color.
    ///
    /// This is a convenience method that creates a command encoder, begins a
    /// render pass with the given clear color, sets the pipeline, draws the
    /// specified number of vertices, ends the pass, finishes the encoder, and
    /// submits the command buffer.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pipeline to use.
    /// - `(f64, f64, f64, f64)` - The clear color as (r, g, b, a) in 0.0–1.0 range.
    /// - `u32` - The number of vertices to draw.
    pub fn render_frame(
        &self,
        pipeline: &JsValue,
        clear_color: (f64, f64, f64, f64),
        vertex_count: u32,
    ) {
        let encoder: JsValue = self.create_command_encoder();
        let pass: JsValue = self.begin_render_pass(&encoder, clear_color);
        self.set_pipeline(&pass, pipeline);
        self.draw(&pass, vertex_count, 1);
        self.end_render_pass(&pass);
        let command_buffer: JsValue = self.finish_command_encoder(&encoder);
        self.submit(&[command_buffer]);
    }
}

/// Implements helper methods on `WebGpuInitError`.
///
/// These methods provide ergonomic access to the diagnostic code and the
/// underlying JS error value, which are useful when surfacing the failure
/// to the user (e.g. via `Console::error` from the example crate).
impl WebGpuInitError {
    /// Returns a short, machine-readable identifier for this error variant.
    ///
    /// Suitable for use as a stable error code in logs or telemetry.
    /// The codes are stable across releases.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The error code (e.g. `"WEBGPU_NAVIGATOR_GPU_MISSING"`).
    pub fn code(&self) -> &'static str {
        match self {
            Self::NavigatorLookup(_) => "WEBGPU_NAVIGATOR_LOOKUP",
            Self::NavigatorGpuMissing => "WEBGPU_NAVIGATOR_GPU_MISSING",
            Self::RequestAdapterLookup(_) => "WEBGPU_REQUEST_ADAPTER_LOOKUP",
            Self::RequestAdapterCall(_) => "WEBGPU_REQUEST_ADAPTER_CALL",
            Self::AdapterPromise(_) => "WEBGPU_ADAPTER_PROMISE",
            Self::AdapterUnavailable => "WEBGPU_ADAPTER_UNAVAILABLE",
            Self::RequestDeviceLookup(_) => "WEBGPU_REQUEST_DEVICE_LOOKUP",
            Self::RequestDeviceCall(_) => "WEBGPU_REQUEST_DEVICE_CALL",
            Self::DevicePromise(_) => "WEBGPU_DEVICE_PROMISE",
            Self::DeviceUnavailable => "WEBGPU_DEVICE_UNAVAILABLE",
            Self::CanvasNotFound(_) => "WEBGPU_CANVAS_NOT_FOUND",
            Self::CanvasQuery(_) => "WEBGPU_CANVAS_QUERY",
            Self::CanvasContextUnavailable => "WEBGPU_CANVAS_CONTEXT_UNAVAILABLE",
            Self::PreferredFormatLookup(_) => "WEBGPU_PREFERRED_FORMAT_LOOKUP",
            Self::PreferredFormatCall(_) => "WEBGPU_PREFERRED_FORMAT_CALL",
            Self::PreferredFormatType(_) => "WEBGPU_PREFERRED_FORMAT_TYPE",
            Self::ConfigureLookup(_) => "WEBGPU_CONFIGURE_LOOKUP",
            Self::QueueLookup(_) => "WEBGPU_QUEUE_LOOKUP",
        }
    }

    /// Returns the underlying JS error value if this variant carries one.
    ///
    /// Variants that do not capture a JS value (e.g. `NavigatorGpuMissing`,
    /// `AdapterUnavailable`, `CanvasNotFound`, `CanvasContextUnavailable`)
    /// return `None`.
    ///
    /// # Returns
    ///
    /// - `Option<&JsValue>` - The captured JS error, if any.
    pub fn js_error(&self) -> Option<&JsValue> {
        match self {
            Self::NavigatorLookup(err)
            | Self::RequestAdapterLookup(err)
            | Self::RequestAdapterCall(err)
            | Self::AdapterPromise(err)
            | Self::RequestDeviceLookup(err)
            | Self::RequestDeviceCall(err)
            | Self::DevicePromise(err)
            | Self::CanvasQuery(err)
            | Self::PreferredFormatLookup(err)
            | Self::PreferredFormatCall(err)
            | Self::PreferredFormatType(err)
            | Self::ConfigureLookup(err)
            | Self::QueueLookup(err) => Some(err),
            Self::NavigatorGpuMissing
            | Self::AdapterUnavailable
            | Self::DeviceUnavailable
            | Self::CanvasContextUnavailable
            | Self::CanvasNotFound(_) => None,
        }
    }
}

/// Renders the JS-side error into a `String` when present, otherwise `"<none>"`.
fn js_error_to_string(value: &JsValue) -> String {
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

/// Implements `std::fmt::Display` for `WebGpuInitError`.
///
/// The formatted message is intended for end-user diagnostic output
/// (typically forwarded to `Console::error` by the calling application)
/// and includes the variant code plus a human-readable description. When
/// the variant carries a JS error, its `Debug` form is appended.
impl std::fmt::Display for WebGpuInitError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NavigatorLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(navigator, webgpu) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::NavigatorGpuMissing => write!(
                formatter,
                "[{}] navigator.gpu is missing - browser does not expose WebGPU on this origin",
                self.code(),
            ),
            Self::RequestAdapterLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(gpu, requestAdapter) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::RequestAdapterCall(err) => write!(
                formatter,
                "[{}] gpu.requestAdapter() threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::AdapterPromise(err) => write!(
                formatter,
                "[{}] adapter promise rejected or timed out: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::AdapterUnavailable => write!(
                formatter,
                "[{}] requestAdapter returned null - no compatible GPU adapter for the requested powerPreference",
                self.code(),
            ),
            Self::RequestDeviceLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(adapter, requestDevice) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::RequestDeviceCall(err) => write!(
                formatter,
                "[{}] adapter.requestDevice() threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::DevicePromise(err) => write!(
                formatter,
                "[{}] device promise rejected or timed out: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::DeviceUnavailable => write!(
                formatter,
                "[{}] requestDevice returned null - adapter could not allocate a device (possibly device-lost)",
                self.code(),
            ),
            Self::CanvasNotFound(selector) => write!(
                formatter,
                "[{}] canvas element {:?} not found in DOM",
                self.code(),
                selector,
            ),
            Self::CanvasQuery(err) => write!(
                formatter,
                "[{}] querySelector threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::CanvasContextUnavailable => write!(
                formatter,
                "[{}] canvas.get_context('webgpu') returned null - the canvas may already be using another context type or WebGPU is disabled",
                self.code(),
            ),
            Self::PreferredFormatLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(gpu, getPreferredCanvasFormat) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::PreferredFormatCall(err) => write!(
                formatter,
                "[{}] gpu.getPreferredCanvasFormat() threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::PreferredFormatType(value) => write!(
                formatter,
                "[{}] getPreferredCanvasFormat returned non-string: {}",
                self.code(),
                js_error_to_string(value),
            ),
            Self::ConfigureLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(context, configure) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::QueueLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(device, queue) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
        }
    }
}

/// Implements the standard `std::error::Error` trait for `WebGpuInitError`.
///
/// The `source()` method delegates to the underlying JS error's `toString()`
/// representation when present, otherwise returns `None`. The engine never
/// logs or prints anything; this impl exists solely so the error composes
/// with `Result`-based APIs and `?` operator chains.
impl std::error::Error for WebGpuInitError {}
