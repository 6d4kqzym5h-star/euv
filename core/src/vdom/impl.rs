use super::*;

impl Display for RawHtml {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get_content())
    }
}
