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
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RawHtml {
    /// The unescaped HTML content.
    content: String,
}

impl RawHtml {
    /// Creates a new `RawHtml` from a string.
    ///
    /// **Security**: the string is NOT escaped. Do not
    /// pass user-controlled input here.
    pub fn new(content: String) -> Self {
        Self { content }
    }

    /// Returns the raw HTML content (no escaping).
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Returns the content as an owned `String`.
    pub fn into_content(self) -> String {
        self.content
    }

    /// Returns `true` if the content is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Returns the byte length of the content.
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Returns `true` if the content contains any HTML
    /// tag (`<...>`).
    ///
    /// This is a heuristic — it does not parse HTML; it
    /// just looks for `<` followed by some non-`<`
    /// content followed by `>`.
    pub fn contains_tag(&self) -> bool {
        let bytes: &[u8] = self.content.as_bytes();
        let mut i: usize = 0;
        while i < bytes.len() {
            if bytes[i] == b'<' && i + 2 < bytes.len() {
                // Find the matching `>`.
                let mut j: usize = i + 1;
                while j < bytes.len() && bytes[j] != b'>' {
                    j += 1;
                }
                if j < bytes.len() {
                    return true;
                }
            }
            i += 1;
        }
        false
    }
}

impl Default for RawHtml {
    fn default() -> Self {
        Self {
            content: String::new(),
        }
    }
}

impl std::fmt::Display for RawHtml {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.content)
    }
}

impl From<String> for RawHtml {
    fn from(content: String) -> Self {
        Self::new(content)
    }
}

impl From<&str> for RawHtml {
    fn from(content: &str) -> Self {
        Self::new(content.to_string())
    }
}

impl AsRef<str> for RawHtml {
    fn as_ref(&self) -> &str {
        &self.content
    }
}
