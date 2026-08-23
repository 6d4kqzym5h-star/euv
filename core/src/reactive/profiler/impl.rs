use super::*;
use crate::Signal;
use lombok_macros::{Data, New};

/// A handle to the profiler registered against the current
/// hook context.
///
/// The handle owns the entries signal; calling
/// `ProfilerHandle::entries()` returns that signal so any
/// reactive read (`Signal::get()`) inside a closure subscribes
/// the enclosing render to new entries. The matching
/// measurement API is `ProfilerHandle::measure(label, f)`
/// (push-on-exit) or `ProfilerHandle::begin(label)` /
/// `ProfilerHandle::end()` (split-timer API for code paths
/// that don't fit inside a single closure).
///
/// # Lifecycle
///
/// The handle is obtained via `App::use_profiler()` (or
/// directly via `HookContext::profiler()`), which slots it into
/// the current hook context. On every render at the same hook
/// index, the same handle is returned — so measurements
/// recorded from a previous render remain visible in
/// `entries()`.
///
/// On hook-context teardown (component unmount, match-arm
/// switch, or explicit `clear()`), the handle is dropped and
/// its entries signal goes with it. If you need to keep
/// measurements alive past the lifetime of the component,
/// clone the entries vector out before the context is cleared.
#[derive(Clone, Data, New)]
pub struct ProfilerHandle {
    /// The reactive log of measurements. Every measurement
    /// pushes a fresh `ProfileEntry` into this vector via
    /// `.set(...)` — the `set` triggers the reactive update
    /// path, so any subscriber re-renders.
    entries: Signal<Vec<ProfileEntry>>,
}

/// A `begin()` marker — RAII guard that records the start
/// timestamp and the label so the matching `end()` call can
/// compute the elapsed time.
///
/// Created by `ProfilerHandle::begin(label)`. Consume with
/// `end()` to push a `ProfileEntry` into the entries signal.
/// Dropping the marker without calling `end()` discards the
/// measurement silently (we don't have a place to push a
/// half-finished entry, and panicking on drop is hostile).
#[derive(Data, New)]
pub struct ProfilerMark {
    /// The label this marker was created with. Copied out of
    /// the `&str` at construction time so the marker does not
    /// outlive any borrowed string.
    label: String,
    /// Wall-clock timestamp captured at `begin()`. Subtracted
    /// from the `end()` timestamp to compute `elapsed_ms`.
    started_ms: f64,
    /// Back-reference to the entries signal. Cloned (cheap —
    /// `Signal<T>` is `Copy`-by-pointer) so `end()` can push
    /// without re-borrowing the handle.
    entries: Signal<Vec<ProfileEntry>>,
}

impl ProfilerHandle {
    /// Records a fresh measurement around the given closure.
    ///
    /// Captures the start timestamp, runs `f`, captures the
    /// end timestamp, and pushes a `ProfileEntry { label,
    /// elapsed_ms, timestamp_ms }` into the entries signal.
    /// The `elapsed_ms` is `>= 0.0` by construction (start is
    /// always captured before `f` runs).
    ///
    /// # Arguments
    ///
    /// - `&str` - The label for this measurement.
    /// - `F: FnOnce() -> R` - The closure to measure. Can
    ///   return any type — the return value is forwarded to
    ///   the caller unchanged.
    ///
    /// # Returns
    ///
    /// - `R` - Whatever `f` returned.
    pub fn measure<F, R>(&self, label: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let started_ms: f64 = now_ms();
        let result: R = f();
        let ended_ms: f64 = now_ms();
        let entry: ProfileEntry =
            ProfileEntry::new(label.to_string(), ended_ms - started_ms, ended_ms);
        // Push the entry onto the existing entries vector.
        // Read-modify-write via `.get()` is necessary because
        // `Signal<T>::set` requires `T: Clone` (which
        // `Vec<ProfileEntry>` is) but does NOT take `&mut T`
        // — the entire new value is supplied as the argument.
        let mut current: Vec<ProfileEntry> = self.entries.get();
        current.push(entry);
        self.entries.set(current);
        result
    }

    /// Starts a measurement that will end later.
    ///
    /// Use this when the measured region is not a single
    /// closure — e.g. you want to bracket an async operation
    /// or a callback fired from event handling. The returned
    /// `ProfilerMark` knows the start timestamp and the
    /// entries signal; pass it to `end()` when the work
    /// completes.
    ///
    /// # Arguments
    ///
    /// - `&str` - The label for this measurement.
    ///
    /// # Returns
    ///
    /// - `ProfilerMark` - An RAII-ish guard. Call `mark.end()`
    ///   to push the `ProfileEntry`; drop without `end()` to
    ///   discard the measurement.
    pub fn begin(&self, label: &str) -> ProfilerMark {
        ProfilerMark::new(label.to_string(), now_ms(), self.entries)
    }

    /// Returns a clone of the entries signal so subscribers
    /// outside this handle can react to new measurements.
    ///
    /// `Signal<T>` is `Copy`-by-pointer (the inner state is
    /// heap-allocated and the handle stores a raw address),
    /// so cloning is cheap. Subscribers call `.get()` inside
    /// their render closure to subscribe to new entries.
    pub fn entries(&self) -> Signal<Vec<ProfileEntry>> {
        self.entries
    }

    /// Empties the entries vector. Useful between benchmarks
    /// ("measure just this call, not the previous ones too")
    /// and in tests ("start from a clean slate").
    pub fn clear(&self) {
        self.entries.set(Vec::new());
    }
}

impl ProfilerMark {
    /// Closes the measurement started by `begin()` and pushes
    /// the resulting `ProfileEntry` into the entries signal.
    ///
    /// After calling `end()`, the marker is consumed and
    /// cannot be reused. Calling `end()` twice is a no-op on
    /// the second call — the marker is moved into the first
    /// call, so the borrow checker prevents a second
    /// invocation in well-typed code.
    pub fn end(self) {
        let ended_ms: f64 = now_ms();
        let entry: ProfileEntry =
            ProfileEntry::new(self.label, ended_ms - self.started_ms, ended_ms);
        let mut current: Vec<ProfileEntry> = self.entries.get();
        current.push(entry);
        self.entries.set(current);
    }
}
