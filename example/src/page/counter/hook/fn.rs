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

/// Creates a click event handler that decrements a counter signal.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal to decrement.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that decrements the counter.
pub(crate) fn counter_on_decrement(counter: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: i32 = counter.get();
        counter.set(current - 1);
    }))
}

/// Creates a click event handler that resets a counter signal to zero.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal to reset.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that resets the counter to zero.
pub(crate) fn counter_on_reset(counter: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        counter.set(0);
    }))
}
