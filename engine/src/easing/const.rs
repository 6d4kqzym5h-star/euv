use super::*;

/// The overshoot amplitude used by the `Back` easing family.
pub(crate) const BACK_OVERSHOOT: f64 = 1.70158;

/// The scaled overshoot used by `InOutBack` so both halves peak at the
/// same amplitude as the single-direction variants.
pub(crate) const BACK_OVERSHOOT_INOUT: f64 = BACK_OVERSHOOT * 1.525;

/// The number of bounce subdivisions used by the `Bounce` easing family.
pub(crate) const BOUNCE_DIVISIONS: f64 = 2.75;

/// The period base used by the `Elastic` easing family.
pub(crate) const ELASTIC_PERIOD: f64 = TWO_PI / 3.0;

/// The period base used by the `InOutElastic` easing variant.
pub(crate) const ELASTIC_PERIOD_INOUT: f64 = TWO_PI / 4.5;
