use crate::*;

/// Props for the `euv_info` component.
///
/// Defines the strongly-typed interface for a key-value information row.
#[derive(Clone, Data, Debug, Default, New)]
pub struct EuvInfoProps {
    /// The label text displayed on the left side.
    #[get(type(copy))]
    pub label: &'static str,
}
