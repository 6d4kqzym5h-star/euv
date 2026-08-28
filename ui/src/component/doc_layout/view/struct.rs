use super::*;

/// Props for the [`euv_doc_layout`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDocLayoutProps {
    /// The right-column table-of-contents title (e.g. `"On this page"`).
    pub toc_title: &'static str,
    /// The right-column table-of-contents items (column hidden when empty).
    pub toc_items: &'static [EuvTocItem],
    /// The "previous page" link label (e.g. `"Previous"`).
    pub prev_label: &'static str,
    /// The "next page" link label (e.g. `"Next"`).
    pub next_label: &'static str,
    /// The previous page entry (skipped when `None`).
    pub prev: Option<EuvPaginationItem>,
    /// The next page entry (skipped when `None`).
    pub next: Option<EuvPaginationItem>,
    /// The footer text under the pagination (skipped when empty).
    pub footer: &'static str,
}
