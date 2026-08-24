use super::*;

/// Represents the geometric shape of a collider.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ColliderShape {
    /// An axis-aligned rectangle defined by a `Rect`.
    #[default]
    Aabb,
    /// A circle defined by a `Circle`.
    Circle,
}

/// Represents the geometric shape of a 3D collider.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ColliderShape3D {
    /// A 3D axis-aligned bounding box defined by an `AABB3D`.
    #[default]
    Aabb,
    /// A sphere defined by a `Sphere`.
    Sphere,
}
