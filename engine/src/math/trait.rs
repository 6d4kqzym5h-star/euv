/// A trait for types that support linear interpolation between two values.
pub trait Interpolable {
    /// Performs linear interpolation between `self` and `other` by the factor `t`.
    ///
    /// # Arguments
    ///
    /// - `f64` - The interpolation factor, typically in the range 0.0 to 1.0.
    /// - `Self` - The target value to interpolate towards.
    ///
    /// # Returns
    ///
    /// - `Self` - The interpolated result.
    fn lerp(&self, other: Self, t: f64) -> Self;
}
