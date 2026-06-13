use crate::*;

/// A counter demo page showcasing reactive signal-driven state updates.
///
/// # Returns
///
/// - `VirtualNode` - The counter demo page virtual DOM tree.
#[component]
pub(crate) fn page_counter(node: VirtualNode<PageCounterProps>) -> VirtualNode {
    let PageCounterProps = node.try_get_props().unwrap_or_default();
    let count: Signal<i32> = use_signal(|| 0);
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Counter"
                subtitle: "Reactive counter driven by signal state."
            }
            my_card {
                title: "Counter"
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
                    label: "Add"
                    onclick: counter_on_increment(count)
                    "Add"
                }
            }
        }
    }
}
