use crate::*;

/// A reactive signals demo page showcasing counters and badges.
///
/// # Returns
///
/// - `VirtualNode`: The signals demo page virtual DOM tree.
pub fn page_signals() -> VirtualNode {
    let count: Signal<i32> = use_signal(|| 0);
    html! {
        div {
            class: c_page_container()
            div {
                class: c_page_header()
                h1 {
                    class: c_page_title()
                    "Reactive Signals"
                }
                p {
                    class: c_page_subtitle()
                    "Counters and badges driven by reactive state."
                }
            }
            my_card {
                title: "Counter"
                div {
                    class: c_counter_row()
                    div {
                        class: c_counter_text()
                        "Count: "
                        span {
                            id: "counter"
                            class: c_counter_value()
                            count
                        }
                    }
                    primary_button {
                        label: "Increment"
                        onclick: move |_event: NativeEvent| {
                            let current: i32 = count.get();
                            count.set(current + 1);
                        }
                        "Increment"
                    }
                }
            }
            my_card {
                title: "Badges"
                p {
                    class: c_badge_hint()
                    "Status indicators with click support"
                }
                div {
                    class: c_badge_row()
                    my_badge {
                        color: "#059669"
                        text: "Success"
                        on_click: move |_event: NativeEvent| {
                            Console::log("Success badge clicked!");
                        }
                    }
                    my_badge {
                        color: "#dc2626"
                        text: "Error"
                        on_click: move |_event: NativeEvent| {
                            Console::log("Error badge clicked!");
                        }
                    }
                    my_badge {
                        color: "#d97706"
                        text: "Warning"
                    }
                    my_badge {
                        color: "#6b7280"
                        text: "Info"
                    }
                }
            }
        }
    }
}
