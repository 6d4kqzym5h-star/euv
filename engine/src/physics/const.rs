/// The default mass for a dynamically simulated rigid body in kilograms.
pub(crate) const PHYSICS_DEFAULT_MASS: f64 = 1.0;

/// The mass value used for static bodies, representing infinite mass.
pub(crate) const PHYSICS_STATIC_MASS: f64 = 0.0;

/// The maximum number of collision iterations per physics step for stable stacking.
pub(crate) const PHYSICS_MAX_ITERATIONS: u32 = 8;

/// The maximum position correction percentage applied per iteration.
pub(crate) const PHYSICS_POSITION_PERCENT: f64 = 0.8;
