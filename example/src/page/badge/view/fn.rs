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
            euv_header {
                icon: "🏷️"
                title: "Badge"
                subtitle: "Status indicators with click support."
            }
            euv_card {
                title: "Solid Tags"
                p {
                    class: c_badge_hint()
                    "Status indicators with click support"
                }
                div {
                    class: c_badge_row()
                    euv_tag {
                        color: EuvTagColor::Black
                        variant: EuvTagVariant::Solid
                        text: "Black"
                        on_click: badge_on_click("Black", LogLevel::Log)
                    }
                }
            }
            euv_card {
                title: "Outline Tags"
                div {
                    class: c_badge_row()
                    euv_tag {
                        color: EuvTagColor::White
                        variant: EuvTagVariant::Outline
                        text: "White"
                        on_click: badge_on_click("Outline White", LogLevel::Log)
                    }
                }
            }
        }
    }
}
