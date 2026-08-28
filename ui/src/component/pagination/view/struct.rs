use super::*;

/// One side of the [`euv_pagination`] prev/next footer.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvPaginationItem {
    /// The display text.
    #[get(type(copy))]
    pub text: &'static str,
    /// The link target (hash route path).
    #[get(type(copy))]
    pub link: &'static str,
}

/// Props for the [`euv_pagination`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvPaginationProps {
    /// The label above the previous entry (for example `Previous`).
    #[get(type(copy))]
    pub prev_label: &'static str,
    /// The label above the next entry (for example `Next`).
    #[get(type(copy))]
    pub next_label: &'static str,
    /// The previous entry; `None` renders a spacer.
    #[get(type(copy))]
    pub prev: Option<EuvPaginationItem>,
    /// The next entry; `None` renders a spacer.
    #[get(type(copy))]
    pub next: Option<EuvPaginationItem>,
}
