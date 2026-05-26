use crate::*;

/// Error type returned when parsing a string into `NativeEventName` fails.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct ParseNativeEventNameError {
    /// The input string that could not be parsed.
    pub(crate) input: String,
}
