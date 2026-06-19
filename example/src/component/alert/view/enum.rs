/// The alert variant determining the visual style.
#[derive(Clone, Default)]
pub enum AlertVariant {
    /// A success alert with green styling.
    #[default]
    Success,
    /// An error alert with red styling.
    Error,
}
