use super::*;

/// Props for the `header` component.
///
/// Defines the strongly-typed interface for a standard header
/// with an emoji icon, title, and subtitle.
#[derive(Clone, Data, Debug, Default, New)]
pub struct EuvHeaderProps {
    /// The emoji icon displayed alongside the title.
    #[get(type(copy))]
    pub icon: &'static str,
    /// The page title text displayed in the heading.
    #[get(type(copy))]
    pub title: &'static str,
    /// The page subtitle / description text.
    #[get(type(copy))]
    pub subtitle: &'static str,
}
