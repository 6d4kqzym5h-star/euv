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
    let label_text: String = props
        .try_get_prop(&Attribute::Other("label".to_string()))
        .unwrap_or_default();
    let placeholder: String = props
        .try_get_prop(&Attribute::Placeholder)
        .unwrap_or_default();
    let value: String = props.try_get_prop(&Attribute::Value).unwrap_or_default();
    html! {
        div {
            class: c_form_input_wrapper()
            label {
                class: c_form_label()
                label_text
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
