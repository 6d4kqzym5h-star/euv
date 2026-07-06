use crate::*;

/// Guards the single-container variant (`schedule_measure`). `true` while an
/// animation frame is pending, cleared when the callback fires.
pub(crate) static PENDING_MEASURE: AtomicBool = AtomicBool::new(false);

/// Guards against scheduling redundant `requestAnimationFrame` callbacks for the
/// same container id. A container id is inserted when a frame is requested and
/// removed once the callback fires. If the id is already present, the new request
/// is skipped, preventing duplicate measurements during rapid re-renders.
pub(crate) static mut PENDING_MEASURE_BY_ID: LazyLock<PendingMeasureCell> =
    LazyLock::new(|| PendingMeasureCell(UnsafeCell::new(HashSet::new())));
