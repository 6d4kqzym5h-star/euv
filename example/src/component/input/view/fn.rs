use crate::*;

/// A custom input component with label and event handling.
///
/// # Arguments
///
/// - `EuvInputProps` - The typed props containing id, label, placeholder, value, autocomplete.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - A labeled input element.
#[component]
pub(crate) fn euv_input(node: VirtualNode<EuvInputProps>) -> VirtualNode {
    let EuvInputProps {
        id,
        label: label_string,
        placeholder,
        value,
        autocomplete,
    }: EuvInputProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_euv_input_wrapper()
            label {
                for: id
                class: c_form_label()
                label_string
            }
            input {
                id: id
                name: id
                type: "text"
                placeholder: placeholder
                value: value
                autocomplete: autocomplete
                class: c_euv_input()
            }
        }
    }
}
