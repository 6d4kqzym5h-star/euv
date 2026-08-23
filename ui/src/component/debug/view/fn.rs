use super::*;

/// A dev-only component for inspecting reactive state inside the
/// component tree.
///
/// Renders a labelled readout block. The body is the result of
/// invoking the `value` closure on every render, which means
/// callers typically embed `Signal::get()` calls inside `value`
/// to subscribe the rendered vnode to live state changes.
///
/// The component emits `data-euv-debug` (on the outer wrapper),
/// `data-euv-debug-label` (on the label span), and
/// `data-euv-debug-value` (on the value element) so that
/// CSS / dev-tools / e2e tests can target each piece without
/// relying on class names.
///
/// # Arguments
///
/// - `VirtualNode<EuvDebugProps>` - The props node containing label,
///   value closure, and `expanded` flag.
///
/// # Returns
///
/// - `VirtualNode` - A labelled Debug readout element.
#[component]
pub fn euv_debug(node: VirtualNode<EuvDebugProps>) -> VirtualNode {
    let EuvDebugProps {
        label,
        value,
        expanded,
    }: EuvDebugProps = node.try_get_props().unwrap_or_default();
    // Render-time value: invoke the closure each render so that
    // any `Signal::get()` calls inside the closure subscribe the
    // rendered vnode to those signals. The closure is `Rc<dyn Fn>`,
    // not `FnMut`, so we cannot mutate captured state — but for a
    // Debug readout, producing a fresh `String` per render is the
    // right contract anyway.
    let rendered: String = match value.as_ref() {
        Some(formatter) => formatter(),
        // `value: None` means the caller constructed the
        // component without a formatter. Render an explicit
        // placeholder so the dev sees the missing-config issue
        // immediately (rather than an empty Debug box that
        // silently misleads them into thinking the value is
        // empty).
        None => String::from("<no formatter>"),
    };
    if expanded {
        html! {
            div {
                class: c_debug()
                data-euv-debug: "expanded"
                span {
                    class: c_debug_label()
                    data-euv-debug-label: label
                    label
                }
                pre {
                    class: c_debug_value()
                    data-euv-debug-value: "expanded"
                    rendered
                }
            }
        }
    } else {
        html! {
            div {
                class: c_debug()
                data-euv-debug: "inline"
                span {
                    class: c_debug_label()
                    data-euv-debug-label: label
                    label
                }
                code {
                    class: c_debug_value()
                    data-euv-debug-value: "inline"
                    rendered
                }
            }
        }
    }
}
