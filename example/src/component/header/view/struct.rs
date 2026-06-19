/// Props for the `header` component.
///
/// Defines the strongly-typed interface for a standard header
/// with an emoji icon, title, and subtitle.
#[derive(Clone, Default)]
pub(crate) struct EuvHeaderProps {
    /// The emoji icon displayed alongside the title.
    pub(crate) icon: &'static str,
    /// The page title text displayed in the heading.
    pub(crate) title: &'static str,
    /// The page subtitle / description text.
    pub(crate) subtitle: &'static str,
}
