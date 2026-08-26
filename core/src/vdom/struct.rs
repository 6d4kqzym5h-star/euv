//! Raw HTML escape-hatch primitive.
//!
//! `RawHtml` wraps an unprocessed HTML string for
//! advanced users who need to embed markup the `html!`
//! macro cannot express (inline SVG fragments, Markdown-
//! rendered HTML, third-party widget blobs).
//!
//! `RawHtml` does NOT escape its content — embedding
//! untrusted strings here is equivalent to
//! `Element.innerHTML`. The companion macro
//! `unsafe_no_inline!` reinforces this by prefixing the
//! name with `unsafe_no_` so the security implication is
//! loud at the call site.
//!
//! The `content` field is `pub(crate)` with a public
//! getter and a private setter / mut-getter, so the
//! unescaped string is set at construction and read
//! thereafter — there is no way to mutate it from
//! outside the crate.
use super::*;

/// A raw HTML string that the renderer must insert
/// without escaping.
///
/// Constructed via `RawHtml::new` (from Lombok `New`) or,
/// preferably, the `unsafe_no_inline!` macro which makes
/// the security warning loud at the call site.
///
/// The `content` field is `pub(crate)` with a public
/// getter (`get_content`), a crate-internal mut-getter,
/// and a crate-internal setter. The public getter lets
/// the renderer read the unescaped content; the
/// setter / mut-getter are kept crate-internal so the
/// content is set at construction (preferably through
/// `unsafe_no_inline!`) and not mutated after.
#[derive(Clone, Data, Debug, Default, Eq, Hash, PartialEq, New)]
pub struct RawHtml {
    /// The unescaped HTML content.
    #[get(pub)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content: String,
}

impl Display for RawHtml {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get_content())
    }
}
