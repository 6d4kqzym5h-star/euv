use crate::*;

/// Props for the `euv_card` component.
///
/// Defines the strongly-typed interface for the card wrapper.
#[derive(Clone, Data, Debug, Default, New)]
pub struct EuvCardProps {
    /// The card title displayed in the header.
    #[get(type(copy))]
    pub title: &'static str,
}
