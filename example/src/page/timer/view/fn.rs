use crate::*;

/// Formats a duration in seconds into a MM:SS display string.
///
/// # Arguments
///
/// - `i32` - The total seconds to format.
///
/// # Returns
///
/// - `String` - The formatted time string in MM:SS format.
fn format_time(total_seconds: i32) -> String {
    let minutes: i32 = total_seconds / 60;
    let seconds: i32 = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

/// A timer demo page with stopwatch and countdown features.
///
/// # Returns
///
/// - `VirtualNode` - The timer demo page virtual DOM tree.
#[component]
pub(crate) fn page_timer(props: PageTimerProps) -> VirtualNode {
    let PageTimerProps = props;
    let stopwatch: UseStopwatch = use_stopwatch();
    let countdown: UseCountdown = use_countdown();
    let sw_handle: Signal<Option<IntervalHandle>> = stopwatch.get_handle();
    let cd_handle: Signal<Option<IntervalHandle>> = countdown.get_handle();
    use_cleanup(move || {
        if let Some(handle) = sw_handle.get() {
            handle.clear();
        }
        if let Some(handle) = cd_handle.get() {
            handle.clear();
        }
    });
    html! {
        div {
            class: c_page_container()
            page_header("Timer", "Stopwatch and countdown timer with interval-based updates.")
            my_card {
                title: "Stopwatch"
                div {
                    class: c_timer_display()
                    span {
                        class: c_timer_value()
                        { format_time(stopwatch.get_seconds().get()) }
                    }
                }
                div {
                    class: c_timer_controls()
                    if { !stopwatch.get_running().get() } {
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
                        r#for: COUNTDOWN_SECONDS_ID
                        class: c_form_label()
                        "Set seconds"
                    }
                    input {
                        id: COUNTDOWN_SECONDS_ID
                        name: COUNTDOWN_SECONDS_NAME
                        r#type: TIMER_NUMBER_TYPE
                        autocomplete: TIMER_AUTOCOMPLETE_OFF
                        min: COUNTDOWN_SECONDS_MIN
                        max: COUNTDOWN_SECONDS_MAX
                        placeholder: COUNTDOWN_SECONDS_PLACEHOLDER
                        value: countdown.get_input()
                        class: c_form_input()
                        oninput: countdown_on_input(countdown)
                    }
                }
                div {
                    class: c_timer_display()
                    span {
                        class: c_timer_value()
                        { format_time(countdown.get_remaining().get()) }
                    }
                }
                div {
                    class: c_timer_controls()
                    if { !countdown.get_running().get() } {
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
                if { countdown.get_remaining().get() == 0 && !countdown.get_running().get() } {
                    div {
                        class: c_timer_done()
                        "⏰ Time's up!"
                    }
                }
            }
        }
    }
}
