use crate::*;

/// A custom input component with label and event handling.
///
/// # Arguments
///
/// - `FormInputProps` - The typed props containing id, label, placeholder, value, autocomplete.
/// - `Vec<VirtualNode>` - The children nodes (unused).
///
/// # Returns
///
/// - `VirtualNode` - A labeled input element.
#[component]
pub(crate) fn form_input(props: FormInputProps, _children: Vec<VirtualNode>) -> VirtualNode {
    let FormInputProps {
        id,
        label: label_string,
        placeholder,
        value,
        autocomplete,
    } = props;
    html! {
        div {
            class: c_form_input_wrapper()
            label {
                r#for: id
                class: c_form_label()
                label_string
            }
            input {
                id: id
                name: id
                r#type: "text"
                placeholder: placeholder
                value: value
                autocomplete: autocomplete
                class: c_form_input()
            }
        }
    }
}
