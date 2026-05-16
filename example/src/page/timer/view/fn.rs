use crate::*;

/// Formats a duration in seconds into a MM:SS display string.
///
/// # Arguments
///
/// - `i32`: The total seconds to format.
///
/// # Returns
///
/// - `String`: The formatted time string in MM:SS format.
fn format_time(total_seconds: i32) -> String {
    let minutes: i32 = total_seconds / 60;
    let seconds: i32 = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

/// A timer demo page with stopwatch and countdown features.
///
/// # Returns
///
/// - `VirtualNode`: The timer demo page virtual DOM tree.
pub fn page_timer() -> VirtualNode {
    let stopwatch: UseStopwatch = use_stopwatch();
    let countdown: UseCountdown = use_countdown();
    html! {
        div {
            class: c_page_container()
            { page_header("Timer", "Stopwatch and countdown timer with interval-based updates.") }
            my_card {
                title: "Stopwatch"
                div {
                    class: c_timer_display()
                    span {
                        class: c_timer_value()
                        { format_time(stopwatch.seconds.get()) }
                    }
                }
                div {
                    class: c_timer_controls()
                    if { !stopwatch.running.get() } {
                        primary_button {
                            label: "Start"
                            onclick: stopwatch_on_start(stopwatch)
                            "Start"
                        }
                    } else {
                        primary_button {
                            label: "Pause"
                            onclick: stopwatch_on_pause(stopwatch)
                            "Pause"
                        }
                    }
                    primary_button {
                        label: "Reset"
                        onclick: stopwatch_on_reset(stopwatch)
                        "Reset"
                    }
                }
            }
            my_card {
                title: "Countdown Timer"
                div {
                    class: c_form_input_wrapper()
                    label {
                        class: c_form_label()
                        "Set seconds"
                    }
                    input {
                        r#type: "number"
                        min: "1"
                        max: "3600"
                        placeholder: "Enter seconds..."
                        value: countdown.input
                        class: c_form_input()
                        oninput: countdown_on_input(countdown)
                    }
                }
                div {
                    class: c_timer_display()
                    span {
                        class: c_timer_value()
                        { format_time(countdown.remaining.get()) }
                    }
                }
                div {
                    class: c_timer_controls()
                    if { !countdown.running.get() } {
                        primary_button {
                            label: "Start"
                            onclick: countdown_on_start(countdown)
                            "Start"
                        }
                    } else {
                        primary_button {
                            label: "Pause"
                            onclick: countdown_on_pause(countdown)
                            "Pause"
                        }
                    }
                    primary_button {
                        label: "Reset"
                        onclick: countdown_on_reset(countdown)
                        "Reset"
                    }
                }
                if { countdown.remaining.get() == 0 && !countdown.running.get() } {
                    div {
                        class: c_timer_done()
                        "⏰ Time's up!"
                    }
                } else {
                    ""
                }
            }
        }
    }
}
