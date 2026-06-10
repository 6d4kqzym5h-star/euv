use crate::*;

/// A custom input component with label and event handling.
///
/// # Arguments
///
/// - `FormInputProps` - The typed props containing id, label, placeholder, value, autocomplete.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - A labeled input element.
#[component]
pub(crate) fn form_input(node: VirtualNode<FormInputProps>) -> VirtualNode {
    let FormInputProps {
        id,
        label: label_string,
        placeholder,
        value,
        autocomplete,
    }: FormInputProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_form_input_wrapper()
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
                class: c_form_input()
            }
        }
    }
}
