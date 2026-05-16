use crate::*;

/// A conditional rendering demo page.
///
/// # Returns
///
/// - `VirtualNode`: The conditional demo page virtual DOM tree.
pub fn page_conditional() -> VirtualNode {
    let show_details: Signal<bool> = use_signal(|| false);
    let user_type: Signal<String> = use_signal(|| "guest".to_string());
    let tab: Signal<String> = use_signal(|| "info".to_string());
    html! {
        div {
            class: c_page_container()
            div {
                class: c_page_header()
                h1 {
                    class: c_page_title()
                    "Conditional Rendering"
                }
                p {
                    class: c_page_subtitle()
                    "Toggle visibility and switch between views."
                }
            }
            my_card {
                title: "Toggle Content"
                primary_button {
                    label: "Toggle"
                    onclick: move |_event: NativeEvent| {
                        let current: bool = show_details.get();
                        show_details.set(!current);
                    }
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
                } else {
                    ""
                }
            }
            my_card {
                title: "Role-Based Rendering"
                div {
                    class: c_role_button_row()
                    primary_button {
                        label: "Guest"
                        onclick: move |_event: NativeEvent| {
                            user_type.set("guest".to_string());
                        }
                        "Guest"
                    }
                    primary_button {
                        label: "User"
                        onclick: move |_event: NativeEvent| {
                            user_type.set("user".to_string());
                        }
                        "User"
                    }
                    primary_button {
                        label: "Admin"
                        onclick: move |_event: NativeEvent| {
                            user_type.set("admin".to_string());
                        }
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
                        style: { padding: "10px 20px"; cursor: "pointer"; border-bottom: { if { tab.get() == "info" } { "2px solid #4f46e5".to_string() } else { "2px solid transparent".to_string() } }; color: { if { tab.get() == "info" } { "#4f46e5".to_string() } else { "inherit".to_string() } }; background: { if { tab.get() == "info" } { "rgba(79, 70, 229, 0.08)".to_string() } else { "transparent".to_string() } }; border-radius: "6px 6px 0 0"; font-size: "14px"; font-weight: "500"; }
                        onclick: move |_event: NativeEvent| { tab.set("info".to_string()); }
                        "Info"
                    }
                    div {
                        style: { padding: "10px 20px"; cursor: "pointer"; border-bottom: { if { tab.get() == "settings" } { "2px solid #4f46e5".to_string() } else { "2px solid transparent".to_string() } }; color: { if { tab.get() == "settings" } { "#4f46e5".to_string() } else { "inherit".to_string() } }; background: { if { tab.get() == "settings" } { "rgba(79, 70, 229, 0.08)".to_string() } else { "transparent".to_string() } }; border-radius: "6px 6px 0 0"; font-size: "14px"; font-weight: "500"; }
                        onclick: move |_event: NativeEvent| { tab.set("settings".to_string()); }
                        "Settings"
                    }
                    div {
                        style: { padding: "10px 20px"; cursor: "pointer"; border-bottom: { if { tab.get() == "about" } { "2px solid #4f46e5".to_string() } else { "2px solid transparent".to_string() } }; color: { if { tab.get() == "about" } { "#4f46e5".to_string() } else { "inherit".to_string() } }; background: { if { tab.get() == "about" } { "rgba(79, 70, 229, 0.08)".to_string() } else { "transparent".to_string() } }; border-radius: "6px 6px 0 0"; font-size: "14px"; font-weight: "500"; }
                        onclick: move |_event: NativeEvent| { tab.set("about".to_string()); }
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
                                label: "Display Name"
                                placeholder: "Enter your name"
                                value: ""
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
