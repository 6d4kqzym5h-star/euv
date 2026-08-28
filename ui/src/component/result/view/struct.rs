use super::*;

/// Props for the [`euv_result`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvResultProps {
    /// The big status code (e.g. `"404"`, skipped when empty).
    #[get(type(copy))]
    pub code: &'static str,
    /// The result title (skipped when empty).
    #[get(type(copy))]
    pub title: &'static str,
    /// The description rendered under the title (skipped when empty).
    #[get(type(copy))]
    pub description: &'static str,
}
