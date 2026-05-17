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
pub fn form_input(props: VirtualNode) -> VirtualNode {
    let FormInputProps {
        label,
        placeholder,
        value,
    }: FormInputProps = props.into();
    html! {
        div {
            class: c_form_input_wrapper()
            label {
                class: c_form_label()
                label
            }
            input {
                r#type: "text"
                placeholder: placeholder
                value: value
                class: c_form_input()
            }
        }
    }
}
