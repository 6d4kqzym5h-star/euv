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
    let stopwatch_seconds: Signal<i32> = use_signal(|| 0);
    let stopwatch_running: Signal<bool> = use_signal(|| false);
    let stopwatch_handle: Signal<Option<IntervalHandle>> = use_signal(|| None);
    let countdown_total: Signal<i32> = use_signal(|| 60);
    let countdown_remaining: Signal<i32> = use_signal(|| 60);
    let countdown_running: Signal<bool> = use_signal(|| false);
    let countdown_handle: Signal<Option<IntervalHandle>> = use_signal(|| None);
    let countdown_input: Signal<String> = use_signal(|| "60".to_string());
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
                        { format_time(stopwatch_seconds.get()) }
                    }
                }
                div {
                    class: c_timer_controls()
                    if { !stopwatch_running.get() } {
                        primary_button {
                            label: "Start"
                            onclick: move |_event: NativeEvent| {
                                let was_running: bool = stopwatch_running.get();
                                if !was_running {
                                    stopwatch_running.set(true);
                                    let handle_opt: Option<IntervalHandle> = stopwatch_handle.get();
                                    if let Some(handle) = handle_opt {
                                        handle.clear();
                                    }
                                    let seconds: Signal<i32> = stopwatch_seconds;
                                    let running: Signal<bool> = stopwatch_running;
                                    let handle: Signal<Option<IntervalHandle>> = stopwatch_handle;
                                    let new_handle: IntervalHandle = use_interval(1000, move || {
                                        if running.get() {
                                            let current: i32 = seconds.get();
                                            seconds.set(current + 1);
                                        }
                                    });
                                    handle.set(Some(new_handle));
                                }
                            }
                            "Start"
                        }
                    } else {
                        primary_button {
                            label: "Pause"
                            onclick: move |_event: NativeEvent| {
                                stopwatch_running.set(false);
                                let handle_opt: Option<IntervalHandle> = stopwatch_handle.get();
                                if let Some(handle) = handle_opt {
                                    handle.clear();
                                }
                                stopwatch_handle.set(None);
                            }
                            "Pause"
                        }
                    }
                    primary_button {
                        label: "Reset"
                        onclick: move |_event: NativeEvent| {
                            stopwatch_running.set(false);
                            let handle_opt: Option<IntervalHandle> = stopwatch_handle.get();
                            if let Some(handle) = handle_opt {
                                handle.clear();
                            }
                            stopwatch_handle.set(None);
                            stopwatch_seconds.set(0);
                        }
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
                        value: countdown_input
                        class: c_form_input()
                        oninput: move |event: NativeEvent| {
                            if let NativeEvent::Input(input_event) = event {
                                countdown_input.set(input_event.get_value().clone());
                            }
                        }
                    }
                }
                div {
                    class: c_timer_display()
                    span {
                        class: c_timer_value()
                        { format_time(countdown_remaining.get()) }
                    }
                }
                div {
                    class: c_timer_controls()
                    if { !countdown_running.get() } {
                        primary_button {
                            label: "Start"
                            onclick: move |_event: NativeEvent| {
                                let input_text: String = countdown_input.get();
                                let parsed: i32 = input_text.parse::<i32>().unwrap_or(60);
                                let total: i32 = if parsed > 0 { parsed } else { 60 };
                                countdown_total.set(total);
                                countdown_remaining.set(total);
                                countdown_running.set(true);
                                let handle_opt: Option<IntervalHandle> = countdown_handle.get();
                                if let Some(handle) = handle_opt {
                                    handle.clear();
                                }
                                let remaining: Signal<i32> = countdown_remaining;
                                let running: Signal<bool> = countdown_running;
                                let handle: Signal<Option<IntervalHandle>> = countdown_handle;
                                let new_handle: IntervalHandle = use_interval(1000, move || {
                                    if running.get() {
                                        let current: i32 = remaining.get();
                                        if current > 0 {
                                            remaining.set(current - 1);
                                        } else {
                                            running.set(false);
                                        }
                                    }
                                });
                                handle.set(Some(new_handle));
                            }
                            "Start"
                        }
                    } else {
                        primary_button {
                            label: "Pause"
                            onclick: move |_event: NativeEvent| {
                                countdown_running.set(false);
                                let handle_opt: Option<IntervalHandle> = countdown_handle.get();
                                if let Some(handle) = handle_opt {
                                    handle.clear();
                                }
                                countdown_handle.set(None);
                            }
                            "Pause"
                        }
                    }
                    primary_button {
                        label: "Reset"
                        onclick: move |_event: NativeEvent| {
                            countdown_running.set(false);
                            let handle_opt: Option<IntervalHandle> = countdown_handle.get();
                            if let Some(handle) = handle_opt {
                                handle.clear();
                            }
                            countdown_handle.set(None);
                            let total: i32 = countdown_total.get();
                            countdown_remaining.set(total);
                        }
                        "Reset"
                    }
                }
                if { countdown_remaining.get() == 0 && !countdown_running.get() } {
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
