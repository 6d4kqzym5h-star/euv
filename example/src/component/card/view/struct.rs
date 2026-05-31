/// Props for the `my_card` component.
///
/// Defines the strongly-typed interface for the card wrapper.
#[derive(Clone, Default)]
pub(crate) struct MyCardProps {
    /// The card title displayed in the header.
    pub(crate) title: &'static str,
}
