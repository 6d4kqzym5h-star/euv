use crate::*;

/// Reactive state for a file upload feature.
#[derive(Clone, Copy, Data, New)]
pub struct UseFileUpload {
    /// The names of selected files.
    #[get(pub, type(copy))]
    pub file_names: Signal<Vec<String>>,
    /// The sizes of selected files in bytes.
    #[get(pub, type(copy))]
    pub file_sizes: Signal<Vec<f64>>,
    /// The MIME types of selected files.
    #[get(pub, type(copy))]
    pub file_types: Signal<Vec<String>>,
    /// Whether multiple file selection is enabled.
    #[get(pub, type(copy))]
    pub multiple: Signal<bool>,
    /// The accepted file types filter string.
    #[get(pub, type(copy))]
    pub accept: Signal<String>,
    /// The drag-over active state.
    #[get(pub, type(copy))]
    pub drag_over: Signal<bool>,
    /// The status message.
    #[get(pub, type(copy))]
    pub status: Signal<String>,
}
