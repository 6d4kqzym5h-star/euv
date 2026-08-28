use super::*;

/// One entry of the [`euv_toc`] anchor list.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvTocItem {
    /// The heading level (levels deeper than 2 render indented).
    #[get(type(copy))]
    pub level: u8,
    /// The display text.
    #[get(type(copy))]
    pub text: &'static str,
    /// The full href (for hash routers typically `#<route>#<anchor>`).
    #[get(type(copy))]
    pub href: &'static str,
}

/// Props for the [`euv_toc`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvTocProps {
    /// The list title (for example `On this page`).
    #[get(type(copy))]
    pub title: &'static str,
    /// The anchor items.
    #[get(type(copy))]
    pub items: &'static [EuvTocItem],
}
