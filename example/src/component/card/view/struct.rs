use crate::*;

/// Props for the `my_card` component.
///
/// Defines the strongly-typed interface for the card wrapper.
#[derive(Data, Debug, Default)]
pub struct MyCardProps {
    /// The card title displayed in the header.
    pub title: String,
}
