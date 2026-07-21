use super::*;

/// Props for the `euv_loading` component.
///
/// Defines the strongly-typed interface for a loading indicator that can render
/// inline or as an absolute overlay covering its parent container.
#[derive(Clone, Data, Debug, Default, New)]
pub struct EuvLoadingProps {
    /// The loading title text displayed beside the spinner.
    #[get(type(copy))]
    pub title: &'static str,
    /// The optional subtitle text displayed below the title (empty string hides it).
    #[get(type(copy))]
    pub subtitle: &'static str,
    /// Whether to render as an absolute overlay instead of an inline flex row.
    #[get(type(copy))]
    pub overlay: bool,
    /// The background color for overlay mode (empty string for transparent).
    #[get(type(copy))]
    pub background: &'static str,
}
