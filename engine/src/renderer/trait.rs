use crate::*;

/// A trait for objects that can be rendered onto a canvas.
pub trait Renderable {
    /// Draws this object onto the given canvas rendering context.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas 2D rendering context.
    /// - `&Transform2D` - The world-space transform to apply.
    fn draw(&self, context: &CanvasRenderingContext2d, transform: &Transform2D);
}
