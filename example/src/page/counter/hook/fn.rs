use crate::*;

/// Creates a click event handler that increments a counter signal.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal to increment.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that increments the counter.
pub(crate) fn counter_on_increment(counter: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: i32 = counter.get();
        counter.set(current + 1);
    }))
}
