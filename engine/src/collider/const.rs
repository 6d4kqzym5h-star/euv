/// A small tolerance value used to determine if two collider boundaries are just touching
/// versus overlapping, preventing false positives at exact boundary contact.
pub(crate) const COLLIDER_CONTACT_EPSILON: f64 = 1e-4;
