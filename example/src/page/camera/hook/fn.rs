use crate::*;

/// Creates the camera page reactive state signals wrapped in a `UseCamera` struct.
///
/// # Returns
///
/// - `UseCamera` - The camera page state.
pub(crate) fn use_camera() -> UseCamera {
    UseCamera::new(
        use_signal(|| false),
        use_signal(|| false),
        use_signal(String::new),
        use_signal(|| CameraFacing::Environment),
        use_signal(String::new),
        use_signal(|| None),
    )
}

/// Requests camera access from the browser and binds the resulting
/// media stream to the `<video>` element identified by the given CSS selector.
///
/// Uses `navigator.mediaDevices.getUserMedia` with a video-only
/// constraint. On success the stream is assigned as `srcObject` on
/// the target video element and `play()` is called. Errors are
/// returned as human-readable strings.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the `<video>` element to bind the stream to.
/// - `CameraFacing` - The desired camera facing direction.
///
/// # Returns
///
/// - `Result<(), String>` - `Ok(())` on success, or an error message on failure.
pub(crate) fn open_camera(video_selector: &str, facing: CameraFacing) -> Result<(), String> {
    let window_value: Window = window().expect("no global window exists");
    let navigator: Navigator = window_value.navigator();
    let media_devices: MediaDevices = navigator
        .media_devices()
        .map_err(|error: JsValue| format!("{error:?}"))?;
    let constraints: MediaStreamConstraints = MediaStreamConstraints::new();
    let facing_mode: &str = match facing {
        CameraFacing::User => CAMERA_FACING_MODE_USER,
        CameraFacing::Environment => CAMERA_FACING_MODE_ENVIRONMENT,
    };
    let video_constraint: js_sys::Object = js_sys::Object::new();
    let _ = Reflect::set(
        &video_constraint,
        &JsValue::from_str("facingMode"),
        &JsValue::from_str(facing_mode),
    );
    constraints.set_video(&video_constraint);
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

/// Opens the camera, starts QR code scanning immediately, and updates
/// the state signals accordingly.
///
/// If the camera fails to open, the error message signal is set.
///
/// # Arguments
///
/// - `UseCamera` - The camera page state.
pub(crate) fn open_camera_and_scan(state: UseCamera) {
    state.get_camera_loading().set(true);
    state.get_error_message().set(String::new());
    state.get_scan_result().set(String::new());
    let facing: CameraFacing = state.get_facing().get();
    let result: Result<(), String> = open_camera(CAMERA_VIDEO_SELECTOR, facing);
    match result {
        Ok(()) => {
            state.get_camera_open().set(true);
            state.get_camera_loading().set(false);
            start_qr_scan(state);
        }
        Err(error) => {
            state.get_error_message().set(error);
            state.get_camera_loading().set(false);
        }
    }
}

/// Switches the camera to the opposite facing direction and restarts
/// QR code scanning.
///
/// Closes the current camera stream and reopens with the new facing
/// mode. On success, QR code scanning is started automatically.
///
/// # Arguments
///
/// - `UseCamera` - The camera page state.
pub(crate) fn switch_camera(state: UseCamera) {
    stop_qr_scan(state);
    close_camera(CAMERA_VIDEO_SELECTOR);
    state.get_camera_open().set(false);
    let new_facing: CameraFacing = match state.get_facing().get() {
        CameraFacing::User => CameraFacing::Environment,
        CameraFacing::Environment => CameraFacing::User,
    };
    state.get_facing().set(new_facing);
    state.get_camera_loading().set(true);
    state.get_error_message().set(String::new());
    let result: Result<(), String> = open_camera(CAMERA_VIDEO_SELECTOR, new_facing);
    match result {
        Ok(()) => {
            state.get_camera_open().set(true);
            state.get_camera_loading().set(false);
            start_qr_scan(state);
        }
        Err(error) => {
            state.get_error_message().set(error);
            state.get_camera_loading().set(false);
        }
    }
}

/// Navigates to the URL detected from a QR code.
///
/// If the URL points to the same origin (current host), extracts the
/// hash fragment route and navigates internally using `navigate`.
/// Otherwise, performs a full page navigation via `location.href`.
///
/// # Arguments
///
/// - `&str` - The URL to navigate to.
pub(crate) fn navigate_qr_url(url: &str) {
    let window_value: Window = window().expect("no global window exists");
    let location: Location = window_value.location();
    let current_origin: String = format!(
        "{}//{}",
        location.protocol().unwrap_or_default(),
        location.host().unwrap_or_default()
    );
    if url.starts_with(&current_origin)
        && let Some(fragment) = url.split('#').nth(1)
    {
        let route: &str = if fragment.is_empty() { "/" } else { fragment };
        navigate(route);
        return;
    }
    let _ = window_value.location().set_href(url);
}

/// Starts a periodic QR code scan using the browser `BarcodeDetector` API.
///
/// If the browser does not support `BarcodeDetector`, the scan is not
/// started and the error signal is set. On each interval tick, captures
/// the current video frame and attempts to detect a QR code. If a QR
/// code is found, the result is stored in `scan_result`. If the result
/// is an HTTP URL, the browser navigates directly to that URL.
///
/// # Arguments
///
/// - `UseCamera` - The camera page state.
pub(crate) fn start_qr_scan(state: UseCamera) {
    let window_value: Window = window().expect("no global window exists");
    let barcode_detector_key: JsValue = JsValue::from_str("BarcodeDetector");
    if Reflect::get(&window_value, &barcode_detector_key).is_err() {
        state
            .get_error_message()
            .set("BarcodeDetector API is not supported in this browser".to_string());
        return;
    }
    let detector_result: Result<JsValue, JsValue> =
        js_sys::Function::new_no_args("return new BarcodeDetector({ formats: ['qr_code'] })")
            .call0(&JsValue::NULL);
    let detector: JsValue = match detector_result {
        Ok(value) => value,
        Err(error) => {
            state
                .get_error_message()
                .set(format!("Failed to create BarcodeDetector: {error:?}"));
            return;
        }
    };
    let handle: IntervalHandle = use_interval(CAMERA_SCAN_INTERVAL_MILLIS, move || {
        let document: Document = window()
            .expect("no global window exists")
            .document()
            .expect("should have a document");
        let Some(element) = document
            .query_selector(CAMERA_VIDEO_SELECTOR)
            .ok()
            .flatten()
        else {
            return;
        };
        let video_element: HtmlVideoElement = element.unchecked_into();
        if video_element.ready_state() != HtmlMediaElement::HAVE_ENOUGH_DATA {
            return;
        }
        let detect_fn: js_sys::Function = Reflect::get(&detector, &JsValue::from_str("detect"))
            .ok()
            .and_then(|value: JsValue| value.dyn_into::<js_sys::Function>().ok())
            .unwrap_or_else(|| js_sys::Function::new_no_args("return Promise.resolve([])"));
        let promise: js_sys::Promise = match detect_fn.call1(&detector, &video_element) {
            Ok(result) => result.into(),
            Err(_) => return,
        };
        let scan_state: UseCamera = state;
        let on_detected: Closure<dyn FnMut(JsValue)> =
            Closure::wrap(Box::new(move |barcodes_value: JsValue| {
                let barcodes: js_sys::Array = match barcodes_value.dyn_into::<js_sys::Array>() {
                    Ok(array) => array,
                    Err(_) => return,
                };
                if barcodes.length() == 0 {
                    return;
                }
                if let Some(first) = barcodes.get(0).as_string() {
                    scan_state.get_scan_result().set(first.clone());
                    if is_valid_qr_url(&first) {
                        stop_qr_scan(scan_state);
                        close_camera(CAMERA_VIDEO_SELECTOR);
                        scan_state.get_camera_open().set(false);
                        navigate_qr_url(&first);
                    }
                } else if let Ok(raw_value) =
                    Reflect::get(&barcodes.get(0), &JsValue::from_str("rawValue"))
                    && let Some(text) = raw_value.as_string()
                {
                    scan_state.get_scan_result().set(text.clone());
                    if is_valid_qr_url(&text) {
                        stop_qr_scan(scan_state);
                        close_camera(CAMERA_VIDEO_SELECTOR);
                        scan_state.get_camera_open().set(false);
                        navigate_qr_url(&text);
                    }
                }
            }));
        let on_scan_error: Closure<dyn FnMut(JsValue)> =
            Closure::wrap(Box::new(move |_error: JsValue| {}));
        let _ = promise.then(&on_detected).catch(&on_scan_error);
        on_detected.forget();
        on_scan_error.forget();
    });
    state.get_scan_handle().set(Some(handle));
}

/// Stops the periodic QR code scan timer if it is running.
///
/// # Arguments
///
/// - `UseCamera` - The camera page state.
pub(crate) fn stop_qr_scan(state: UseCamera) {
    if let Some(handle) = state.get_scan_handle().get() {
        handle.clear();
        state.get_scan_handle().set(None);
    }
}

/// Checks whether the given string is a valid QR code URL that the
/// camera scanner should navigate to.
///
/// A valid URL must start with `http://` or `https://`.
///
/// # Arguments
///
/// - `&str` - The string to check.
///
/// # Returns
///
/// - `bool` - `true` if the string is a valid HTTP or HTTPS URL.
pub(crate) fn is_valid_qr_url(text: &str) -> bool {
    text.starts_with(CAMERA_URL_PREFIX_HTTP) || text.starts_with(CAMERA_URL_PREFIX_HTTPS)
}

/// Registers a cleanup callback that closes the camera stream and
/// stops the QR code scan timer when the component unmounts or
/// the page route switches away.
///
/// # Arguments
///
/// - `UseCamera` - The camera page state.
pub(crate) fn camera_cleanup(state: UseCamera) {
    use_cleanup(move || {
        stop_qr_scan(state);
        close_camera(CAMERA_VIDEO_SELECTOR);
        state.get_camera_open().set(false);
        state.get_camera_loading().set(false);
        state.get_error_message().set(String::new());
        state.get_scan_result().set(String::new());
    });
}
