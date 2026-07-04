use crate::*;

/// A conditional rendering demo page.
///
/// # Returns
///
/// - `VirtualNode` - The conditional demo page virtual DOM tree.
#[component]
pub(crate) fn page_conditional(node: VirtualNode<PageConditionalProps>) -> VirtualNode {
    let PageConditionalProps = node.try_get_props().unwrap_or_default();
    let show_details: Signal<bool> = use_signal(|| false);
    let user_type: Signal<String> = use_signal(|| "guest".to_string());
    let tab: Signal<String> = use_signal(|| "info".to_string());
    let display_name: Signal<String> = use_signal(|| "".to_string());
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🔀"
                title: "Conditional Rendering"
                subtitle: "Toggle element visibility with if expressions, render different content based on role with match, and implement tab switching — all driven by Signal state."
            }
            euv_card {
                title: "Toggle Content"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { show_details.get() } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: "Toggle"
                        onclick: use_toggle(show_details)
                    }
                }
                if { show_details.get() } {
                    div {
                        class: c_toggle_content()
                        h4 {
                            class: c_toggle_title()
                            "Hidden Details"
                        }
                        p {
                            class: c_demo_text()
                            "This content is conditionally rendered using an if expression bound to a Signal. When the Signal is false, the node is removed from the Virtual DOM entirely."
                        }
                        p {
                            class: c_demo_text_muted()
                            "Use this pattern for modals, accordions, feature flags, permission-based UI, and any scenario where content should appear or disappear reactively."
                        }
                    }
                }
            }
            euv_card {
                title: "Role-Based Rendering"
                div {
                    class: c_role_button_row()
                    euv_button {
                        variant: if { user_type.get() == "guest" } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: "Guest"
                        onclick: user_type_on_select(user_type, "guest")
                    }
                    euv_button {
                        variant: if { user_type.get() == "user" } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: "User"
                        onclick: user_type_on_select(user_type, "user")
                    }
                    euv_button {
                        variant: if { user_type.get() == "admin" } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: "Admin"
                        onclick: user_type_on_select(user_type, "admin")
                    }
                }
                match { user_type.get().as_str() } {
                    "guest" => {
                        div {
                            p {
                                class: c_role_guest_text()
                                "Welcome, guest! Please sign in to access more features."
                            }
                        }
                    }
                    "user" => {
                        div {
                            p {
                                class: c_role_user_text()
                                "Hello, user! You have standard access."
                            }
                        }
                    }
                    _ => {
                        div {
                            p {
                                class: c_role_admin_text()
                                "Welcome, administrator! You have full access."
                            }
                        }
                    }
                }
            }
            euv_card {
                title: "Tab Switching"
                div {
                    class: c_tab_bar()
                    div {
                        class: if { tab.get() == "info" } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: tab_on_select(tab, "info")
                        "Info"
                    }
                    div {
                        class: if { tab.get() == "settings" } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: tab_on_select(tab, "settings")
                        "Settings"
                    }
                    div {
                        class: if { tab.get() == "about" } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: tab_on_select(tab, "about")
                        "About"
                    }
                }
                match { tab.get().as_str() } {
                    "info" => {
                        div {
                            class: c_tab_content()
                            p {
                                class: c_tab_text()
                                "This is the information tab."
                            }
                            p {
                                class: c_tab_text_muted()
                                "Here you can find general details about the application."
                            }
                        }
                    }
                    "settings" => {
                        div {
                            class: c_tab_content()
                            p {
                                class: c_tab_text_input()
                                "This is the settings tab."
                            }
                            euv_input {
                                id: SETTINGS_DISPLAY_NAME_ID
                                label: "Display Name"
                                placeholder: SETTINGS_DISPLAY_NAME_PLACEHOLDER
                                value: display_name
                                autocomplete: CONDITIONAL_AUTOCOMPLETE_NAME
                            }
                        }
                    }
                    _ => {
                        div {
                            class: c_tab_content()
                            p {
                                class: c_tab_text()
                                "This is the about tab."
                            }
                            p {
                                class: c_tab_text_muted()
                                "euv is a declarative UI framework for Rust."
                            }
                        }
                    }
                }
            }
        }
    }
}
