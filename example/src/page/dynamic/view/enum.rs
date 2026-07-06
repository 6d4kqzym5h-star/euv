/// Represents the available tag types for the dynamic tag demo page.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum DynamicTagType {
    /// A native div element.
    #[default]
    Div,
    /// A native span element.
    Span,
    /// The euv_card user component.
    EuvCard,
    /// The badge user component.
    Badge,
}
