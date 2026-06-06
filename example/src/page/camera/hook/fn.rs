use crate::*;

/// Requests camera access from the browser and binds the resulting
/// media stream to the `<video>` element identified by the given CSS
/// selector.
///
/// Uses `navigator.mediaDevices.getUserMedia` with a video-only
/// constraint. On success the stream is assigned as `srcObject` on
/// the target video element and `play()` is called. Errors are
/// returned as human-readable strings.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the `<video>` element to bind the stream to.
///
/// # Returns
///
/// - `Result<(), String>` - `Ok(())` on success, or an error message on failure.
pub(crate) fn open_camera(video_selector: &str) -> Result<(), String> {
    let window_value: Window = window().expect("no global window exists");
    let navigator: Navigator = window_value.navigator();
    let media_devices: MediaDevices = navigator
        .media_devices()
        .map_err(|error: JsValue| format!("{error:?}"))?;
    let constraints: MediaStreamConstraints = MediaStreamConstraints::new();
    constraints.set_video(&JsValue::from_bool(true));
    constraints.set_audio(&JsValue::from_bool(false));
    let promise: js_sys::Promise = media_devices
        .get_user_media_with_constraints(&constraints)
        .map_err(|error: JsValue| format!("{error:?}"))?;
    let selector: String = video_selector.to_string();
    let on_fulfilled: Closure<dyn FnMut(JsValue)> =
        Closure::wrap(Box::new(move |stream_value: JsValue| {
            let stream: MediaStream = stream_value.unchecked_into();
            let document: Document = window()
                .expect("no global window exists")
                .document()
                .expect("should have a document");
            if let Some(element) = document.query_selector(&selector).ok().flatten() {
                let video_element: HtmlVideoElement = element.unchecked_into();
                video_element.set_src_object(Some(&stream));
                let _ = video_element.play();
            }
        }));
    let on_rejected: Closure<dyn FnMut(JsValue)> =
        Closure::wrap(Box::new(move |error: JsValue| {
            web_sys::console::log_2(&wasm_bindgen::JsValue::from_str("[euv-camera]"), &error);
        }));
    let _ = promise.then(&on_fulfilled).catch(&on_rejected);
    on_fulfilled.forget();
    on_rejected.forget();
    Ok(())
}

/// Stops all tracks on the media stream currently attached to the
/// `<video>` element identified by the given CSS selector.
///
/// Iterates over `videoElement.srcObject.getTracks()` and calls
/// `stop()` on each one, then clears `srcObject`.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the `<video>` element whose stream should be stopped.
pub(crate) fn close_camera(video_selector: &str) {
    let window_value: Window = window().expect("no global window exists");
    let document: Document = window_value.document().expect("should have a document");
    if let Some(element) = document.query_selector(video_selector).ok().flatten() {
        let video_element: HtmlVideoElement = element.unchecked_into();
        if let Some(stream) = video_element.src_object() {
            let stream: MediaStream = stream.unchecked_into();
            let tracks: js_sys::Array = stream.get_tracks();
            for track_value in tracks.iter() {
                let track: MediaStreamTrack = track_value.unchecked_into();
                track.stop();
            }
        }
        video_element.set_src_object(None);
    }
}
