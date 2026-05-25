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
pub(crate) fn form_input(props: VirtualNode) -> VirtualNode {
    let FormInputProps {
        id,
        label,
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
                label
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
