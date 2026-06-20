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
            euv_header {
                icon: "🔢"
                title: "Counter"
                subtitle: "Reactive counter driven by signal state."
            }
            euv_card {
                title: "Counter"
                p {
                    class: c_counter_text()
                    "The current count is "
                    span {
                        id: COUNTER_ID
                        class: c_counter_value()
                        count
                    }
                    " clicks."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Add"
                        onclick: counter_on_increment(count)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Subtract"
                        onclick: counter_on_decrement(count)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Reset"
                        onclick: counter_on_reset(count)
                    }
                }
            }
        }
    }
}
