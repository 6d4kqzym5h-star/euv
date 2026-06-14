use crate::*;

/// An animation demo page showcasing CSS animations and transitions.
///
/// # Returns
///
/// - `VirtualNode` - The animation demo page virtual DOM tree.
#[component]
pub(crate) fn page_animation(node: VirtualNode<PageAnimationProps>) -> VirtualNode {
    let PageAnimationProps = node.try_get_props().unwrap_or_default();
    let box_visible: Signal<bool> = use_signal(|| false);
    let spin_active: Signal<bool> = use_signal(|| false);
    let pulse_active: Signal<bool> = use_signal(|| false);
    let progress: UseProgress = use_progress();
    let color_index: Signal<i32> = use_signal(|| 0);
    let scale_active: Signal<bool> = use_signal(|| false);
    let _progress_cols: Signal<usize> = use_equal_wrap_all(2, TIMER_CONTROLS_SELECTOR);
    html! {
        div {
            class: c_page_container()
            page_header {
                icon: "🎬"
                title: "Animation"
                subtitle: "CSS transitions, keyframe animations, and reactive style changes."
            }
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
                    class: format!("{} {}", c_equal_wrap().get_name(), c_timer_controls().get_name())
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
                div {
                    class: c_progress_container()
                    div {
                        class: if { progress.get_running().get() } { c_progress_bar_running() } else { c_progress_bar_stopped() }
                    }
                }
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

/// Returns a CSS variable reference string based on the animation color index.
///
/// Uses theme-aware CSS variables so colors adapt to light/dark mode.
///
/// # Arguments
///
/// - `i32` - The color index (0-4).
///
/// # Returns
///
/// - `&'static str` - The CSS variable reference string.
fn get_anim_color(index: i32) -> &'static str {
    let colors: Vec<&'static str> = vec![
        "var(--accent)",
        "var(--color-success)",
        "var(--color-warning)",
        "var(--color-error)",
        "var(--color-purple)",
    ];
    let safe_index: usize = if index >= 0 && (index as usize) < colors.len() {
        index as usize
    } else {
        0
    };
    colors[safe_index]
}
