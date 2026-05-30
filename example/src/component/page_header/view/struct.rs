/// Props for the `page_header` component.
///
/// Defines the strongly-typed interface for a standard page header
/// with a title and subtitle.
#[derive(Default)]
pub(crate) struct PageHeaderProps {
    /// The page title text displayed in the heading.
    pub(crate) title: &'static str,
    /// The page subtitle / description text.
    pub(crate) subtitle: &'static str,
}
