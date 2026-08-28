use super::*;

/// One action button of the [`euv_hero`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvHeroAction {
    /// The button text.
    #[get(type(copy))]
    pub text: &'static str,
    /// The target route or external URL.
    #[get(type(copy))]
    pub link: &'static str,
    /// Whether this is the primary (solid) button style.
    #[get(type(copy))]
    pub primary: bool,
}

/// Props for the [`euv_hero`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvHeroProps {
    /// The main title.
    #[get(type(copy))]
    pub title: &'static str,
    /// The tagline rendered under the title (skipped when empty).
    #[get(type(copy))]
    pub subtitle: &'static str,
    /// The action buttons (skipped when empty).
    #[get(type(copy))]
    pub actions: &'static [EuvHeroAction],
}

/// One card of the [`euv_feature_grid`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvFeature {
    /// The feature icon (emoji, skipped when empty).
    #[get(type(copy))]
    pub icon: &'static str,
    /// The feature title.
    #[get(type(copy))]
    pub title: &'static str,
    /// The feature details.
    #[get(type(copy))]
    pub details: &'static str,
}

/// Props of the [`euv_hero_action`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvHeroActionProps {
    /// The hero action to render.
    pub action: EuvHeroAction,
}

/// Props for the [`euv_feature_grid`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvFeatureGridProps {
    /// The feature cards (skipped when empty).
    #[get(type(copy))]
    pub features: &'static [EuvFeature],
}
