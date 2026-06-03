use crate::*;

/// Reactive state for the select and textarea demo feature.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseSelect {
    /// The selected fruit value.
    #[get(type(copy))]
    pub(crate) selected_fruit: Signal<String>,
    /// The selected country value.
    #[get(type(copy))]
    pub(crate) selected_country: Signal<String>,
    /// The selected city value.
    #[get(type(copy))]
    pub(crate) selected_city: Signal<String>,
    /// The available city options for the selected country.
    #[get(type(copy))]
    pub(crate) cities: Signal<Vec<(String, String)>>,
    /// The feedback result message.
    #[get(type(copy))]
    pub(crate) feedback: Signal<String>,
    /// The textarea content.
    #[get(type(copy))]
    pub(crate) textarea_content: Signal<String>,
    /// The textarea validation error message.
    #[get(type(copy))]
    pub(crate) textarea_error: Signal<String>,
}
