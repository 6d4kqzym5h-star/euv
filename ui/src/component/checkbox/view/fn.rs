use super::*;

/// A labeled checkbox component with two-way binding via a signal.
///
/// Renders a checkbox input paired with a label, using `c_form_checkbox_row`,
/// `c_form_checkbox`, and `c_form_checkbox_label` styling.
/// The checked state is bound to the provided signal.
///
/// # Arguments
///
/// - `VirtualNode<EuvCheckboxProps>` - The props node containing checkbox configuration.
///
/// # Returns
///
/// - `VirtualNode` - A styled labeled checkbox element.
#[component]
pub fn euv_checkbox(node: VirtualNode<EuvCheckboxProps>) -> VirtualNode {
    let EuvCheckboxProps {
        id,
        name,
        autocomplete,
        checked,
        label: label_text,
    }: EuvCheckboxProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_form_checkbox_row()
            input {
                id: id
                name: name
                type: "checkbox"
                autocomplete: autocomplete
                checked: checked
                class: c_form_checkbox()
                onchange: UseEuvInput::on_change_checked(checked)
            }
            label {
                for: id
                class: c_form_checkbox_label()
                label_text
            }
        }
    }
}
