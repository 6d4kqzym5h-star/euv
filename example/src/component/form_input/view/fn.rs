use crate::*;

/// A custom input component with label and event handling.
///
/// # Arguments
///
/// - `VirtualNode` - The props node containing label, placeholder, and value.
///
/// # Returns
///
/// - `VirtualNode` - A labeled input element.
#[component]
pub(crate) fn form_input(props: VirtualNode) -> VirtualNode {
    let FormInputProps {
        id,
        label: label_string,
        placeholder,
        value,
        autocomplete,
    }: FormInputProps = props.into();
    html! {
        div {
            class: c_form_input_wrapper()
            label {
                r#for: id.clone()
                class: c_form_label()
                label: label_string
            }
            input {
                id: id.clone()
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
