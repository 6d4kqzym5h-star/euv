use crate::*;

/// A badge demo page showcasing status indicators with click support.
///
/// # Returns
///
/// - `VirtualNode` - The badge demo page virtual DOM tree.
#[component]
pub(crate) fn page_badge(node: VirtualNode<PageBadgeProps>) -> VirtualNode {
    let PageBadgeProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Badge"
                subtitle: "Status indicators with click support."
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
                        color: var!(badge-bg-success)
                        text: "Success"
                        on_click: badge_on_click("Success", LogLevel::Log)
                    }
                    my_badge {
                        color: var!(badge-bg-error)
                        text: "Error"
                        on_click: badge_on_click("Error", LogLevel::Error)
                    }
                    my_badge {
                        color: var!(badge-bg-warning)
                        text: "Warning"
                        on_click: badge_on_click("Warning", LogLevel::Warn)
                    }
                    my_badge {
                        color: var!(badge-bg-info)
                        text: "Info"
                        on_click: badge_on_click("Info", LogLevel::Log)
                    }
                }
            }
            my_card {
                title: "Outline Badges"
                div {
                    class: c_badge_row()
                    my_badge {
                        color: var!(badge-bg-success)
                        text: "Success"
                        outline: true
                        on_click: badge_on_click("Outline Success", LogLevel::Log)
                    }
                    my_badge {
                        color: var!(badge-bg-error)
                        text: "Error"
                        outline: true
                        on_click: badge_on_click("Outline Error", LogLevel::Error)
                    }
                    my_badge {
                        color: var!(badge-bg-warning)
                        text: "Warning"
                        outline: true
                        on_click: badge_on_click("Outline Warning", LogLevel::Warn)
                    }
                    my_badge {
                        color: var!(badge-bg-info)
                        text: "Info"
                        outline: true
                        on_click: badge_on_click("Outline Info", LogLevel::Log)
                    }
                }
            }
        }
    }
}
