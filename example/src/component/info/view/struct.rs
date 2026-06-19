/// Props for the `euv_info` component.
///
/// Defines the strongly-typed interface for a key-value information row.
#[derive(Clone, Default)]
pub(crate) struct EuvInfoProps {
    /// The label text displayed on the left side.
    pub(crate) label: &'static str,
}
