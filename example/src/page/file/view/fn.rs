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
        .map(|(index, name)| {
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
            ondrag_enter: file_upload_on_drag_enter(state)
            ondrag_leave: file_upload_on_drag_leave(state)
            ondrag_over: file_upload_on_drag_over(state)
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
pub fn page_file_upload() -> VirtualNode {
    let state: UseFileUpload = use_file_upload();
    html! {
        div {
            class: c_page_container()
            { page_header("File Upload", "File selection, drag-and-drop zone, and file list display.") }
            my_card {
                title: "File Input"
                div {
                    class: c_form_input_wrapper()
                    label {
                        class: c_form_label()
                        "Select file(s)"
                    }
                    input {
                        r#type: "file"
                        class: c_file_upload_input()
                        accept: state.get_accept()
                        multiple: state.get_multiple()
                        onchange: file_upload_on_change(state)
                    }
                }
                div {
                    class: c_file_upload_options()
                    div {
                        class: c_form_checkbox_row()
                        input {
                            r#type: "checkbox"
                            class: c_form_checkbox()
                            checked: state.get_multiple()
                            onchange: on_change_checked(state.get_multiple())
                        }
                        label {
                            class: c_form_checkbox_label()
                            "Allow multiple files"
                        }
                    }
                    div {
                        class: c_form_input_wrapper()
                        label {
                            class: c_form_label()
                            "Accept filter (e.g. .png,.jpg,image/*)"
                        }
                        input {
                            r#type: "text"
                            class: c_form_input_no_transition()
                            placeholder: ".png,.jpg,image/*"
                            value: state.get_accept()
                            oninput: on_input_value(state.get_accept())
                        }
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
                primary_button {
                    label: "Clear"
                    onclick: file_upload_on_clear(state)
                    "Clear"
                }
            }
            my_card {
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
