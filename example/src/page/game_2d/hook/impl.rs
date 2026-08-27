use super::*;

/// Equality comparison for [`BallStore`].
impl PartialEq for BallStore {
    /// Returns `true` when `self` and `other` are equivalent by the [`PartialEq`] contract.
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

/// Equality comparison for [`CanvasCache`].
impl PartialEq for CanvasCache {
    /// Returns `true` when `self` and `other` are equivalent by the [`PartialEq`] contract.
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
