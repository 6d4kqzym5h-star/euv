use super::*;

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

impl Display for RawHtml {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
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
