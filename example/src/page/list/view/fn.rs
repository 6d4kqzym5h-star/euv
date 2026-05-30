use crate::*;

/// A list rendering demo page with dynamic item management.
///
/// # Returns
///
/// - `VirtualNode` - The list demo page virtual DOM tree.
#[component]
pub(crate) fn page_list(props: PageListProps) -> VirtualNode {
    let PageListProps = props;
    let state: UseTodoList = use_todo_list();
    html! {
        div {
            class: c_page_container()
            page_header("List Rendering", "Dynamic list with add and remove operations.")
            my_card {
                title: "Todo List"
                div {
                    class: c_list_input_row()
                    input {
                        id: LIST_NEW_ITEM_ID
                        name: LIST_NEW_ITEM_NAME
                        r#type: LIST_TEXT_TYPE
                        autocomplete: LIST_AUTOCOMPLETE_OFF
                        placeholder: LIST_NEW_ITEM_PLACEHOLDER
                        value: state.get_new_item()
                        class: if { state.get_add_error().get().is_empty() } { c_list_input() } else { c_list_input_error() }
                        oninput: todo_list_on_input_new_item(state)
                    }
                    button {
                        class: c_primary_button()
                        class: c_list_item_button()
                        onclick: todo_list_on_add(state)
                        "Add Item"
                    }
                }
                if { !state.get_add_error().get().is_empty() } {
                    p {
                        class: c_field_error_text()
                        state.get_add_error()
                    }
                }
                ul {
                    class: c_list_ul()
                    data_list_container: "true"
                    for (index, item) in { state.get_items().get().iter().enumerate() } {
                        li {
                            key: index.to_string()
                            class: if { index % 2 == 0 } { c_list_item_even() } else { c_list_item_odd() }
                            data_index: index.to_string()
                            span {
                                class: c_list_item_text()
                                item.clone()
                            }
                            button {
                                class: c_primary_button()
                                class: c_list_item_button()
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
