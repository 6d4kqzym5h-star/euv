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
    let scale_active: Signal<bool> = use_signal(|| false);
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🎬"
                title: "Animation"
                subtitle: "CSS transitions, keyframe animations, and reactive style changes."
            }
            euv_card {
                title: "Fade In / Out"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { box_visible.get() } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: "Toggle"
                        onclick: use_toggle(box_visible)
                        if { box_visible.get() } {
                            "Hide Element"
                        } else {
                            "Show Element"
                        }
                    }
                }
                if { box_visible.get() } {
                    div {
                        class: c_anim_fade_in()
                        "This element fades in and out with a smooth transition."
                    }
                }
            }
            euv_card {
                title: "Spinning Element"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { spin_active.get() } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: "Toggle"
                        onclick: use_toggle(spin_active)
                        if { spin_active.get() } {
                            "Stop Spin"
                        } else {
                            "Start Spin"
                        }
                    }
                }
                div {
                    class: c_anim_spin_container()
                    div {
                        class: if { spin_active.get() } {
                            c_anim_spin()
                        } else {
                            c_anim_spin_stopped()
                        }
                        "⟳"
                    }
                }
            }
            euv_card {
                title: "Pulse Effect"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { pulse_active.get() } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: "Toggle"
                        onclick: use_toggle(pulse_active)
                        if { pulse_active.get() } {
                            "Stop Pulse"
                        } else {
                            "Start Pulse"
                        }
                    }
                }
                div {
                    class: c_anim_pulse_container()
                    div {
                        class: if { pulse_active.get() } {
                            c_anim_pulse()
                        } else {
                            c_anim_pulse_stopped()
                        }
                        "♥"
                    }
                }
            }
            euv_card {
                title: "Progress Bar"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Start"
                        onclick: progress_on_start(progress)
                        "Start"
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Reset"
                        onclick: progress_on_reset(progress)
                        "Reset"
                    }
                }
                div {
                    class: c_progress_container()
                    div {
                        class: if { progress.get_running().get() } {
                            c_progress_bar_running()
                        } else {
                            c_progress_bar_stopped()
                        }
                    }
                }
            }
            euv_card {
                title: "Size Scale"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { scale_active.get() } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: "Toggle"
                        onclick: use_toggle(scale_active)
                        if { scale_active.get() } {
                            "Click me to restore!"
                        } else {
                            "Click me to shrink!"
                        }
                    }
                }
                div {
                    class: c_anim_scale_box()
                    style: {
                        transition: "transform 0.3s ease"; transform: if { scale_active.get() } {
                            "scale(0.85)"
                        } else {
                            "scale(1)"
                        };
                    }
                }
            }
        }
    }
}
