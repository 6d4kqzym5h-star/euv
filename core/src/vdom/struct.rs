//! Raw HTML escape-hatch primitive.
//!
//! Provides a struct that wraps an unprocessed HTML
//! string for advanced users who need to embed markup
//! the `html!` macro cannot express (e.g., inline SVG
//! fragments, Markdown-rendered HTML, or third-party
//! widget blobs).
//!
//! # Why a separate primitive?
//!
//! The `html!` macro is designed for safe, declarative
//! markup. Every attribute and every child is type-
//! checked at compile time. Sometimes you need to drop
//! down a level — to inject a pre-rendered SVG path
//! from a third-party library, or to embed the result
//! of a Markdown renderer. `RawHtml` is the boundary
//! where the framework stops parsing for you and hands
//! the raw string to the renderer.
//!
//! # XSS warning
//!
//! `RawHtml` does NOT escape its content. Embedding
//! untrusted strings here is equivalent to using
//! `innerHTML` directly. The companion macro
//! `unsafe_no_inline!` reinforces this by prefixing
//! the name with `unsafe_no_` to make the security
//! implications loud at the call site.
//!
//! # Current status
//!
//! This PR ships the primitive and the macro. The
//! follow-up PR will wire `RawHtml` into the renderer
//! so that `RawHtml::into_virtual_node()` actually
//! dispatches to `set_innerHTML` on mount. Until that
//! follow-up lands, `into_virtual_node()` returns a
//! `VirtualNode::Fragment` containing a single
//! `TextNode` (escaped on render), so raw HTML is
//! visible as text until the renderer integration lands.
//!
//! # Example
//!
//! ```ignore
//! use euv::vdom::{RawHtml, VirtualNode};
//!
//! let raw: RawHtml = unsafe_no_inline!("<svg viewBox=\"0 0 10 10\"></svg>");
//! let node: VirtualNode = raw.into_virtual_node();
//! ```
/// A raw HTML string that the renderer must insert
/// without escaping.
///
/// Use this for third-party widget blobs, Markdown-
/// rendered output, or SVG fragments that `html!` cannot
/// express. The string is NOT escaped; treat it like
/// `Element.innerHTML` in JavaScript.
///
/// # Construction
///
/// Construct via the `unsafe_no_inline!` macro. Direct
/// construction via `RawHtml::new` is also supported
/// but bypasses the macro-level `unsafe_no_` warning
/// prefix.
use super::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct RawHtml {
    /// The unescaped HTML content.
    pub(crate) content: String,
}
