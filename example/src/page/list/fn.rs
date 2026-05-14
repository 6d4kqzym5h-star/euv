use crate::*;

/// A list rendering demo page with dynamic item management.
///
/// # Returns
///
/// - `VirtualNode`: The list demo page virtual DOM tree.
fn render_list(items: Signal<Vec<String>>, items_remove: Signal<Vec<String>>) -> VirtualNode {
    let item_list: Vec<String> = items.get();
    let mut children: Vec<VirtualNode> = Vec::new();
    for (index, item) in item_list.iter().enumerate() {
        let item_clone: String = item.clone();
        let index_clone: usize = index;
        let items_remove_clone: Signal<Vec<String>> = items_remove;
        let bg_color: String = if index_clone.is_multiple_of(2) {
            "#fafbfc".to_string()
        } else {
            "white".to_string()
        };
        let node: VirtualNode = rsx! {
            li {
                key: index_clone.to_string()
                class: c_list_item()
                style: {background: {bg_color};}
                span {
                    class: c_list_item_text()
                    item_clone
                }
                primary_button {
                    label: "Remove"
                    onclick: move |_event: NativeEvent| {
                        let mut current: Vec<String> = items_remove_clone.get();
                        if index_clone < current.len() {
                            current.remove(index_clone);
                            items_remove_clone.set(current);
                        }
                    }
                    "Remove"
                }
            }
        };
        children.push(node);
    }
    VirtualNode::Fragment(children)
}

pub fn page_list() -> VirtualNode {
    let items: Signal<Vec<String>> = use_signal(|| {
        vec![
            "Learn Rust".to_string(),
            "Build a UI framework".to_string(),
            "Write documentation".to_string(),
        ]
    });
    let items_updater: Signal<Vec<String>> = items;
    let items_remove: Signal<Vec<String>> = items;
    let new_item: Signal<String> = use_signal(|| "".to_string());
    let new_item_updater_add: Signal<String> = new_item;
    let new_item_updater_input: Signal<String> = new_item;
    let new_item_read: Signal<String> = new_item;
    rsx! {
        div {
            class: c_page_container()
            div {
                class: c_page_header()
                h1 {
                    class: c_page_title()
                    "List Rendering"
                }
                p {
                    class: c_page_subtitle()
                    "Dynamic list with add and remove operations."
                }
            }
            my_card {
                title: "Todo List"
                div {
                    class: c_list_input_row()
                    input {
                        r#type: "text"
                        placeholder: "Enter new item"
                        value: new_item_read
                        class: c_list_input()
                        oninput: move |event: NativeEvent| {
                            if let NativeEvent::Input(input_event) = event {
                                new_item_updater_input.set(input_event.get_value().clone());
                            }
                        }
                    }
                    primary_button {
                        label: "Add"
                        onclick: move |_event: NativeEvent| {
                            let text: String = new_item_updater_add.get();
                            if !text.trim().is_empty() {
                                let mut current: Vec<String> = items_updater.get();
                                current.push(text.clone());
                                items_updater.set(current);
                                new_item_updater_add.set("".to_string());
                            }
                        }
                        "Add Item"
                    }
                }
                ul {
                    class: c_list_ul()
                    render_list(items, items_remove)
                }
            }
        }
    }
}
