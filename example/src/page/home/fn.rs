use crate::*;

/// The home page component.
///
/// # Returns
///
/// - `VirtualNode`: The home page virtual DOM tree.
pub fn page_home() -> VirtualNode {
    let count: Signal<i32> = use_signal(|| 0);
    let count_updater: Signal<i32> = count;
    let theme: Signal<String> = use_signal(|| "light".to_string());
    let theme_updater: Signal<String> = theme;
    let theme_read: Signal<String> = theme;
    let is_dark: bool = theme_read.get() == "dark";
    let bg_color: String = if is_dark {
        "#1a1a2e".to_string()
    } else {
        "#ffffff".to_string()
    };
    let text_color: String = if is_dark {
        "#e0e0e0".to_string()
    } else {
        "#1a1a2e".to_string()
    };
    let theme_label: String = if is_dark {
        "☀ Light".to_string()
    } else {
        "🌙 Dark".to_string()
    };
    rsx! {
        div {
            class: c_page_container()
            style: {background: {bg_color}; color: {text_color}; transition: "all 0.3s ease"; min_height: "100vh"; padding: "32px 24px";}
            div {
                class: c_page_header()
                h1 {
                    class: c_page_title()
                    "euv Component System"
                }
                p {
                    class: c_page_subtitle()
                    "This example demonstrates reactive signals for attributes and text."
                }
            }
            my_card {
                title: "Counter Component"
                div {
                    class: c_counter_row()
                    div {
                        class: c_counter_text()
                        "Count: "
                        span {
                            id: "counter"
                            class: c_counter_value()
                            count
                        }
                    }
                    primary_button {
                        label: "Increment"
                        onclick: move |_event: NativeEvent| {
                            let current: i32 = count_updater.get();
                            count_updater.set(current + 1);
                        }
                        "Click to Count"
                    }
                    primary_button {
                        label: "Toggle Theme"
                        onclick: move |_event: NativeEvent| {
                            let current: String = theme_updater.get();
                            if current == "light" {
                                theme_updater.set("dark".to_string());
                            } else {
                                theme_updater.set("light".to_string());
                            }
                        }
                        theme_label
                    }
                }
            }
            my_card {
                title: "Badge Components"
                p {
                    class: c_badge_hint()
                    "Status indicators with click support"
                }
                div {
                    class: c_badge_row()
                    my_badge {
                        color: "#059669"
                        text: "Success"
                        on_click: move |_event: NativeEvent| {
                            web_sys::console::log_1(&"Success badge clicked!".into());
                        }
                    }
                    my_badge {
                        color: "#dc2626"
                        text: "Error"
                        on_click: move |_event: NativeEvent| {
                            web_sys::console::log_1(&"Error badge clicked!".into());
                        }
                    }
                    my_badge {
                        color: "#d97706"
                        text: "Warning"
                    }
                    my_badge {
                        color: "#6b7280"
                        text: "Info"
                    }
                }
            }
            my_card {
                title: "Form Components"
                div {
                    class: c_form_grid()
                    form_input {
                        label: "Username"
                        placeholder: "Enter your username"
                        value: ""
                    }
                    form_input {
                        label: "Email"
                        placeholder: "Enter your email"
                        value: ""
                    }
                }
            }
            my_card {
                title: "Custom Attributes Demo"
                div {
                    data_role: "container"
                    data_id: "12345"
                    aria_label: "Demo section"
                    class: c_custom_attrs_demo()
                    p {
                        class: c_demo_text()
                        "This div has custom data-* and aria-* attributes."
                    }
                    p {
                        class: c_demo_text_muted()
                        "Inspect the element to see data-role, data-id, and aria-label."
                    }
                }
            }
            my_card {
                title: "Nested Components"
                primary_button {
                    my_badge {
                        color: "#7c3aed"
                        text: "Nested Badge in Button"
                    }
                }
            }
        }
    }
}
