use crate::*;

/// Cleans up the keep-alive timer interval when the page is unmounted.
///
/// # Arguments
///
/// - `Signal<Option<IntervalHandle>>` - The handle signal for the timer interval.
fn keep_alive_cleanup(handle: Signal<Option<IntervalHandle>>) {
    use_cleanup(move || {
        if let Some(h) = handle.get() {
            h.clear();
        }
    });
}

/// Renders the counter tab content for the keep-alive demo.
///
/// Maintains its own counter signal that persists when the tab is hidden
/// via CSS `display: none` instead of being destroyed.
///
/// # Returns
///
/// - `VirtualNode` - The counter tab virtual DOM tree.
fn counter_tab() -> VirtualNode {
    let count: Signal<i32> = use_signal(|| 0);
    html! {
        div {
            class: c_keep_alive_tab_panel()
            h4 {
                class: c_keep_alive_panel_title()
                "Counter"
            }
            p {
                class: c_keep_alive_demo_text()
                "This counter preserves its value when you switch tabs and come back."
            }
            div {
                class: c_keep_alive_counter_display()
                span {
                    class: c_keep_alive_counter_value()
                    count.get()
                }
            }
            div {
                class: c_keep_alive_counter_controls()
                primary_button {
                    label: "Decrement"
                    onclick: keep_alive_counter_on_decrement(count)
                    "-1"
                }
                primary_button {
                    label: "Increment"
                    onclick: keep_alive_counter_on_increment(count)
                    "+1"
                }
                primary_button {
                    label: "Reset"
                    onclick: keep_alive_counter_on_reset(count)
                    "Reset"
                }
            }
        }
    }
}

/// Renders the form tab content for the keep-alive demo.
///
/// Contains text inputs and a textarea whose values persist when the tab
/// is hidden via CSS `display: none` instead of being destroyed.
///
/// # Returns
///
/// - `VirtualNode` - The form tab virtual DOM tree.
fn form_tab() -> VirtualNode {
    let name: Signal<String> = use_signal(String::new);
    let email: Signal<String> = use_signal(String::new);
    let message: Signal<String> = use_signal(String::new);
    html! {
        div {
            class: c_keep_alive_tab_panel()
            h4 {
                class: c_keep_alive_panel_title()
                "Form"
            }
            p {
                class: c_keep_alive_demo_text()
                "Type something in the fields, switch tabs, and come back — your input is preserved."
            }
            div {
                class: c_keep_alive_form_group()
                label {
                    r#for: KEEP_ALIVE_NAME_ID
                    class: c_form_label()
                    "Name"
                }
                input {
                    id: KEEP_ALIVE_NAME_ID
                    name: KEEP_ALIVE_NAME_NAME
                    r#type: KEEP_ALIVE_TEXT_TYPE
                    autocomplete: KEEP_ALIVE_AUTOCOMPLETE_NAME
                    placeholder: KEEP_ALIVE_NAME_PLACEHOLDER
                    value: name.get()
                    class: c_form_input()
                    oninput: on_input_value(name)
                }
            }
            div {
                class: c_keep_alive_form_group()
                label {
                    r#for: KEEP_ALIVE_EMAIL_ID
                    class: c_form_label()
                    "Email"
                }
                input {
                    id: KEEP_ALIVE_EMAIL_ID
                    name: KEEP_ALIVE_EMAIL_NAME
                    r#type: KEEP_ALIVE_EMAIL_TYPE
                    autocomplete: KEEP_ALIVE_AUTOCOMPLETE_EMAIL
                    placeholder: KEEP_ALIVE_EMAIL_PLACEHOLDER
                    value: email.get()
                    class: c_form_input()
                    oninput: on_input_value(email)
                }
            }
            div {
                class: c_keep_alive_form_group()
                label {
                    r#for: KEEP_ALIVE_MESSAGE_ID
                    class: c_form_label()
                    "Message"
                }
                textarea {
                    id: KEEP_ALIVE_MESSAGE_ID
                    name: KEEP_ALIVE_MESSAGE_NAME
                    autocomplete: KEEP_ALIVE_AUTOCOMPLETE_OFF
                    placeholder: KEEP_ALIVE_MESSAGE_PLACEHOLDER
                    value: message.get()
                    class: c_textarea_input()
                    rows: KEEP_ALIVE_MESSAGE_ROWS
                    oninput: on_input_value(message)
                }
            }
            if { !name.get().is_empty() || !email.get().is_empty() || !message.get().is_empty() } {
                div {
                    class: c_keep_alive_form_preview()
                    p {
                        class: c_keep_alive_preview_label()
                        "Live Preview:"
                    }
                    p {
                        class: c_keep_alive_demo_text()
                        { format!("Name: {} | Email: {} | Message: {}", name.get(), email.get(), message.get()) }
                    }
                }
            }
        }
    }
}

/// Renders the timer tab content for the keep-alive demo.
///
/// Maintains an auto-incrementing timer that continues running even when
/// the tab is hidden via CSS `display: none`, demonstrating that hooks
/// and intervals stay alive.
///
/// # Returns
///
/// - `VirtualNode` - The timer tab virtual DOM tree.
fn timer_tab() -> VirtualNode {
    let elapsed: Signal<i32> = use_signal(|| 0);
    let running: Signal<bool> = use_signal(|| false);
    let handle: Signal<Option<IntervalHandle>> = use_signal(|| None);
    keep_alive_cleanup(handle);
    watch!(running, |is_running| {
        if is_running {
            let elapsed_signal: Signal<i32> = elapsed;
            let handle_signal: Signal<Option<IntervalHandle>> = handle;
            let new_handle: IntervalHandle = use_interval(1000, move || {
                let current: i32 = elapsed_signal.get();
                elapsed_signal.set(current + 1);
            });
            handle_signal.set(Some(new_handle));
        } else {
            let handle_opt: Option<IntervalHandle> = handle.get();
            if let Some(existing_handle) = handle_opt {
                existing_handle.clear();
            }
            handle.set(None);
        }
    });
    html! {
        div {
            class: c_keep_alive_tab_panel()
            h4 {
                class: c_keep_alive_panel_title()
                "Timer"
            }
            p {
                class: c_keep_alive_demo_text()
                "Start the timer, switch tabs, and come back — it keeps running in the background!"
            }
            div {
                class: c_keep_alive_counter_display()
                span {
                    class: c_keep_alive_counter_value()
                    { format_time(elapsed.get()) }
                }
            }
            div {
                class: c_keep_alive_counter_controls()
                if { !running.get() } {
                    primary_button {
                        label: "Start"
                        onclick: keep_alive_timer_on_start(running)
                        "Start"
                    }
                } else {
                    primary_button {
                        label: "Pause"
                        onclick: keep_alive_timer_on_pause(running, handle)
                        "Pause"
                    }
                }
                primary_button {
                    label: "Reset"
                    onclick: keep_alive_timer_on_reset(elapsed, running, handle)
                    "Reset"
                }
            }
        }
    }
}

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

/// Creates a click event handler that increments the counter signal.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal to increment.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that increments the counter.
pub(crate) fn keep_alive_counter_on_increment(count: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        let current: i32 = count.get();
        count.set(current + 1);
    }))
}

/// Creates a click event handler that decrements the counter signal.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal to decrement.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that decrements the counter.
pub(crate) fn keep_alive_counter_on_decrement(count: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        let current: i32 = count.get();
        count.set(current - 1);
    }))
}

/// Creates a click event handler that resets the counter signal to zero.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal to reset.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that resets the counter.
pub(crate) fn keep_alive_counter_on_reset(count: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        count.set(0);
    }))
}

/// Creates a click event handler that starts the timer.
///
/// # Arguments
///
/// - `Signal<bool>` - The running signal to set to true.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that starts the timer.
pub(crate) fn keep_alive_timer_on_start(running: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        running.set(true);
    }))
}

/// Creates a click event handler that pauses the timer.
///
/// # Arguments
///
/// - `Signal<bool>` - The running signal to set to false.
/// - `Signal<Option<IntervalHandle>>` - The handle signal for clearing the interval.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that pauses the timer.
pub(crate) fn keep_alive_timer_on_pause(
    running: Signal<bool>,
    _handle: Signal<Option<IntervalHandle>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        running.set(false);
    }))
}

/// Creates a click event handler that resets the timer.
///
/// # Arguments
///
/// - `Signal<i32>` - The elapsed signal to reset to zero.
/// - `Signal<bool>` - The running signal to set to false.
/// - `Signal<Option<IntervalHandle>>` - The handle signal for clearing the interval.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that resets the timer.
pub(crate) fn keep_alive_timer_on_reset(
    elapsed: Signal<i32>,
    running: Signal<bool>,
    _handle: Signal<Option<IntervalHandle>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        running.set(false);
        elapsed.set(0);
    }))
}

/// A keep-alive demo page demonstrating state preservation across tab switches.
///
/// Uses CSS `display: none` to hide inactive tab content instead of
/// destroying and recreating it, which preserves all hook state (signals,
/// intervals, form inputs) across tab switches.
///
/// # Returns
///
/// - `VirtualNode` - The keep-alive demo page virtual DOM tree.
#[component]
pub(crate) fn page_keep_alive(props: PageKeepAliveProps) -> VirtualNode {
    let PageKeepAliveProps = props;
    let tab: Signal<String> = use_signal(|| "counter".to_string());
    html! {
        div {
            class: c_page_container()
            page_header("Keep-Alive", "Preserve component state across tab switches using CSS display toggling.")
            my_card {
                title: "Tab Switching with State Preservation"
                div {
                    class: c_keep_alive_tab_bar()
                    div {
                        style: { padding: "10px 20px"; cursor: "pointer"; border-bottom: { if { tab.get() == "counter" } { "2px solid #4f46e5".to_string() } else { "2px solid transparent".to_string() } }; color: { if { tab.get() == "counter" } { "#4f46e5".to_string() } else { "inherit".to_string() } }; background: { if { tab.get() == "counter" } { "rgba(79, 70, 229, 0.08)".to_string() } else { "transparent".to_string() } }; border-radius: "6px 6px 0 0"; font-size: "14px"; font-weight: "500"; }
                        onclick: keep_alive_tab_on_select(tab, "counter")
                        "Counter"
                    }
                    div {
                        style: { padding: "10px 20px"; cursor: "pointer"; border-bottom: { if { tab.get() == "form" } { "2px solid #4f46e5".to_string() } else { "2px solid transparent".to_string() } }; color: { if { tab.get() == "form" } { "#4f46e5".to_string() } else { "inherit".to_string() } }; background: { if { tab.get() == "form" } { "rgba(79, 70, 229, 0.08)".to_string() } else { "transparent".to_string() } }; border-radius: "6px 6px 0 0"; font-size: "14px"; font-weight: "500"; }
                        onclick: keep_alive_tab_on_select(tab, "form")
                        "Form"
                    }
                    div {
                        style: { padding: "10px 20px"; cursor: "pointer"; border-bottom: { if { tab.get() == "timer" } { "2px solid #4f46e5".to_string() } else { "2px solid transparent".to_string() } }; color: { if { tab.get() == "timer" } { "#4f46e5".to_string() } else { "inherit".to_string() } }; background: { if { tab.get() == "timer" } { "rgba(79, 70, 229, 0.08)".to_string() } else { "transparent".to_string() } }; border-radius: "6px 6px 0 0"; font-size: "14px"; font-weight: "500"; }
                        onclick: keep_alive_tab_on_select(tab, "timer")
                        "Timer"
                    }
                }
                div {
                    style: { display: if { tab.get() == "counter" } { "block".to_string() } else { "none".to_string() }; }
                    { counter_tab() }
                }
                div {
                    style: { display: if { tab.get() == "form" } { "block".to_string() } else { "none".to_string() }; }
                    { form_tab() }
                }
                div {
                    style: { display: if { tab.get() == "timer" } { "block".to_string() } else { "none".to_string() }; }
                    { timer_tab() }
                }
            }
            my_card {
                title: "How It Works"
                p {
                    class: c_keep_alive_demo_text()
                    "In euv, using match or if to switch between components destroys and recreates them, resetting all hook state. To preserve state (keep-alive), render all tabs simultaneously and toggle visibility with CSS display: none / block. This way, all DynamicNodes and their HookContexts stay alive — signals retain values, intervals keep running, and form inputs are preserved."
                }
            }
        }
    }
}
