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
/// # Arguments
///
/// - `UseFileUpload` - The file upload state.
///
/// # Returns
///
/// - `NativeEventHandler` - A change handler for the file input.
pub fn file_upload_on_change(state: UseFileUpload) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Change, move |event: NativeEvent| {
        if let NativeEvent::Change(change_event) = event {
            let value: String = change_event.get_value().clone();
            let names: Vec<String> = if value.is_empty() {
                Vec::new()
            } else {
                value
                    .split(',')
                    .map(|s| {
                        s.trim()
                            .split('\\')
                            .next_back()
                            .unwrap_or("")
                            .split('/')
                            .next_back()
                            .unwrap_or("")
                            .to_string()
                    })
                    .filter(|s| !s.is_empty())
                    .collect()
            };
            state.get_file_names().set(names.clone());
            if names.is_empty() {
                state.get_file_sizes().set(Vec::new());
                state.get_file_types().set(Vec::new());
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
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
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
    NativeEventHandler::new(NativeEventName::DragEnter, move |_event: NativeEvent| {
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
    NativeEventHandler::new(NativeEventName::DragLeave, move |_event: NativeEvent| {
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
    NativeEventHandler::new(NativeEventName::DragOver, move |_event: NativeEvent| {
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
    NativeEventHandler::new(NativeEventName::Drop, move |event: NativeEvent| {
        state.get_drag_over().set(false);
        if let NativeEvent::Drag(drag_event) = event {
            let types: Vec<String> = drag_event.get_types().clone();
            let has_files: bool = types.iter().any(|t| t == "Files");
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
