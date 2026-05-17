use crate::*;

/// Reactive state for the select and textarea demo feature.
#[derive(Clone, Copy, Data, New)]
pub struct UseSelect {
    /// The selected fruit value.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub selected_fruit: Signal<String>,
    /// The selected country value.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub selected_country: Signal<String>,
    /// The selected city value.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub selected_city: Signal<String>,
    /// The feedback result message.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub feedback: Signal<String>,
    /// The textarea content.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub textarea_content: Signal<String>,
    /// The textarea validation error message.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub textarea_error: Signal<String>,
}
