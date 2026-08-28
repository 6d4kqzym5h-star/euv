use super::*;

/// A generic left-side mobile drawer with overlay, aligned with common
/// component libraries.
///
/// The drawer and overlay are always mounted and slide/fade via reactive
/// classes driven by the caller-owned `open` signal; clicking the overlay
/// closes the drawer.
///
/// # Arguments
///
/// - `VirtualNode<EuvDrawerProps>` - The props node containing the open signal.
///
/// # Returns
///
/// - `VirtualNode` - The drawer virtual DOM tree.
#[component]
pub fn euv_drawer(node: VirtualNode<EuvDrawerProps>) -> VirtualNode {
    let EuvDrawerProps { open }: EuvDrawerProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_child_node();
    html! {
        div {
            class: c_euv_drawer_overlay()
            class: if { open } {
                c_euv_drawer_overlay_open()
            } else {
                c_euv_drawer_overlay_closed()
            }
            onclick: close_drawer(open)
        }
        div {
            class: c_euv_drawer()
            class: if { open } {
                c_euv_drawer_open()
            } else {
                c_euv_drawer_closed()
            }
            children
        }
    }
}

/// Closes the drawer.
///
/// # Arguments
///
/// - `Signal<bool>` - The drawer-open signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_drawer(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_| open.set(false)))
}
