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
            page_header {
                title: "File Upload"
                subtitle: "File selection, drag-and-drop zone, and file list display."
            }
            my_card {
                title: "File Input"
                div {
                    class: c_form_input_wrapper()
                    label {
                        class: c_form_label()
                        "Select file(s)"
                    }
                    input {
                        id: FILE_UPLOAD_ID
                        name: FILE_UPLOAD_NAME
                        r#type: FILE_INPUT_TYPE
                        autocomplete: FILE_AUTOCOMPLETE_OFF
                        class: c_file_upload_input_hidden()
                        accept: state.get_accept()
                        multiple: state.get_multiple()
                        onchange: file_upload_on_change(state)
                    }
                    primary_button {
                        label: "Choose File"
                        onclick: file_upload_on_select()
                        "Choose File"
                    }
                }
                div {
                    class: c_file_upload_options()
                    div {
                        class: c_form_checkbox_row()
                            input {
                            id: FILE_MULTIPLE_ID
                            name: FILE_MULTIPLE_NAME
                            r#type: FILE_CHECKBOX_TYPE
                            autocomplete: FILE_AUTOCOMPLETE_OFF
                            class: c_form_checkbox()
                            checked: state.get_multiple()
                            onchange: on_change_checked(state.get_multiple())
                        }
                        label {
                            r#for: FILE_MULTIPLE_ID
                            class: c_form_checkbox_label()
                            "Allow multiple files"
                        }
                    }
                    div {
                        class: c_form_input_wrapper()
                        label {
                            r#for: FILE_ACCEPT_ID
                            class: c_form_label()
                            "Accept filter (e.g. .png,.jpg,image/*)"
                        }
                        input {
                            id: FILE_ACCEPT_ID
                        name: FILE_ACCEPT_NAME
                        r#type: FILE_TEXT_TYPE
                        autocomplete: FILE_AUTOCOMPLETE_OFF
                        class: c_form_input_no_transition()
                            placeholder: FILE_ACCEPT_PLACEHOLDER
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
