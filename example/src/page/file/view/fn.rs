use crate::*;

/// Builds the file list display node from selected file names.
///
/// # Arguments
///
/// - `UseFileUpload` - The file upload state.
///
/// # Returns
///
/// - `VirtualNode` - A list of file name items, or `VirtualNode::Empty`.
fn build_file_list(state: UseFileUpload) -> VirtualNode {
    let names: Vec<String> = state.get_file_names().get();
    if names.is_empty() {
        return VirtualNode::Empty;
    }
    let items: Vec<VirtualNode> = names
        .iter()
        .enumerate()
        .map(|tuple: (usize, &String)| {
            let (index, name) = tuple;
            let display_name: String = name.clone();
            html! {
                div {
                    class: c_file_upload_item()
                    span {
                        class: c_file_upload_item_index()
                        { format!("#{}", index + 1) }
                    }
                    span {
                        class: c_file_upload_item_name()
                        display_name
                    }
                }
            }
        })
        .collect();
    VirtualNode::Fragment(items)
}

/// Builds the drag-and-drop zone node.
///
/// # Arguments
///
/// - `UseFileUpload` - The file upload state.
///
/// # Returns
///
/// - `VirtualNode` - The drop zone element.
fn build_drop_zone(state: UseFileUpload) -> VirtualNode {
    let is_drag_over: bool = state.get_drag_over().get();
    html! {
        div {
            class: if { is_drag_over } { c_file_upload_drop_zone_active() } else { c_file_upload_drop_zone() }
            ondragenter: file_upload_on_drag_enter(state)
            ondragleave: file_upload_on_drag_leave(state)
            ondragover: file_upload_on_drag_over(state)
            ondrop: file_upload_on_drop(state)
            span {
                class: c_file_upload_drop_icon()
                "📁"
            }
            p {
                class: c_file_upload_drop_text()
                "Drag & drop files here"
            }
            p {
                class: c_file_upload_drop_hint()
                "or use the file input above"
            }
        }
    }
}

/// A file upload demo page showcasing file selection, drag-and-drop, and file list display.
///
/// # Returns
///
/// - `VirtualNode` - The file upload demo page virtual DOM tree.
#[component]
pub(crate) fn page_file_upload(node: VirtualNode<PageFileUploadProps>) -> VirtualNode {
    let PageFileUploadProps = node.try_get_props().unwrap_or_default();
    let state: UseFileUpload = use_file_upload();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "📁"
                title: "File Upload"
                subtitle: "File selection, drag-and-drop zone, and file list display."
            }
            euv_card {
                title: "File Input"
                input {
                    id: FILE_UPLOAD_ID
                    name: FILE_UPLOAD_NAME
                    type: FILE_INPUT_TYPE
                    autocomplete: FILE_AUTOCOMPLETE_OFF
                    class: c_file_upload_input_hidden()
                    accept: state.get_accept()
                    multiple: state.get_multiple()
                    onchange: file_upload_on_change(state)
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Browse"
                        onclick: file_upload_on_select()
                        "Browse"
                    }
                }
                div {
                    class: c_file_upload_options()
                    euv_checkbox {
                        id: FILE_MULTIPLE_ID
                        name: FILE_MULTIPLE_NAME
                        autocomplete: FILE_AUTOCOMPLETE_OFF
                        checked: state.get_multiple()
                        label: "Allow multiple files"
                    }
                    euv_field {
                        id: FILE_ACCEPT_ID
                        name: FILE_ACCEPT_NAME
                        label: "Accept filter"
                        input_type: FILE_TEXT_TYPE
                        placeholder: FILE_ACCEPT_PLACEHOLDER
                        autocomplete: FILE_AUTOCOMPLETE_OFF
                        value: state.get_accept()
                        error: None
                    }
                }
                p {
                    class: c_event_result()
                    "Status: "
                    span {
                        class: c_event_highlight()
                        state.get_status()
                    }
                }
                build_file_list(state)
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Clear"
                        onclick: file_upload_on_clear(state)
                        "Clear"
                    }
                }
            }
            euv_card {
                title: "Drag & Drop Zone"
                build_drop_zone(state)
                p {
                    class: c_event_result()
                    "Drag over: "
                    span {
                        class: c_event_highlight()
                        state.get_drag_over()
                    }
                }
            }
        }
    }
}
