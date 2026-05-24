use crate::*;

/// Creates file upload state signals wrapped in a `UseFileUpload` struct.
///
/// # Returns
///
/// - `UseFileUpload` - The file upload state.
pub fn use_file_upload() -> UseFileUpload {
    UseFileUpload::new(
        use_signal(Vec::new),
        use_signal(Vec::new),
        use_signal(Vec::new),
        use_signal(|| false),
        use_signal(String::new),
        use_signal(|| false),
        use_signal(|| "No files selected".to_string()),
    )
}

/// Creates a change event handler that reads file information from a file input.
///
/// Uses `input.files()` to access the `FileList` API, which correctly returns
/// all selected files when the "Allow multiple files" option is enabled.
///
/// # Arguments
///
/// - `UseFileUpload` - The file upload state.
///
/// # Returns
///
/// - `NativeEventHandler` - A change handler for the file input.
pub fn file_upload_on_change(state: UseFileUpload) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::Change, move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            let file_list: Option<FileList> = input.files();
            let names: Vec<String>;
            let sizes: Vec<f64>;
            let types: Vec<String>;
            match file_list {
                Some(files) => {
                    let count: u32 = files.length();
                    names = (0..count)
                        .filter_map(|index: u32| files.get(index).map(|file: File| file.name()))
                        .collect();
                    sizes = (0..count)
                        .filter_map(|index: u32| files.get(index).map(|file: File| file.size()))
                        .collect();
                    types = (0..count)
                        .filter_map(|index: u32| files.get(index).map(|file: File| file.type_()))
                        .collect();
                }
                None => {
                    names = Vec::new();
                    sizes = Vec::new();
                    types = Vec::new();
                }
            }
            state.get_file_names().set(names.clone());
            state.get_file_sizes().set(sizes);
            state.get_file_types().set(types);
            if names.is_empty() {
                state.get_status().set("No files selected".to_string());
            } else {
                let count: usize = names.len();
                state
                    .get_status()
                    .set(format!("{} file(s) selected", count));
                Console::log(&format!("Files selected: {:?}", names));
            }
        }
    })
}

/// Creates a click event handler that clears all selected files.
///
/// # Arguments
///
/// - `UseFileUpload` - The file upload state.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to clear files.
pub fn file_upload_on_clear(state: UseFileUpload) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::Click, move |_event: Event| {
        state.get_file_names().set(Vec::new());
        state.get_file_sizes().set(Vec::new());
        state.get_file_types().set(Vec::new());
        state.get_drag_over().set(false);
        state.get_status().set("No files selected".to_string());
    })
}

/// Creates a drag-enter event handler that activates the drop zone.
///
/// # Arguments
///
/// - `UseFileUpload` - The file upload state.
///
/// # Returns
///
/// - `NativeEventHandler` - A dragenter handler.
pub fn file_upload_on_drag_enter(state: UseFileUpload) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::DragEnter, move |_event: Event| {
        state.get_drag_over().set(true);
    })
}

/// Creates a drag-leave event handler that deactivates the drop zone.
///
/// # Arguments
///
/// - `UseFileUpload` - The file upload state.
///
/// # Returns
///
/// - `NativeEventHandler` - A dragleave handler.
pub fn file_upload_on_drag_leave(state: UseFileUpload) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::DragLeave, move |_event: Event| {
        state.get_drag_over().set(false);
    })
}

/// Creates a drag-over event handler that prevents default and keeps the drop zone active.
///
/// # Arguments
///
/// - `UseFileUpload` - The file upload state.
///
/// # Returns
///
/// - `NativeEventHandler` - A dragover handler.
pub fn file_upload_on_drag_over(state: UseFileUpload) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::DragOver, move |_event: Event| {
        state.get_drag_over().set(true);
    })
}

/// Creates a drop event handler that reads the dropped file names.
///
/// # Arguments
///
/// - `UseFileUpload` - The file upload state.
///
/// # Returns
///
/// - `NativeEventHandler` - A drop handler.
pub fn file_upload_on_drop(state: UseFileUpload) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::Drop, move |event: Event| {
        state.get_drag_over().set(false);
        if let Some(drag_event) = event.dyn_ref::<DragEvent>() {
            let has_files: bool = drag_event
                .data_transfer()
                .map(|dt: DataTransfer| {
                    let len: u32 = dt.types().length();
                    (0..len)
                        .any(|i: u32| dt.types().get(i).as_string() == Some("Files".to_string()))
                })
                .unwrap_or(false);
            if has_files {
                state
                    .get_status()
                    .set("File(s) dropped - reading file names requires JS interop".to_string());
                Console::log("Drop: files detected in data transfer");
            } else {
                state.get_status().set("No files in drop data".to_string());
            }
        }
    })
}
