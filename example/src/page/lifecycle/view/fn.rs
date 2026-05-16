use crate::*;

/// A lifecycle demo page showing mount and update tracking.
///
/// # Returns
///
/// - `VirtualNode`: The lifecycle demo page virtual DOM tree.
pub fn page_lifecycle() -> VirtualNode {
    let render_count: Signal<i32> = use_signal(|| 1);
    let logs: Signal<Vec<String>> = use_signal(|| vec!["Component mounted".to_string()]);
    watch!(render_count, |render_count_value| {
        Console::log(&format!(
            "watch! render count changed: {}",
            render_count_value
        ));
    });
    let increment_and_log: Rc<dyn Fn()> = Rc::new(move || {
        let current: i32 = render_count.get();
        render_count.set(current + 1);
        let mut current_logs: Vec<String> = logs.get();
        current_logs.push(format!("Updated: render count = {}", current + 1));
        logs.set(current_logs);
    });
    html! {
        div {
            class: c_page_container()
            { page_header("Lifecycle", "Track component updates and lifecycle events.") }
            my_card {
                title: "Render Counter"
                p {
                    class: c_render_count_text()
                    "This page has been rendered "
                    span {
                        class: c_counter_value()
                        render_count
                    }
                    " times."
                }
                primary_button {
                    label: "Trigger Update"
                    onclick: move |_event: NativeEvent| {
                        increment_and_log();
                    }
                    "Trigger Update"
                }
            }
            my_card {
                title: "Event Log"
                div {
                    class: c_log_container()
                    for (index, log) in { logs.get().iter().enumerate() } {
                        div {
                            key: index.to_string()
                            class: c_log_item()
                            style: { color: if { index == 0 } { "#059669".to_string() } else { "#6b7280".to_string() }; font-weight: if { index == 0 } { "600".to_string() } else { "400".to_string() }; }
                            log.clone()
                        }
                    }
                }
            }
        }
    }
}
