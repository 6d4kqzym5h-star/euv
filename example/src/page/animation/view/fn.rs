use crate::*;

/// An animation demo page showcasing CSS animations and transitions.
///
/// # Returns
///
/// - `VirtualNode` - The animation demo page virtual DOM tree.
pub(crate) fn page_animation() -> VirtualNode {
    let box_visible: Signal<bool> = use_signal(|| false);
    let spin_active: Signal<bool> = use_signal(|| false);
    let pulse_active: Signal<bool> = use_signal(|| false);
    let progress: UseProgress = use_progress();
    let color_index: Signal<i32> = use_signal(|| 0);
    let scale_active: Signal<bool> = use_signal(|| false);
    let progress_handle: Signal<Option<IntervalHandle>> = progress.get_handle();
    use_cleanup(move || {
        if let Some(handle) = progress_handle.get() {
            handle.clear();
        }
    });
    html! {
        div {
            class: c_page_container()
            page_header("Animation", "CSS transitions, keyframe animations, and reactive style changes.")
            my_card {
                title: "Fade In / Out"
                primary_button {
                    label: "Toggle"
                    onclick: use_toggle(box_visible)
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
                    onclick: use_toggle(spin_active)
                    if { spin_active.get() } { "Stop Spin" } else { "Start Spin" }
                }
                div {
                    class: c_anim_spin_container()
                    div {
                        class: if { spin_active.get() } { c_anim_spin() } else { c_anim_spin_stopped() }
                        "⟳"
                    }
                }
            }
            my_card {
                title: "Pulse Effect"
                primary_button {
                    label: "Toggle"
                    onclick: use_toggle(pulse_active)
                    if { pulse_active.get() } { "Stop Pulse" } else { "Start Pulse" }
                }
                div {
                    class: c_anim_pulse_container()
                    div {
                        class: if { pulse_active.get() } { c_anim_pulse() } else { c_anim_pulse_stopped() }
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
                        onclick: progress_on_start(progress)
                        "Start"
                    }
                    primary_button {
                        label: "Reset"
                        onclick: progress_on_reset(progress)
                        "Reset"
                    }
                }
                { build_progress_bar(progress.get_value()) }
            }
            my_card {
                title: "Color Cycle"
                primary_button {
                    label: "Next"
                    onclick: color_cycle_on_next(color_index)
                    "Next Color"
                }
                div {
                    class: c_anim_color_box()
                    style: { background: get_anim_color(color_index.get()); transition: "background 0.5s ease, transform 0.3s ease"; transform: if { scale_active.get() } { "scale(0.85)" } else { "scale(1)" }; }
                    onclick: use_toggle(scale_active)
                    "Click me to shrink!"
                }
            }
        }
    }
}

/// Builds a reactive progress bar section that updates when the signal changes.
///
/// # Arguments
///
/// - `Signal<i32>` - The progress value signal (0-100).
///
/// # Returns
///
/// - `VirtualNode` - A fragment containing the progress bar and percentage text.
fn build_progress_bar(progress_value: Signal<i32>) -> VirtualNode {
    html! {
        div {
            class: c_progress_container()
            div {
                class: c_progress_bar()
                style: { width: format!("{}%", progress_value.get()); background: if { progress_value.get() >= 100 } { "#059669".to_string() } else { "#4f46e5".to_string() }; }
            }
        }
        p {
            class: c_progress_text()
            progress_value
            "%"
        }
    }
}

/// Returns a color string based on the animation color index.
///
/// # Arguments
///
/// - `i32` - The color index (0-4).
///
/// # Returns
///
/// - `String` - The CSS color string.
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
