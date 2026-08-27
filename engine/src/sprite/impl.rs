use super::*;

/// Implements frame extraction, rendering, and animation creation for `SpriteSheet`.
impl SpriteSheet {
    /// Creates a new sprite sheet from an image element and uniform frame dimensions.
    ///
    /// Computes the grid columns and rows from the image dimensions and frame size.
    ///
    /// # Arguments
    ///
    /// - `HtmlImageElement` - The sprite sheet image.
    /// - `f64` - The width of each frame.
    /// - `f64` - The height of each frame.
    ///
    /// # Returns
    ///
    /// - `SpriteSheet` - The new sprite sheet.
    pub fn from_image(image: HtmlImageElement, frame_width: f64, frame_height: f64) -> SpriteSheet {
        let total_width: f64 = image.width() as f64;
        let total_height: f64 = image.height() as f64;
        let columns: u32 = (total_width / frame_width).max(1.0) as u32;
        let rows: u32 = (total_height / frame_height).max(1.0) as u32;
        SpriteSheet::new(image, frame_width, frame_height, columns, rows)
    }

    /// Returns the source rectangle for the frame at the given grid index.
    ///
    /// # Arguments
    ///
    /// - `u32` - The frame index (left-to-right, top-to-bottom).
    ///
    /// # Returns
    ///
    /// - `Rect` - The source rectangle.
    pub fn frame_source(&self, index: u32) -> Rect {
        let column: u32 = index % self.get_columns();
        let row: u32 = index / self.get_columns();
        Rect::new(
            column as f64 * self.get_frame_width(),
            row as f64 * self.get_frame_height(),
            self.get_frame_width(),
            self.get_frame_height(),
        )
    }

    /// Creates a frame at the given grid index with a default duration.
    ///
    /// # Arguments
    ///
    /// - `u32` - The frame index.
    ///
    /// # Returns
    ///
    /// - `SpriteFrame` - The new frame.
    pub fn frame(&self, index: u32) -> SpriteFrame {
        SpriteFrame::new(self.frame_source(index), SPRITE_DEFAULT_FRAME_DURATION)
    }

    /// Creates an animation from a range of frame indices.
    ///
    /// # Arguments
    ///
    /// - `&str` - The animation name.
    /// - `u32` - The starting frame index (inclusive).
    /// - `u32` - The ending frame index (exclusive).
    /// - `AnimationMode` - The playback mode.
    ///
    /// # Returns
    ///
    /// - `SpriteAnimation` - The new animation.
    pub fn animation(
        &self,
        name: &str,
        start: u32,
        end: u32,
        mode: AnimationMode,
    ) -> SpriteAnimation {
        let frames: Vec<SpriteFrame> = (start..end).map(|index: u32| self.frame(index)).collect();
        SpriteAnimation::new(name.to_string(), frames, mode)
    }

    /// Draws a static (non-animated) sprite frame onto the canvas.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas context.
    /// - `u32` - The frame index to draw.
    /// - `&Transform2D` - The world-space transform to apply.
    pub fn draw_frame(
        &self,
        context: &CanvasRenderingContext2d,
        frame_index: u32,
        transform: &Transform2D,
    ) {
        let source: Rect = self.frame_source(frame_index);
        // Compose the TRS matrix in Rust and apply it with a single `set_transform`
        // instead of save/translate/rotate/scale/restore (5 FFI calls + a canvas
        // state-stack push/pop per sprite). Scale signs handle flipping.
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
                self.get_image(),
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
}

/// Implements playback control for `Animator`.
impl Animator {
    /// Creates a new idle animator with no animation.
    ///
    /// # Returns
    ///
    /// - `Animator` - The new animator.
    pub fn create() -> Animator {
        Animator::new(AnimationState::Paused, 1)
    }

    /// Plays the given animation from the beginning.
    ///
    /// # Arguments
    ///
    /// - `SpriteAnimation` - The animation to play.
    pub fn play(&mut self, animation: SpriteAnimation) {
        self.set_current_animation(Some(animation));
        self.set_current_frame_index(0);
        self.set_elapsed_time(0.0);
        self.set_state(AnimationState::Playing);
        self.set_direction(1);
    }

    /// Pauses the current animation.
    pub fn pause(&mut self) {
        if self.get_state() == AnimationState::Playing {
            self.set_state(AnimationState::Paused);
        }
    }

    /// Resumes the current animation from where it was paused.
    pub fn resume(&mut self) {
        if self.get_state() == AnimationState::Paused {
            self.set_state(AnimationState::Playing);
        }
    }

    /// Stops the current animation and resets to the first frame.
    pub fn stop(&mut self) {
        self.set_current_frame_index(0);
        self.set_elapsed_time(0.0);
        self.set_state(AnimationState::Paused);
        self.set_direction(1);
    }

    /// Advances the animation by the given delta time.
    ///
    /// # Arguments
    ///
    /// - `f64` - The time elapsed since the last update, in seconds.
    pub fn update(&mut self, delta_time: f64) {
        if self.get_state() != AnimationState::Playing {
            return;
        }
        let current_frame_index: usize = self.get_current_frame_index();
        let (frame_count, current_duration, mode) = {
            let Some(animation) = self.get_mut_current_animation().as_ref() else {
                return;
            };
            if animation.get_frames().is_empty() {
                return;
            }
            (
                animation.get_frames().len(),
                animation.get_frames()[current_frame_index].get_duration(),
                animation.get_mode(),
            )
        };
        *self.get_mut_elapsed_time() += delta_time;
        if self.get_elapsed_time() < current_duration {
            return;
        }
        self.set_elapsed_time(0.0);
        self.advance_frame(frame_count, mode);
    }

    /// Returns the source rectangle of the current frame, or `None` if no animation is playing.
    ///
    /// # Returns
    ///
    /// - `Option<Rect>` - The current frame's source rectangle.
    pub fn current_frame_source(&self) -> Option<Rect> {
        let animation: Option<SpriteAnimation> = self.get_current_animation();
        let animation: &SpriteAnimation = animation.as_ref()?;
        let frame: &SpriteFrame = animation.get_frames().get(self.get_current_frame_index())?;
        Some(frame.get_source())
    }

    /// Advances to the next frame according to the animation mode.
    ///
    /// # Arguments
    ///
    /// - `&SpriteAnimation` - The current animation.
    fn advance_frame(&mut self, frame_count: usize, mode: AnimationMode) {
        match mode {
            AnimationMode::Loop => {
                self.set_current_frame_index((self.get_current_frame_index() + 1) % frame_count);
            }
            AnimationMode::Once => {
                if self.get_current_frame_index() + 1 < frame_count {
                    *self.get_mut_current_frame_index() += 1;
                } else {
                    self.set_state(AnimationState::Finished);
                }
            }
            AnimationMode::PingPong => {
                if self.get_direction() > 0 {
                    if self.get_current_frame_index() + 1 < frame_count {
                        *self.get_mut_current_frame_index() += 1;
                    } else {
                        self.set_direction(-1);
                        if self.get_current_frame_index() > 0 {
                            *self.get_mut_current_frame_index() -= 1;
                        }
                    }
                } else if self.get_current_frame_index() > 0 {
                    *self.get_mut_current_frame_index() -= 1;
                } else {
                    self.set_direction(1);
                    if self.get_current_frame_index() + 1 < frame_count {
                        *self.get_mut_current_frame_index() += 1;
                    }
                }
            }
        }
    }
}

/// Forwards `Animator::update` through the [`Updatable`] trait so that
/// collections of heterogeneous updateable objects (entities, animators,
/// physics worlds, scene managers) can be driven by a single scheduler loop.
///
/// The inherent [`Animator::update`] method is the canonical implementation;
/// this impl exists purely for trait dispatch. The inherent call resolves
/// first when both are in scope, so there is no recursion.
impl Updatable for Animator {
    /// Advances the simulation by `delta_time` seconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - Seconds elapsed since the previous update.
    fn update(&mut self, delta_time: f64) {
        Animator::update(self, delta_time);
    }
}

/// Implements animated sprite rendering for `Animator`.
impl Animator {
    /// Draws the current frame of this animator onto the canvas.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas context.
    /// - `&SpriteSheet` - The sprite sheet containing the image.
    /// - `&Transform2D` - The world-space transform to apply.
    pub fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        sheet: &SpriteSheet,
        transform: &Transform2D,
    ) {
        let Some(source) = self.current_frame_source() else {
            return;
        };
        // Compose the TRS matrix in Rust (flip signs folded into scale) and apply
        // it with a single `set_transform` instead of save/translate/rotate/scale/restore.
        let rotation: f64 = transform.get_rotation();
        let cos: f64 = rotation.cos();
        let sin: f64 = rotation.sin();
        let scale_x: f64 = if self.get_flip_x() {
            -transform.get_scale().get_x()
        } else {
            transform.get_scale().get_x()
        };
        let scale_y: f64 = if self.get_flip_y() {
            -transform.get_scale().get_y()
        } else {
            transform.get_scale().get_y()
        };
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
                sheet.get_image(),
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
}

/// Implements `Default` for `Animator` as a new idle animator.
impl Default for Animator {
    /// Constructs a default [`Animator`] value.
    ///
    /// # Returns
    ///
    /// - `Animator` - A default-constructed instance with the documented initial state.
    fn default() -> Animator {
        Animator::create()
    }
}
