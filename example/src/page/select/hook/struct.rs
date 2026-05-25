use crate::*;

/// Reactive state for the select and textarea demo feature.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseSelect {
    /// The selected fruit value.
    #[get(pub, type(copy))]
    pub(crate) selected_fruit: Signal<String>,
    /// The selected country value.
    #[get(pub, type(copy))]
    pub(crate) selected_country: Signal<String>,
    /// The selected city value.
    #[get(pub, type(copy))]
    pub(crate) selected_city: Signal<String>,
    /// The feedback result message.
    #[get(pub, type(copy))]
    pub(crate) feedback: Signal<String>,
    /// The textarea content.
    #[get(pub, type(copy))]
    pub(crate) textarea_content: Signal<String>,
    /// The textarea validation error message.
    #[get(pub, type(copy))]
    pub(crate) textarea_error: Signal<String>,
}
