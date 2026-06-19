use crate::*;

/// A sticky and CSS effects demo page showcasing position: sticky, glassmorphism,
/// backdrop-filter, gradients, text-shadow, clip-path,
/// and scroll-driven shadow effects.
///
/// # Returns
///
/// - `VirtualNode` - The sticky demo page virtual DOM tree.
#[component]
pub(crate) fn page_sticky(node: VirtualNode<PageStickyProps>) -> VirtualNode {
    let PageStickyProps = node.try_get_props().unwrap_or_default();
    let state: UseSticky = use_sticky();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "📌"
                title: "Sticky & CSS Effects"
                subtitle: "Position sticky, glassmorphism, backdrop-filter, clip-path, and more."
            }
            euv_card {
                title: "Sticky Header"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Toggle"
                        onclick: sticky_on_toggle_sticky(state)
                        if { state.get_sticky_enabled().get() } { "Off" } else { "On" }
                    }
                }
                div {
                    class: c_sticky_scroll_area()
                    div {
                        class: if { state.get_sticky_enabled().get() } { c_sticky_header() } else { c_sticky_header_relative() }
                        STICKY_HEADER_TEXT
                    }
                    for i in { 0..8 } {
                        p {
                            class: c_sticky_paragraph()
                            { format!("{}. {}", i + 1, STICKY_PLACEHOLDER_TEXT) }
                        }
                    }
                }
            }
            euv_card {
                title: "Glassmorphism"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Toggle"
                        onclick: sticky_on_toggle_glass(state)
                        if { state.get_glass_enabled().get() } { "Off" } else { "On" }
                    }
                }
                div {
                    class: c_sticky_glass_demo()
                    div {
                        class: format!("{} {}", c_sticky_glass_circle().get_name(), c_sticky_glass_circle_first().get_name())
                    }
                    div {
                        class: format!("{} {}", c_sticky_glass_circle().get_name(), c_sticky_glass_circle_second().get_name())
                    }
                    div {
                        class: format!("{} {}", c_sticky_glass_circle().get_name(), c_sticky_glass_circle_third().get_name())
                    }
                    div {
                        class: if { state.get_glass_enabled().get() } { c_sticky_glass_card() } else { c_sticky_glass_card_no_blur() }
                        h3 {
                            class: c_sticky_glass_title()
                            "Glassmorphism"
                        }
                        p {
                            class: c_sticky_glass_text()
                            "A frosted glass effect using backdrop-filter and semi-transparent backgrounds."
                        }
                    }
                }
            }
            euv_card {
                title: "Backdrop Blur Control"
                div {
                    class: c_sticky_slider_row()
                    label {
                        class: format!("{} {}", c_sticky_slider_label().get_name(), "c_sticky_slider_blur_label")
                        { format!("{}: {}px", STICKY_BLUR_LABEL, STICKY_BLUR_INITIAL) }
                    }
                    input {
                        r#type: "range"
                        min: STICKY_BLUR_MIN
                        max: STICKY_BLUR_MAX
                        step: STICKY_BLUR_STEP
                        value: STICKY_BLUR_INITIAL.to_string()
                        class: c_sticky_slider()
                        oninput: sticky_on_blur_change()
                    }
                }
                div {
                    class: c_sticky_blur_demo()
                    div {
                        class: c_sticky_blur_background()
                    }
                    div {
                        class: c_sticky_blur_overlay()
                        style: format!("backdrop-filter: blur({}px); -webkit-backdrop-filter: blur({}px)", STICKY_BLUR_INITIAL, STICKY_BLUR_INITIAL)
                        p {
                            class: c_sticky_blur_text()
                            "This text is behind a backdrop-filter blur layer. Adjust the slider to change the blur intensity."
                        }
                    }
                }
            }
            euv_card {
                title: "Conic Gradient"
                div {
                    class: c_sticky_slider_row()
                    label {
                        class: format!("{} {}", c_sticky_slider_label().get_name(), "c_sticky_slider_angle_label")
                        format!("{}: {}°", STICKY_ANGLE_LABEL, STICKY_ANGLE_INITIAL)
                    }
                    input {
                        r#type: "range"
                        min: STICKY_ANGLE_MIN
                        max: STICKY_ANGLE_MAX
                        step: STICKY_ANGLE_STEP
                        value: STICKY_ANGLE_INITIAL.to_string()
                        class: c_sticky_slider()
                        oninput: sticky_on_angle_change()
                    }
                }
                div {
                    class: c_sticky_gradient_demo()
                    div {
                        class: c_sticky_gradient_wheel()
                        style: format!("background: conic-gradient(from {}deg, {}, {}, {}, {}, {})", STICKY_ANGLE_INITIAL, var!(accent), var!(color-success), var!(color-warning), var!(color-error), var!(accent))
                    }
                }
            }
            euv_card {
                title: "Text Shadow"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Toggle"
                        onclick: sticky_on_toggle_text_shadow(state)
                        if { state.get_text_shadow_enabled().get() } { "Off" } else { "On" }
                    }
                }
                div {
                    class: c_sticky_text_demo()
                    p {
                        class: if { state.get_text_shadow_enabled().get() } { c_sticky_text_glow() } else { c_sticky_text_plain() }
                        "Glowing Text"
                    }
                    p {
                        class: if { state.get_text_shadow_enabled().get() } { c_sticky_text_neon() } else { c_sticky_text_plain() }
                        "Neon Effect"
                    }
                    p {
                        class: if { state.get_text_shadow_enabled().get() } { c_sticky_text_emboss() } else { c_sticky_text_plain() }
                        "Emboss Effect"
                    }
                }
            }
            euv_card {
                title: "Clip Path"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Toggle"
                        onclick: sticky_on_toggle_clip_path(state)
                        if { state.get_clip_path_enabled().get() } { "Off" } else { "On" }
                    }
                }
                div {
                    class: c_sticky_clip_demo()
                    div {
                        class: if { state.get_clip_path_enabled().get() } { c_sticky_clip_hexagon() } else { c_sticky_clip_square() }
                        "Hex"
                    }
                    div {
                        class: if { state.get_clip_path_enabled().get() } { c_sticky_clip_diamond() } else { c_sticky_clip_square() }
                        "Dia"
                    }
                    div {
                        class: if { state.get_clip_path_enabled().get() } { c_sticky_clip_star() } else { c_sticky_clip_square() }
                        "Star"
                    }
                    div {
                        class: if { state.get_clip_path_enabled().get() } { c_sticky_clip_message() } else { c_sticky_clip_square() }
                        "Msg"
                    }
                }
            }
            euv_card {
                title: "Scroll Shadow"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Toggle"
                        onclick: sticky_on_toggle_scroll_shadow(state)
                        if { state.get_scroll_shadow_visible().get() } { "Off" } else { "On" }
                    }
                }
                div {
                    class: if { state.get_scroll_shadow_visible().get() } { c_sticky_scroll_shadow() } else { c_sticky_scroll_no_shadow() }
                    for i in { 0..10 } {
                        p {
                            class: c_sticky_paragraph()
                            format!("Scroll item {} - This content demonstrates inner scrolling with a shadow indicator.", i + 1)
                        }
                    }
                }
            }
        }
    }
}
