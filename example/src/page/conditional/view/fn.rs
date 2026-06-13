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
    let _role_cols: Signal<usize> = use_equal_wrap(3, CONDITIONAL_ROLE_BUTTON_ROW_SELECTOR);
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Conditional Rendering"
                subtitle: "Toggle visibility and switch between views."
            }
            my_card {
                title: "Toggle Content"
                primary_button {
                    label: "Toggle"
                    onclick: use_toggle(show_details)
                    "Toggle"
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
                            "This content is conditionally rendered based on the toggle state."
                        }
                        p {
                            class: c_demo_text_muted()
                            "You can use this pattern for modals, accordions, and more."
                        }
                    }
                }
            }
            my_card {
                title: "Role-Based Rendering"
                div {
                    class: format!("{} {}", c_equal_wrap().get_name(), c_role_button_row().get_name())
                    primary_button {
                        label: "Guest"
                        onclick: user_type_on_select(user_type, "guest")
                        "Guest"
                    }
                    primary_button {
                        label: "User"
                        onclick: user_type_on_select(user_type, "user")
                        "User"
                    }
                    primary_button {
                        label: "Admin"
                        onclick: user_type_on_select(user_type, "admin")
                        "Admin"
                    }
                }
                match { user_type.get().as_str() } {
                    "guest" => {
                        div {
                            class: c_role_guest()
                            p {
                                class: c_role_guest_text()
                                "Welcome, guest! Please sign in to access more features."
                            }
                        }
                    }
                    "user" => {
                        div {
                            class: c_role_user()
                            p {
                                class: c_role_user_text()
                                "Hello, user! You have standard access."
                            }
                        }
                    }
                    _ => {
                        div {
                            class: c_role_admin()
                            p {
                                class: c_role_admin_text()
                                "Welcome, administrator! You have full access."
                            }
                        }
                    }
                }
            }
            my_card {
                title: "Tab Switching"
                div {
                    class: c_tab_bar()
                    div {
                        class: if { tab.get() == "info" } { c_tab_item_active() } else { c_tab_item_inactive() }
                        onclick: tab_on_select(tab, "info")
                        "Info"
                    }
                    div {
                        class: if { tab.get() == "settings" } { c_tab_item_active() } else { c_tab_item_inactive() }
                        onclick: tab_on_select(tab, "settings")
                        "Settings"
                    }
                    div {
                        class: if { tab.get() == "about" } { c_tab_item_active() } else { c_tab_item_inactive() }
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
                            form_input {
                                id: SETTINGS_DISPLAY_NAME_ID
                                label: "Display Name"
                                placeholder: SETTINGS_DISPLAY_NAME_PLACEHOLDER
                                value: ""
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
