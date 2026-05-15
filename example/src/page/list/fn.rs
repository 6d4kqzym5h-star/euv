use crate::*;

/// A list rendering demo page with dynamic item management.
///
/// # Returns
///
/// - `VirtualNode`: The list demo page virtual DOM tree.
pub fn page_list() -> VirtualNode {
    let items: Signal<Vec<String>> = use_signal(|| {
        vec![
            "Learn Rust".to_string(),
            "Build a UI framework".to_string(),
            "Write documentation".to_string(),
        ]
    });
    let new_item: Signal<String> = use_signal(|| "".to_string());
    html! {
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
                        value: new_item
                        class: c_list_input()
                        oninput: move |event: NativeEvent| {
                            if let NativeEvent::Input(input_event) = event {
                                new_item.set(input_event.get_value().clone());
                            }
                        }
                    }
                    primary_button {
                        label: "Add"
                        onclick: move |_event: NativeEvent| {
                            let text: String = new_item.get();
                            if !text.trim().is_empty() {
                                let mut current: Vec<String> = items.get();
                                current.push(text.clone());
                                items.set(current);
                                new_item.set("".to_string());
                            }
                        }
                        "Add Item"
                    }
                }
                ul {
                    class: c_list_ul()
                    for (index, item) in {items.get().iter().enumerate()} {
                        li {
                            key: index.to_string()
                            class: if index % 2 == 0 { c_list_item_even() } else { c_list_item_odd() }
                            span {
                                class: c_list_item_text()
                                item.clone()
                            }
                            primary_button {
                                label: "Remove"
                                onclick: move |_event: NativeEvent| {
                                    let mut current: Vec<String> = items.get();
                                    if index < current.len() {
                                        current.remove(index);
                                        items.set(current);
                                    }
                                }
                                "Remove"
                            }
                        }
                    }
                }
            }
        }
    }
}
