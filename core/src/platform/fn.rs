use crate::*;

/// Requests the browser to execute a callback before the next repaint.
///
/// # Arguments
///
/// - `FnOnce() + 'static`: A closure to invoke before the next repaint.
///
/// # Panics
///
/// Panics if no global `window` exists or if the animation frame request fails.
pub fn request_animation_frame<F>(callback: F)
where
    F: FnOnce() + 'static,
{
    let closure: Closure<dyn FnMut()> = Closure::once(callback);
    web_sys::window()
        .expect("no global window exists")
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("failed to request animation frame");
    closure.forget();
}
