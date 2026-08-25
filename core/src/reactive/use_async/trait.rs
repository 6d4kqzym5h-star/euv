/// Optional payload for the `Loading` state.
///
/// Most users will leave this at the default `()` — the variant
/// then carries no information beyond "we're fetching". Implement
/// this trait for your loading-hint type to attach stale-while-
/// revalidate metadata (last-known data, fetched-at timestamp, …)
/// to the `Loading` arm.
///
/// The trait has a single associated constant — a zero-sized marker
/// so the type can be used as a default type parameter — and one
/// method that produces the "no prior data, no hint" sentinel.
pub trait HasLoadingHint: Clone + 'static {
    /// The sentinel value representing "no prior data is available".
    /// `use_async` slots this in for the first render so that
    /// `.loading_hint()` always returns a usable value, even before
    /// the future has a chance to produce one.
    fn empty() -> Self;
}
