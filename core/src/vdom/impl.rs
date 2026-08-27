use super::*;

/// Formatting / debug-printing for [`RawHtml`].
impl Display for RawHtml {
    /// Formats the [`RawHtml`] via the supplied formatter.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get_content())
    }
}
