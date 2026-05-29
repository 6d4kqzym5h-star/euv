#[allow(unused_imports)]
use super::*;

/// The DOM `id` attribute value for the shared `<style>` element used by euv.
///
/// This ID is used to locate or create the `<style>` element in the document `<head>`
/// where all dynamically generated CSS rules are appended.
pub(crate) const EUV_CSS_INJECTED_ID: &str = "euv-css-injected";

/// The HTML `style` tag name used when creating a `<style>` element in the DOM.
pub(crate) const STYLE_TAG: &str = "style";

/// The callback event name used as a default for component prop event handlers.
///
/// When a closure is passed as a component prop via `IntoCallbackAttribute`,
/// it is wrapped with this generic event name. The actual DOM event type is
/// later resolved via `EventAdapter::into_attribute`.
pub(crate) const CALLBACK_EVENT_NAME: &str = "callback";

/// The CSS pseudo-rule serialization separator between selector and style block.
///
/// Used by `Css::parse_pseudo_rules` and `Css::parse_media_rules` to locate
/// the boundary between the selector and the style declarations in the
/// compact serialization format produced by the `class!` macro.
pub(crate) const CSS_RULE_OPEN: &str = " { ";

/// The CSS `@media` rule prefix used in serialized media query strings.
///
/// Used by `Css::parse_media_rules` to identify and extract media query
/// blocks from the compact serialization format produced by the `class!` macro.
pub(crate) const CSS_MEDIA_PREFIX: &str = "@media ";

/// The space character used in class/style name merging.
pub(crate) const CHAR_SPACE: char = ' ';

/// The CSS property separator string (name: value).
pub(crate) const CSS_PROP_SEPARATOR: &str = ": ";

/// The CSS declaration terminator character.
pub(crate) const CHAR_CSS_DECL_TERMINATOR: char = ';';

/// The CSS rule closing brace character.
pub(crate) const CHAR_CSS_RULE_CLOSE: char = '}';

/// The CSS class selector prefix character.
pub(crate) const CHAR_CSS_CLASS_PREFIX: char = '.';

/// The hyphen string used for replacing underscores in kebab-case conversion.
pub(crate) const STR_HYPHEN: &str = "-";

/// The underscore character used in CSS property name conversion.
pub(crate) const CHAR_UNDERSCORE: char = '_';

/// The signal addresses separator character.
pub(crate) const CHAR_SIGNAL_ADDRS_SEPARATOR: char = ',';
