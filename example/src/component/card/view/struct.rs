use crate::*;

/// Props for the `my_card` component.
///
/// Defines the strongly-typed interface for the card wrapper.
#[derive(Data, Debug, Default, New)]
pub(crate) struct MyCardProps {
    /// The card title displayed in the header.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) title: String,
}
