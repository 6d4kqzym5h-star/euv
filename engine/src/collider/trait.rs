use super::*;

/// A trait for objects that can participate in collision detection.
pub trait Collider {
    /// Returns the geometric shape of this collider.
    ///
    /// # Returns
    ///
    /// - `ColliderShape` - The shape variant.
    fn shape(&self) -> ColliderShape;

    /// Returns the axis-aligned bounding box that encloses this collider.
    ///
    /// # Returns
    ///
    /// - `Rect` - The bounding box.
    fn bounding_box(&self) -> Rect;

    /// Tests whether this collider contains the given point.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The point to test.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the point is inside.
    fn contains_point(&self, point: Vector2D) -> bool;

    /// Returns the center point of this collider.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The center point.
    fn center(&self) -> Vector2D;
}

/// A trait for 3D objects that can participate in collision detection.
pub trait Collider3D {
    /// Returns the geometric shape of this 3D collider.
    ///
    /// # Returns
    ///
    /// - `ColliderShape3D` - The shape variant.
    fn shape(&self) -> ColliderShape3D;

    /// Returns the 3D axis-aligned bounding box that encloses this collider.
    ///
    /// # Returns
    ///
    /// - `AABB3D` - The bounding box.
    fn bounding_box(&self) -> AABB3D;

    /// Tests whether this collider contains the given 3D point.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The point to test.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the point is inside.
    fn contains_point(&self, point: Vector3D) -> bool;

    /// Returns the center point of this 3D collider.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The center point.
    fn center(&self) -> Vector3D;
}
