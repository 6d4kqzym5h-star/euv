use crate::*;

/// Defines how a rigid body participates in the physics simulation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BodyType {
    /// A static body that does not move under forces or collisions (e.g., walls, ground).
    #[default]
    Static,
    /// A dynamic body that is fully simulated by forces, gravity, and collisions.
    Dynamic,
    /// A kinematic body that moves programmatically but is not affected by forces or gravity.
    Kinematic,
}

/// Wraps the concrete collider shape data attached to a rigid body.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BodyCollider {
    /// An axis-aligned bounding box collider.
    Aabb(AabbCollider),
    /// A circle collider.
    Circle(CircleCollider),
}

/// Wraps the concrete 3D collider shape data attached to a 3D rigid body.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BodyCollider3D {
    /// A 3D axis-aligned bounding box collider.
    Aabb(AabbCollider3D),
    /// A 3D sphere collider.
    Sphere(SphereCollider3D),
}
