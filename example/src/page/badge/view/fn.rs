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
                        color: EuvTagColor::Success
                        variant: EuvTagVariant::Solid
                        text: "Success"
                        on_click: badge_on_click("Success", LogLevel::Log)
                    }
                    euv_tag {
                        color: EuvTagColor::Error
                        variant: EuvTagVariant::Solid
                        text: "Error"
                        on_click: badge_on_click("Error", LogLevel::Error)
                    }
                    euv_tag {
                        color: EuvTagColor::Warning
                        variant: EuvTagVariant::Solid
                        text: "Warning"
                        on_click: badge_on_click("Warning", LogLevel::Warn)
                    }
                    euv_tag {
                        color: EuvTagColor::Info
                        variant: EuvTagVariant::Solid
                        text: "Info"
                        on_click: badge_on_click("Info", LogLevel::Log)
                    }
                    euv_tag {
                        color: EuvTagColor::Purple
                        variant: EuvTagVariant::Solid
                        text: "Purple"
                        on_click: badge_on_click("Purple", LogLevel::Log)
                    }
                }
            }
            euv_card {
                title: "Outline Tags"
                div {
                    class: c_badge_row()
                    euv_tag {
                        color: EuvTagColor::Success
                        variant: EuvTagVariant::Outline
                        text: "Success"
                        on_click: badge_on_click("Outline Success", LogLevel::Log)
                    }
                    euv_tag {
                        color: EuvTagColor::Error
                        variant: EuvTagVariant::Outline
                        text: "Error"
                        on_click: badge_on_click("Outline Error", LogLevel::Error)
                    }
                    euv_tag {
                        color: EuvTagColor::Warning
                        variant: EuvTagVariant::Outline
                        text: "Warning"
                        on_click: badge_on_click("Outline Warning", LogLevel::Warn)
                    }
                    euv_tag {
                        color: EuvTagColor::Info
                        variant: EuvTagVariant::Outline
                        text: "Info"
                        on_click: badge_on_click("Outline Info", LogLevel::Log)
                    }
                    euv_tag {
                        color: EuvTagColor::Purple
                        variant: EuvTagVariant::Outline
                        text: "Purple"
                        on_click: badge_on_click("Outline Purple", LogLevel::Log)
                    }
                }
            }
        }
    }
}
