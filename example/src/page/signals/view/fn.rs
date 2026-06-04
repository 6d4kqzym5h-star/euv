use crate::*;

/// A reactive signals demo page showcasing counters and badges.
///
/// # Returns
///
/// - `VirtualNode` - The signals demo page virtual DOM tree.
#[component]
pub(crate) fn page_signals(node: VirtualNode<PageSignalsProps>) -> VirtualNode {
    let PageSignalsProps = node.try_get_props().unwrap_or_default();
    let count: Signal<i32> = use_signal(|| 0);
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Reactive Signals"
                subtitle: "Counters and badges driven by reactive state."
            }
            my_card {
                title: "Counter"
                div {
                    class: c_counter_row()
                    div {
                        class: c_counter_text()
                        "Count: "
                        span {
                            id: COUNTER_ID
                            class: c_counter_value()
                            count
                        }
                    }
                    primary_button {
                        label: "Increment"
                        onclick: counter_on_increment(count)
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
                        color: "var(--badge-bg-success)"
                        text: "Success"
                        on_click: badge_on_click("Success", LogLevel::Log)
                    }
                    my_badge {
                        color: "var(--badge-bg-error)"
                        text: "Error"
                        on_click: badge_on_click("Error", LogLevel::Error)
                    }
                    my_badge {
                        color: "var(--badge-bg-warning)"
                        text: "Warning"
                        on_click: badge_on_click("Warning", LogLevel::Warn)
                    }
                    my_badge {
                        color: "var(--badge-bg-info)"
                        text: "Info"
                        on_click: badge_on_click("Info", LogLevel::Log)
                    }
                }
            }
        }
    }
}
