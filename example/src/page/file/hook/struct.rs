use crate::*;

/// Reactive state for a file upload feature.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub(crate) struct UseFileUpload {
    /// The names of selected files.
    #[get(type(copy))]
    pub(crate) file_names: Signal<Vec<String>>,
    /// The sizes of selected files in bytes.
    #[get(type(copy))]
    pub(crate) file_sizes: Signal<Vec<f64>>,
    /// The MIME types of selected files.
    #[get(type(copy))]
    pub(crate) file_types: Signal<Vec<String>>,
    /// Whether multiple file selection is enabled.
    #[get(type(copy))]
    pub(crate) multiple: Signal<bool>,
    /// The accepted file types filter string.
    #[get(type(copy))]
    pub(crate) accept: Signal<String>,
    /// The status message.
    #[get(type(copy))]
    pub(crate) status: Signal<String>,
}
