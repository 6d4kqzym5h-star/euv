use crate::*;

/// Registers the current RenderEffect (if any) as a subscriber on the given signal.
///
/// This function is called by `Signal::get()` to establish a dependency from
/// the active RenderEffect to the signal being read. If no RenderEffect is
/// active, this is a no-op.
///
/// # Arguments
///
/// - `&Signal<T>` - The signal being read.
pub(crate) fn track_dependency<T>(signal: &Signal<T>)
where
    T: Clone + PartialEq + 'static,
{
    if let Some(effect_addr) = current_effect_addr() {
        crate::reactive::effect::r#fn::track_signal(signal, effect_addr);
    }
}
