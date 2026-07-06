use crate::*;

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
    /// - `&str` - The font family name.
    ///
    /// # Returns
    ///
    /// - `String` - The CSS font string (e.g., `"16px sans-serif"`).
    pub fn build_font_string(size: f64, family: &str) -> String {
        format!("{}px {}", size, family)
    }

    /// Creates a default font string using the default font size and family.
    ///
    /// # Returns
    ///
    /// - `String` - The default CSS font string.
    pub fn default_font_string() -> String {
        Self::build_font_string(RENDERER_DEFAULT_FONT_SIZE, RENDERER_DEFAULT_FONT_FAMILY)
    }

    /// Enables high-quality anti-aliasing on an arbitrary canvas 2D context.
    ///
    /// Sets `imageSmoothingEnabled` to `true` and `imageSmoothingQuality` to `"high"`
    /// on the given context. This is a static utility for code that manages its own
    /// `CanvasRenderingContext2d` without using a `CanvasRenderer` instance.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas context to configure.
    pub fn enable_context_anti_aliasing(context: &CanvasRenderingContext2d) {
        let _ = Reflect::set(
            context,
            &JsValue::from_str(RENDERER_PROPERTY_IMAGE_SMOOTHING_ENABLED),
            &JsValue::from_bool(true),
        );
        let _ = Reflect::set(
            context,
            &JsValue::from_str(RENDERER_PROPERTY_IMAGE_SMOOTHING_QUALITY),
            &JsValue::from_str(RENDERER_IMAGE_SMOOTHING_QUALITY_HIGH),
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

/// Implements drawing and camera management methods for `CanvasRenderer`.
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
    pub fn from_selector(
        canvas_selector: &str,
        viewport_width: f64,
        viewport_height: f64,
    ) -> Option<CanvasRenderer> {
        let window_value: Window = window().expect("no global window exists");
        let document_value: Document = window_value.document().expect("should have a document");
        let element: Element = document_value
            .query_selector(canvas_selector)
            .ok()
            .flatten()?;
        let canvas_element: HtmlCanvasElement = element.unchecked_into();
        let context_object: Object = canvas_element
            .get_context(RENDERER_CONTEXT_TYPE_2D)
            .ok()
            .flatten()?;
        let context: CanvasRenderingContext2d = context_object.unchecked_into();
        Some(CanvasRenderer::new(
            context,
            Camera2D::create(viewport_width, viewport_height),
        ))
    }

    /// Enables high-quality anti-aliasing on the canvas context by setting
    /// `imageSmoothingEnabled` to `true` and `imageSmoothingQuality` to `"high"`.
    ///
    /// This improves the quality of image scaling operations (e.g., `draw_image`)
    /// but does not affect vector primitives like lines and fills, which are
    /// already anti-aliased by the browser's 2D canvas implementation.
    pub fn enable_anti_aliasing(&self) {
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_IMAGE_SMOOTHING_ENABLED),
            &JsValue::from_bool(true),
        );
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_IMAGE_SMOOTHING_QUALITY),
            &JsValue::from_str(RENDERER_IMAGE_SMOOTHING_QUALITY_HIGH),
        );
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
    /// - `&str` - The CSS color string (e.g., `"#000000"`).
    pub fn clear_with_color(&self, color: &str) {
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_FILL_STYLE),
            &JsValue::from_str(color),
        );
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
    /// - `&str` - The CSS color string.
    pub fn set_fill_color(&self, color: &str) {
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_FILL_STYLE),
            &JsValue::from_str(color),
        );
    }

    /// Sets the stroke color for subsequent stroke operations.
    ///
    /// # Arguments
    ///
    /// - `&str` - The CSS color string.
    pub fn set_stroke_color(&self, color: &str) {
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_STROKE_STYLE),
            &JsValue::from_str(color),
        );
    }

    /// Sets the line width for subsequent stroke operations.
    ///
    /// # Arguments
    ///
    /// - `f64` - The line width in pixels.
    pub fn set_line_width(&self, width: f64) {
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_LINE_WIDTH),
            &JsValue::from_f64(width),
        );
    }

    /// Sets the global alpha (opacity) for all subsequent drawing operations.
    ///
    /// # Arguments
    ///
    /// - `f64` - The alpha value in the range 0.0 to 1.0.
    pub fn set_global_alpha(&self, alpha: f64) {
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_GLOBAL_ALPHA),
            &JsValue::from_f64(Numeric::clamp(alpha, 0.0, 1.0)),
        );
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
    /// - `&str` - The text to draw.
    /// - `Vector2D` - The position in world space.
    pub fn fill_text(&self, text: &str, position: Vector2D) {
        self.get_context()
            .fill_text(text, position.get_x(), position.get_y())
            .unwrap_or(());
    }

    /// Sets the font for subsequent text rendering.
    ///
    /// # Arguments
    ///
    /// - `&str` - The CSS font string (e.g., `"16px sans-serif"`).
    pub fn set_font(&self, font: &str) {
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_FONT),
            &JsValue::from_str(font),
        );
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
    pub fn draw_image_subregion(
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
    pub fn view_projection_matrix(&self) -> Matrix4x4 {
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
        let clip: Vector3D = self.view_projection_matrix().transform_point(world);
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
    pub fn is_in_frustum(&self, world: Vector3D) -> bool {
        let clip: Vector3D = self.view_projection_matrix().transform_point(world);
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
    /// - `&str` - The CSS selector for the display canvas element.
    /// - `f64` - The logical display width in CSS pixels.
    /// - `f64` - The logical display height in CSS pixels.
    ///
    /// # Returns
    ///
    /// - `Option<SsaaCanvas>` - The SSAA canvas, or `None` if the canvas was not found.
    pub fn from_selector(canvas_selector: &str, width: f64, height: f64) -> Option<SsaaCanvas> {
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
    /// - `&str` - The CSS selector for the display canvas element.
    /// - `f64` - The logical display width in CSS pixels.
    /// - `f64` - The logical display height in CSS pixels.
    /// - `f64` - The supersampling scale factor (e.g., 2.0 for 4x SSAA).
    ///
    /// # Returns
    ///
    /// - `Option<SsaaCanvas>` - The SSAA canvas, or `None` if the canvas was not found.
    pub fn from_selector_with_scale(
        canvas_selector: &str,
        width: f64,
        height: f64,
        scale_factor: f64,
    ) -> Option<SsaaCanvas> {
        let window_value: Window = window().expect("no global window exists");
        let document_value: Document = window_value.document().expect("should have a document");
        let element: Element = document_value
            .query_selector(canvas_selector)
            .ok()
            .flatten()?;
        let display_canvas: HtmlCanvasElement = element.unchecked_into();
        display_canvas.set_width(width as u32);
        display_canvas.set_height(height as u32);
        let display_context_object: Object = display_canvas
            .get_context(RENDERER_CONTEXT_TYPE_2D)
            .ok()
            .flatten()?;
        let display_context: CanvasRenderingContext2d = display_context_object.unchecked_into();
        let offscreen_canvas: HtmlCanvasElement = document_value
            .create_element(RENDERER_ELEMENT_CANVAS)
            .ok()?
            .unchecked_into();
        let scaled_width: u32 = (width * scale_factor) as u32;
        let scaled_height: u32 = (height * scale_factor) as u32;
        offscreen_canvas.set_width(scaled_width);
        offscreen_canvas.set_height(scaled_height);
        let offscreen_context_object: Object = offscreen_canvas
            .get_context(RENDERER_CONTEXT_TYPE_2D)
            .ok()
            .flatten()?;
        let offscreen_context: CanvasRenderingContext2d = offscreen_context_object.unchecked_into();
        let _ = offscreen_context.scale(scale_factor, scale_factor);
        let ssaa_canvas: SsaaCanvas = SsaaCanvas::new(
            display_canvas,
            display_context,
            offscreen_canvas,
            offscreen_context,
            scale_factor,
            width,
            height,
        );
        ssaa_canvas.enable_anti_aliasing();
        Some(ssaa_canvas)
    }

    /// Presents the offscreen buffer onto the display canvas with high-quality downscaling.
    ///
    /// Enables `imageSmoothingEnabled` and `imageSmoothingQuality = "high"` on the
    /// display context, clears the display canvas, then draws the offscreen canvas
    /// scaled down to the logical display size. This is the core SSAA step that
    /// produces smooth polygon edges.
    pub fn present(&self) {
        let _ = Reflect::set(
            &self.display_context,
            &JsValue::from_str(RENDERER_PROPERTY_IMAGE_SMOOTHING_ENABLED),
            &JsValue::from_bool(true),
        );
        let _ = Reflect::set(
            &self.display_context,
            &JsValue::from_str(RENDERER_PROPERTY_IMAGE_SMOOTHING_QUALITY),
            &JsValue::from_str(RENDERER_IMAGE_SMOOTHING_QUALITY_HIGH),
        );
        self.display_context
            .clear_rect(0.0, 0.0, self.width, self.height);
        let _ = self
            .display_context
            .draw_image_with_html_canvas_element_and_dw_and_dh(
                &self.offscreen_canvas,
                0.0,
                0.0,
                self.width,
                self.height,
            );
    }

    /// Clears the offscreen buffer to transparent.
    pub fn clear(&self) {
        self.offscreen_context
            .clear_rect(0.0, 0.0, self.width, self.height);
    }

    /// Clears the offscreen buffer and fills it with the given CSS color.
    ///
    /// # Arguments
    ///
    /// - `&str` - The CSS color string.
    pub fn clear_with_color(&self, color: &str) {
        let _ = Reflect::set(
            &self.offscreen_context,
            &JsValue::from_str(RENDERER_PROPERTY_FILL_STYLE),
            &JsValue::from_str(color),
        );
        self.offscreen_context
            .fill_rect(0.0, 0.0, self.width, self.height);
    }

    /// Enables high-quality anti-aliasing on both the display and offscreen contexts.
    ///
    /// Sets `imageSmoothingEnabled = true` and `imageSmoothingQuality = "high"`
    /// on both contexts to ensure optimal image scaling quality.
    pub fn enable_anti_aliasing(&self) {
        let _ = Reflect::set(
            &self.display_context,
            &JsValue::from_str(RENDERER_PROPERTY_IMAGE_SMOOTHING_ENABLED),
            &JsValue::from_bool(true),
        );
        let _ = Reflect::set(
            &self.display_context,
            &JsValue::from_str(RENDERER_PROPERTY_IMAGE_SMOOTHING_QUALITY),
            &JsValue::from_str(RENDERER_IMAGE_SMOOTHING_QUALITY_HIGH),
        );
        let _ = Reflect::set(
            &self.offscreen_context,
            &JsValue::from_str(RENDERER_PROPERTY_IMAGE_SMOOTHING_ENABLED),
            &JsValue::from_bool(true),
        );
        let _ = Reflect::set(
            &self.offscreen_context,
            &JsValue::from_str(RENDERER_PROPERTY_IMAGE_SMOOTHING_QUALITY),
            &JsValue::from_str(RENDERER_IMAGE_SMOOTHING_QUALITY_HIGH),
        );
    }
}

/// Implements CSS composite operation string conversion for `BlendMode`.
impl BlendMode {
    /// Returns the CSS `globalCompositeOperation` string for this blend mode.
    ///
    /// # Returns
    ///
    /// - `&str` - The CSS composite operation string.
    pub fn to_css_string(&self) -> &str {
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
    pub fn to_canvas_gradient(&self, context: &CanvasRenderingContext2d) -> Option<CanvasGradient> {
        let canvas_gradient: CanvasGradient = context.create_linear_gradient(
            self.get_start().get_x(),
            self.get_start().get_y(),
            self.get_end().get_x(),
            self.get_end().get_y(),
        );
        for (position, color) in &self.stops {
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
    pub fn to_canvas_gradient(&self, context: &CanvasRenderingContext2d) -> Option<CanvasGradient> {
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
        for (position, color) in &self.stops {
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
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_GLOBAL_COMPOSITE_OPERATION),
            &JsValue::from_str(mode.to_css_string()),
        );
    }

    /// Applies a shadow configuration for subsequent draw operations.
    ///
    /// # Arguments
    ///
    /// - `&ShadowConfig` - The shadow configuration to apply.
    pub fn set_shadow(&self, config: &ShadowConfig) {
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_SHADOW_COLOR),
            &JsValue::from_str(config.get_color().as_str()),
        );
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_SHADOW_BLUR),
            &JsValue::from_f64(config.get_blur()),
        );
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_SHADOW_OFFSET_X),
            &JsValue::from_f64(config.get_offset_x()),
        );
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_SHADOW_OFFSET_Y),
            &JsValue::from_f64(config.get_offset_y()),
        );
    }

    /// Clears any previously applied shadow, disabling shadow rendering.
    pub fn clear_shadow(&self) {
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_SHADOW_COLOR),
            &JsValue::from_str("rgba(0, 0, 0, 0)"),
        );
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_SHADOW_BLUR),
            &JsValue::from_f64(0.0),
        );
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_SHADOW_OFFSET_X),
            &JsValue::from_f64(0.0),
        );
        let _ = Reflect::set(
            self.get_context(),
            &JsValue::from_str(RENDERER_PROPERTY_SHADOW_OFFSET_Y),
            &JsValue::from_f64(0.0),
        );
    }

    /// Applies a linear gradient as the fill style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&LinearGradient` - The linear gradient to use as fill style.
    pub fn set_linear_gradient_fill(&self, gradient: &LinearGradient) {
        if let Some(canvas_gradient) = gradient.to_canvas_gradient(self.get_context()) {
            let _ = Reflect::set(
                self.get_context(),
                &JsValue::from_str(RENDERER_PROPERTY_FILL_STYLE),
                &canvas_gradient,
            );
        }
    }

    /// Applies a radial gradient as the fill style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&RadialGradient` - The radial gradient to use as fill style.
    pub fn set_radial_gradient_fill(&self, gradient: &RadialGradient) {
        if let Some(canvas_gradient) = gradient.to_canvas_gradient(self.get_context()) {
            let _ = Reflect::set(
                self.get_context(),
                &JsValue::from_str(RENDERER_PROPERTY_FILL_STYLE),
                &canvas_gradient,
            );
        }
    }

    /// Applies a linear gradient as the stroke style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&LinearGradient` - The linear gradient to use as stroke style.
    pub fn set_linear_gradient_stroke(&self, gradient: &LinearGradient) {
        if let Some(canvas_gradient) = gradient.to_canvas_gradient(self.get_context()) {
            let _ = Reflect::set(
                self.get_context(),
                &JsValue::from_str(RENDERER_PROPERTY_STROKE_STYLE),
                &canvas_gradient,
            );
        }
    }

    /// Applies a radial gradient as the stroke style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&RadialGradient` - The radial gradient to use as stroke style.
    pub fn set_radial_gradient_stroke(&self, gradient: &RadialGradient) {
        if let Some(canvas_gradient) = gradient.to_canvas_gradient(self.get_context()) {
            let _ = Reflect::set(
                self.get_context(),
                &JsValue::from_str(RENDERER_PROPERTY_STROKE_STYLE),
                &canvas_gradient,
            );
        }
    }
}

/// Implements the `RenderBackend` trait for `CanvasRenderer`, providing
/// a backend-agnostic rendering interface.
impl RenderBackend for CanvasRenderer {
    fn clear(&self) {
        self.clear();
    }

    fn clear_with_color(&self, color: &str) {
        self.clear_with_color(color);
    }

    fn save(&self) {
        self.save();
    }

    fn restore(&self) {
        self.restore();
    }

    fn set_fill_color(&self, color: &str) {
        self.set_fill_color(color);
    }

    fn set_stroke_color(&self, color: &str) {
        self.set_stroke_color(color);
    }

    fn set_line_width(&self, width: f64) {
        self.set_line_width(width);
    }

    fn set_global_alpha(&self, alpha: f64) {
        self.set_global_alpha(alpha);
    }

    fn set_blend_mode(&self, mode: BlendMode) {
        self.set_blend_mode(mode);
    }

    fn set_shadow(&self, config: &ShadowConfig) {
        self.set_shadow(config);
    }

    fn clear_shadow(&self) {
        self.clear_shadow();
    }

    fn fill_rect(&self, position: Vector2D, width: f64, height: f64) {
        self.fill_rect(position, width, height);
    }

    fn stroke_rect(&self, position: Vector2D, width: f64, height: f64) {
        self.stroke_rect(position, width, height);
    }

    fn fill_circle(&self, center: Vector2D, radius: f64) {
        self.fill_circle(center, radius);
    }

    fn stroke_circle(&self, center: Vector2D, radius: f64) {
        self.stroke_circle(center, radius);
    }

    fn draw_line(&self, start: Vector2D, end: Vector2D) {
        self.draw_line(start, end);
    }

    fn fill_text(&self, text: &str, position: Vector2D) {
        self.fill_text(text, position);
    }

    fn set_font(&self, font: &str) {
        self.set_font(font);
    }

    fn draw_image(&self, image: &HtmlImageElement, position: Vector2D, width: f64, height: f64) {
        self.draw_image(image, position, width, height);
    }

    fn set_linear_gradient_fill(&self, gradient: &LinearGradient) {
        self.set_linear_gradient_fill(gradient);
    }

    fn set_radial_gradient_fill(&self, gradient: &RadialGradient) {
        self.set_radial_gradient_fill(gradient);
    }
}
