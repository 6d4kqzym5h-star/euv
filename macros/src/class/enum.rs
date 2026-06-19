/// The key side of a CSS property in a class definition.
///
/// Supports two forms:
/// - `Static`: A compile-time string literal or kebab-case identifier (e.g., `font_size`, `"background"`).
/// - `Dynamic`: A runtime expression wrapped in braces (e.g., `{key_var}`) that evaluates to a string.
#[derive(Clone, Debug)]
pub(crate) enum ClassPropKey {
    /// A static key known at compile time.
    Static(String),
    /// A dynamic expression that evaluates to a key string at runtime.
    Dynamic(proc_macro2::TokenStream),
}

/// The value side of a CSS property in a class definition.
///
/// Always stored as an expression token stream. String literals like `"#fff"`
/// and dynamic expressions (including `var!()` calls) are all parsed as
/// Rust expressions.
#[derive(Clone, Debug)]
pub(crate) enum ClassPropValue {
    /// A dynamic expression (Rust code token stream) that evaluates to a string.
    Expr(proc_macro2::TokenStream),
}

/// The type of CSS at-rule.
///
/// Each variant corresponds to a CSS at-rule and determines
/// how the rule is serialized into CSS output.
#[derive(Clone, Debug)]
pub(crate) enum AtRuleKind {
    /// `@media` — conditional styles based on media queries.
    Media,
    /// `@keyframes` — animation keyframe definitions.
    Keyframes,
    /// `@supports` — conditional styles based on feature support.
    Supports,
    /// `@layer` — cascade layer declaration.
    Layer,
    /// `@container` — container query conditional styles.
    Container,
    /// `@property` — CSS custom property registration.
    Property,
    /// `@scope` — scoped style rules.
    Scope,
    /// `@font-face` — custom font definition.
    FontFace,
    /// `@charset` — character encoding declaration.
    Charset,
    /// `@import` — stylesheet import.
    Import,
    /// `@namespace` — XML namespace declaration.
    Namespace,
    /// `@page` — printed page style rules.
    Page,
    /// `@color-profile` — ICC color profile definition.
    ColorProfile,
    /// `@counter-style` — custom counter style definition.
    CounterStyle,
    /// `@font-feature-values` — OpenType feature values.
    FontFeatureValues,
    /// `@font-palette-values` — font palette override values.
    FontPaletteValues,
    /// `@document` — conditional rules for URL matching (deprecated).
    Document,
    /// `@starting-style` — initial transition styles.
    StartingStyle,
    /// `@view-transition` — view transition configuration.
    ViewTransition,
    /// `@position-try` — position fallback options.
    PositionTry,
    /// `@custom-media` — custom media query aliases (deprecated).
    CustomMedia,
    /// `@function` — CSS function definitions (experimental).
    Function,
}
