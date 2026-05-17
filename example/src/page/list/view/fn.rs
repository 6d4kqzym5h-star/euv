use crate::*;

/// A list rendering demo page with dynamic item management.
///
/// # Returns
///
/// - `VirtualNode` - The list demo page virtual DOM tree.
pub fn page_list() -> VirtualNode {
    let state: UseTodoList = use_todo_list();
    html! {
        div {
            class: c_page_container()
            { page_header("List Rendering", "Dynamic list with add and remove operations.") }
            my_card {
                title: "Todo List"
                div {
                    class: c_list_input_row()
                    input {
                        r#type: "text"
                        placeholder: "Enter new item"
                        value: state.get_new_item()
                        class: if { state.get_add_error().get().is_empty() } { c_list_input() } else { c_form_input_error() }
                        oninput: todo_list_on_input_new_item(state)
                    }
                    primary_button {
                        label: "Add"
                        onclick: todo_list_on_add(state)
                        "Add Item"
                    }
                }
                if { !state.get_add_error().get().is_empty() } {
                    p {
                        class: c_field_error_text()
                        state.get_add_error()
                    }
                } else {
                    ""
                }
                ul {
                    class: c_list_ul()
                    for (index, item) in { state.get_items().get().iter().enumerate() } {
                        li {
                            key: index.to_string()
                            class: if { index % 2 == 0 } { c_list_item_even() } else { c_list_item_odd() }
                            span {
                                class: c_list_item_text()
                                item.clone()
                            }
                            primary_button {
                                label: "Remove"
                                onclick: todo_list_on_remove(state.get_items(), index)
                                "Remove"
                            }
                        }
                    }
                }
            }
        }
    }
}
