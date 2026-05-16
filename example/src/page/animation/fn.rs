use crate::*;

/// An animation demo page showcasing CSS animations and transitions.
///
/// # Returns
///
/// - `VirtualNode`: The animation demo page virtual DOM tree.
pub fn page_animation() -> VirtualNode {
    let box_visible: Signal<bool> = use_signal(|| false);
    let spin_active: Signal<bool> = use_signal(|| false);
    let pulse_active: Signal<bool> = use_signal(|| false);
    let progress_value: Signal<i32> = use_signal(|| 0);
    let color_index: Signal<i32> = use_signal(|| 0);
    let scale_active: Signal<bool> = use_signal(|| false);
    html! {
        div {
            class: c_page_container()
            div {
                class: c_page_header()
                h1 {
                    class: c_page_title()
                    "Animation"
                }
                p {
                    class: c_page_subtitle()
                    "CSS transitions, keyframe animations, and reactive style changes."
                }
            }
            my_card {
                title: "Fade In / Out"
                primary_button {
                    label: "Toggle"
                    onclick: move |_event: NativeEvent| {
                        let current: bool = box_visible.get();
                        box_visible.set(!current);
                    }
                    "Toggle Visibility"
                }
                if { box_visible.get() } {
                    div {
                        class: c_anim_fade_in()
                        "This element fades in and out with a smooth transition."
                    }
                } else {
                    ""
                }
            }
            my_card {
                title: "Spinning Element"
                primary_button {
                    label: "Toggle"
                    onclick: move |_event: NativeEvent| {
                        let current: bool = spin_active.get();
                        spin_active.set(!current);
                    }
                    if {spin_active.get()} { "Stop Spin" } else { "Start Spin" }
                }
                div {
                    class: c_anim_spin_container()
                    div {
                        class: if spin_active.get() { c_anim_spin() } else { c_anim_spin_stopped() }
                        "⟳"
                    }
                }
            }
            my_card {
                title: "Pulse Effect"
                primary_button {
                    label: "Toggle"
                    onclick: move |_event: NativeEvent| {
                        let current: bool = pulse_active.get();
                        pulse_active.set(!current);
                    }
                    if {pulse_active.get()} { "Stop Pulse" } else { "Start Pulse" }
                }
                div {
                    class: c_anim_pulse_container()
                    div {
                        class: if pulse_active.get() { c_anim_pulse() } else { c_anim_pulse_stopped() }
                        "♥"
                    }
                }
            }
            my_card {
                title: "Progress Bar"
                div {
                    class: c_timer_controls()
                    primary_button {
                        label: "Start"
                        onclick: move |_event: NativeEvent| {
                            progress_value.set(0);
                            let progress: Signal<i32> = progress_value;
                            let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                                let current: i32 = progress.get();
                                if current < 100 {
                                    progress.set(current + 1);
                                }
                            }));
                            let window: Window = window().expect("no global window exists");
                            let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
                                closure.as_ref().unchecked_ref(),
                                30,
                            );
                            closure.forget();
                        }
                        "Start"
                    }
                    primary_button {
                        label: "Reset"
                        onclick: move |_event: NativeEvent| {
                            progress_value.set(0);
                        }
                        "Reset"
                    }
                }
                div {
                    class: c_progress_container()
                    div {
                        class: c_progress_bar()
                        style: {width: format!("{}%", progress_value.get()); transition: "width 0.1s ease"; background: if progress_value.get() >= 100 { "#059669".to_string() } else { "#4f46e5".to_string() }; height: "100%"; border-radius: "999px"; transition: "all 0.3s ease";}
                    }
                }
                p {
                    class: c_progress_text()
                    progress_value
                    "%"
                }
            }
            my_card {
                title: "Color Cycle"
                primary_button {
                    label: "Next"
                    onclick: move |_event: NativeEvent| {
                        let current: i32 = color_index.get();
                        color_index.set((current + 1) % 5);
                    }
                    "Next Color"
                }
                div {
                    class: c_anim_color_box()
                    style: { background: get_anim_color(color_index.get()); transition: "background 0.5s ease, transform 0.3s ease"; transform: if scale_active.get() { "scale(1.1)" } else { "scale(1)" }; }
                    onclick: move |_event: NativeEvent| {
                        let current: bool = scale_active.get();
                        scale_active.set(!current);
                    }
                    "Click me to scale!"
                }
            }
        }
    }
}

/// Returns a color string based on the animation color index.
///
/// # Arguments
///
/// - `i32`: The color index (0-4).
///
/// # Returns
///
/// - `String`: The CSS color string.
fn get_anim_color(index: i32) -> String {
    let colors: Vec<String> = vec![
        "#4f46e5".to_string(),
        "#059669".to_string(),
        "#d97706".to_string(),
        "#dc2626".to_string(),
        "#7c3aed".to_string(),
    ];
    let safe_index: usize = if index >= 0 && (index as usize) < colors.len() {
        index as usize
    } else {
        0
    };
    colors[safe_index].clone()
}
