/// Represents the available tabs in the conditional rendering demo page.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum ConditionalTab {
    /// The info tab showing general information.
    #[default]
    Info,
    /// The settings tab containing an input field.
    Settings,
    /// The about tab showing framework description.
    About,
}

/// Represents the user role for role-based rendering in the conditional demo page.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum ConditionalUserType {
    /// A guest user with limited access.
    #[default]
    Guest,
    /// A standard user with basic access.
    User,
    /// An administrator with full access.
    Admin,
}
