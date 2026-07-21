use super::*;

/// Implementation of camera functionality.
impl UseEuvCamera {
    /// Creates camera state for controlling camera stream and QR scanning.
    ///
    /// # Returns
    ///
    /// - `UseEuvCamera` - The camera state.
    pub fn use_camera_state() -> UseEuvCamera {
        UseEuvCamera::new(
            App::use_signal(|| false),
            App::use_signal(|| false),
            App::use_signal(String::new),
            App::use_signal(EuvCameraFacing::default),
            App::use_signal(String::new),
            App::use_signal(|| None),
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
    /// - `EuvCameraFacing` - The desired camera facing direction.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - `Ok(())` on success, or an error message on failure.
    pub(crate) fn open(video_selector: &str, facing: EuvCameraFacing) -> Result<(), String> {
        let window_value: Window = window().expect("no global window exists");
        let navigator: Navigator = window_value.navigator();
        let media_devices: MediaDevices = navigator
            .media_devices()
            .map_err(|error: JsValue| format!("{error:?}"))?;
        let constraints: MediaStreamConstraints = MediaStreamConstraints::new();
        let facing_mode: &str = match facing {
            EuvCameraFacing::User => CAMERA_FACING_MODE_USER,
            EuvCameraFacing::Environment => CAMERA_FACING_MODE_ENVIRONMENT,
        };
        let video_constraint: Object = Object::new();
        let _ = Reflect::set(
            &video_constraint,
            &JsValue::from_str("facingMode"),
            &JsValue::from_str(facing_mode),
        );
        constraints.set_video(&video_constraint);
        constraints.set_audio(&JsValue::from_bool(false));
        let promise: Promise = media_devices
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
    pub(crate) fn close(video_selector: &str) {
        let window_value: Window = window().expect("no global window exists");
        let document: Document = window_value.document().expect("should have a document");
        if let Some(element) = document.query_selector(video_selector).ok().flatten() {
            let video_element: HtmlVideoElement = element.unchecked_into();
            if let Some(stream) = video_element.src_object() {
                let stream: MediaStream = stream.unchecked_into();
                let tracks: Array = stream.get_tracks();
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
    /// - `Option<&EuvCameraConfig>` - Optional camera configuration.
    pub(crate) fn open_and_scan(self, config: Option<&EuvCameraConfig>) {
        let cfg: EuvCameraConfig =
            config.map_or_else(EuvCameraConfig::default, |c: &EuvCameraConfig| c.clone());
        self.get_camera_loading().set(true);
        self.get_error_message().set(String::new());
        self.get_scan_result().set(String::new());
        let facing: EuvCameraFacing = self.get_facing().get();
        let result: Result<(), String> = Self::open(cfg.video_selector, facing);
        match result {
            Ok(()) => {
                self.get_camera_open().set(true);
                self.get_camera_loading().set(false);
                if cfg.auto_scan {
                    self.start_qr_scan(config);
                }
            }
            Err(error) => {
                self.get_error_message().set(error);
                self.get_camera_loading().set(false);
                if let Some(ref on_error) = cfg.on_error {
                    on_error(self.get_error_message().get());
                }
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
    /// - `Option<&EuvCameraConfig>` - Optional camera configuration.
    pub(crate) fn switch(self, config: Option<&EuvCameraConfig>) {
        let cfg: EuvCameraConfig =
            config.map_or_else(EuvCameraConfig::default, |c: &EuvCameraConfig| c.clone());
        self.stop_qr_scan();
        Self::close(cfg.video_selector);
        self.get_camera_open().set(false);
        let new_facing: EuvCameraFacing = match self.get_facing().get() {
            EuvCameraFacing::User => EuvCameraFacing::Environment,
            EuvCameraFacing::Environment => EuvCameraFacing::User,
        };
        self.get_facing().set(new_facing);
        self.get_camera_loading().set(true);
        self.get_error_message().set(String::new());
        let result: Result<(), String> = Self::open(cfg.video_selector, new_facing);
        match result {
            Ok(()) => {
                self.get_camera_open().set(true);
                self.get_camera_loading().set(false);
                if cfg.auto_scan {
                    self.start_qr_scan(config);
                }
            }
            Err(error) => {
                self.get_error_message().set(error);
                self.get_camera_loading().set(false);
            }
        }
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
    /// - `Option<&EuvCameraConfig>` - Optional camera configuration.
    pub(crate) fn start_qr_scan(self, config: Option<&EuvCameraConfig>) {
        let cfg: EuvCameraConfig =
            config.map_or_else(EuvCameraConfig::default, |c: &EuvCameraConfig| c.clone());
        let window_value: Window = window().expect("no global window exists");
        let barcode_detector_key: JsValue = JsValue::from_str("BarcodeDetector");
        let barcode_detector_constructor: Function =
            match Reflect::get(&window_value, &barcode_detector_key) {
                Ok(value) if !value.is_undefined() && !value.is_null() => value.unchecked_into(),
                _ => {
                    self.get_error_message()
                        .set("BarcodeDetector API is not supported in this browser".to_string());
                    return;
                }
            };
        let formats_array: Array = Array::new();
        formats_array.push(&JsValue::from_str("qr_code"));
        let init_object: Object = Object::new();
        let _ = Reflect::set(&init_object, &JsValue::from_str("formats"), &formats_array);
        let args_array: Array = Array::new();
        args_array.push(&init_object.into());
        let detector: JsValue = match Reflect::construct(&barcode_detector_constructor, &args_array)
        {
            Ok(value) => value,
            Err(error) => {
                self.get_error_message()
                    .set(format!("Failed to create BarcodeDetector: {error:?}"));
                return;
            }
        };
        let video_selector: Rc<String> = Rc::new(cfg.video_selector.to_string());
        let on_qr_detected: Option<QrDetectedCallback> = cfg.on_qr_detected.clone();
        let handle: IntervalHandle = App::use_interval(cfg.scan_interval_millis, move || {
            let document: Document = window()
                .expect("no global window exists")
                .document()
                .expect("should have a document");
            let Some(element) = document.query_selector(&video_selector).ok().flatten() else {
                return;
            };
            let video_element: HtmlVideoElement = element.unchecked_into();
            if video_element.ready_state() != HtmlMediaElement::HAVE_ENOUGH_DATA {
                return;
            }
            let detect_fn: Function = Reflect::get(&detector, &JsValue::from_str("detect"))
                .ok()
                .and_then(|value: JsValue| value.dyn_into::<Function>().ok())
                .unwrap_or_else(|| Function::new_no_args("return Promise.resolve([])"));
            let promise: Promise = match detect_fn.call1(&detector, &video_element) {
                Ok(result) => result.into(),
                Err(_) => return,
            };
            let on_qr_detected_clone: Option<QrDetectedCallback> = on_qr_detected.clone();
            let video_selector_clone: Rc<String> = video_selector.clone();
            let on_detected: Closure<dyn FnMut(JsValue)> =
                Closure::wrap(Box::new(move |barcodes_value: JsValue| {
                    let barcodes: Array = match barcodes_value.dyn_into::<Array>() {
                        Ok(array) => array,
                        Err(_) => return,
                    };
                    if barcodes.length() == 0 {
                        return;
                    }
                    let text: Option<String> = barcodes.get(0).as_string().or_else(|| {
                        Reflect::get(&barcodes.get(0), &JsValue::from_str("rawValue"))
                            .ok()
                            .and_then(|v: JsValue| v.as_string())
                    });
                    if let Some(text) = text {
                        self.get_scan_result().set(text.clone());
                        if let Some(ref callback) = on_qr_detected_clone {
                            callback(&text);
                        }
                        if Self::is_valid_qr_url(&text) {
                            self.stop_qr_scan();
                            Self::close(&video_selector_clone);
                            self.get_camera_open().set(false);
                            Self::navigate_qr_url(&text);
                        }
                    }
                }));
            let on_scan_error: Closure<dyn FnMut(JsValue)> =
                Closure::wrap(Box::new(move |_error: JsValue| {}));
            let _ = promise.then(&on_detected).catch(&on_scan_error);
            on_detected.forget();
            on_scan_error.forget();
        });
        self.get_scan_handle().set(Some(handle));
    }

    /// Stops the periodic QR code scan timer if it is running.
    pub(crate) fn stop_qr_scan(self) {
        if let Some(handle) = self.get_scan_handle().get() {
            handle.clear();
            self.get_scan_handle().set(None);
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

    /// Extracts the hostname from an absolute URL string using pure Rust
    /// string parsing.
    ///
    /// Supports `http://` and `https://` schemes, strips IPv6 brackets,
    /// and ignores the port portion. Returns an empty string if the URL
    /// format is not recognised.
    ///
    /// # Arguments
    ///
    /// - `&str` - The absolute URL to parse.
    ///
    /// # Returns
    ///
    /// - `String` - The extracted hostname, or an empty string on failure.
    pub(crate) fn extract_hostname(url: &str) -> String {
        let rest: &str = if let Some(stripped) = url.strip_prefix(CAMERA_URL_PREFIX_HTTPS) {
            stripped
        } else if let Some(stripped) = url.strip_prefix(CAMERA_URL_PREFIX_HTTP) {
            stripped
        } else {
            return String::new();
        };
        let authority: &str = rest.split('/').next().unwrap_or("");
        let host_with_brackets: &str = authority.split(':').next().unwrap_or("");
        if let Some(stripped) = host_with_brackets.strip_prefix('[')
            && let Some(inner) = stripped.strip_suffix(']')
        {
            return inner.to_string();
        }
        host_with_brackets.to_string()
    }

    /// Checks whether the given hostname is a private or loopback IP
    /// address.
    ///
    /// Recognises loopback (`127.0.0.0/8`), link-local (`169.254.0.0/16`),
    /// RFC 1918 private ranges (`10.0.0.0/8`, `172.16.0.0/12`,
    /// `192.168.0.0/16`), and the `localhost` hostname.
    ///
    /// # Arguments
    ///
    /// - `&str` - The hostname to inspect.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the hostname is a private/internal address.
    pub(crate) fn is_private_host(hostname: &str) -> bool {
        if hostname.is_empty() {
            return false;
        }
        if hostname.eq_ignore_ascii_case(CAMERA_LOCALHOST_HOSTNAME) {
            return true;
        }
        let octets: Vec<&str> = hostname.split('.').collect();
        if octets.len() != 4 {
            return false;
        }
        let Ok(first) = octets[0].parse::<u8>() else {
            return false;
        };
        let Ok(second) = octets[1].parse::<u8>() else {
            return false;
        };
        if first == 127 {
            return true;
        }
        if first == 10 {
            return true;
        }
        if first == 172 && (16..=31).contains(&second) {
            return true;
        }
        if first == 192 && second == 168 {
            return true;
        }
        if first == 169 && second == 254 {
            return true;
        }
        false
    }

    /// Navigates to the URL detected from a QR code.
    ///
    /// If the URL points to the same origin (current host), extracts the
    /// hash fragment route and navigates internally using `navigate`.
    /// If the URL host is a private/internal IP address, performs a full
    /// page navigation via `location.href` within the current browser.
    /// Otherwise (external public URL), opens the link in the system
    /// browser via `window.open` so the user stays in the app.
    ///
    /// # Arguments
    ///
    /// - `&str` - The URL to navigate to.
    pub(crate) fn navigate_qr_url(url: &str) {
        let window_value: Window = window().expect("no global window exists");
        let location: Location = window_value.location();
        let current_hostname: String = location.hostname().unwrap_or_default();
        let url_hostname: String = Self::extract_hostname(url);
        if url_hostname == current_hostname
            && let Some(fragment) = url.split('#').nth(1)
        {
            let route: &str = if fragment.is_empty() { "/" } else { fragment };
            Router::navigate(route);
            return;
        }
        if Self::is_private_host(&url_hostname) {
            let _ = window_value.location().set_href(url);
            return;
        }
        if let Ok(open_fn) = Reflect::get(&window_value, &JsValue::from_str("open"))
            .and_then(|value: JsValue| value.dyn_into::<Function>())
        {
            let _ = open_fn.call2(
                &window_value,
                &JsValue::from_str(url),
                &JsValue::from_str(SYSTEM_BROWSER_TARGET),
            );
        }
    }

    /// Creates a click event handler that closes the camera stream and
    /// stops the QR code scan.
    ///
    /// # Arguments
    ///
    /// - `Option<&EuvCameraConfig>` - Optional camera configuration.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click event handler.
    pub fn on_close(self, config: Option<&EuvCameraConfig>) -> Option<Rc<dyn Fn(Event)>> {
        let cfg: EuvCameraConfig =
            config.map_or_else(EuvCameraConfig::default, |c: &EuvCameraConfig| c.clone());
        Some(Rc::new(move |_: Event| {
            self.stop_qr_scan();
            Self::close(cfg.video_selector);
            self.get_camera_open().set(false);
            self.get_scan_result().set(String::new());
        }))
    }

    /// Creates a click event handler that switches the camera facing direction.
    ///
    /// # Arguments
    ///
    /// - `Option<&EuvCameraConfig>` - Optional camera configuration.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click event handler.
    pub fn on_switch(self, config: Option<&EuvCameraConfig>) -> Option<Rc<dyn Fn(Event)>> {
        let cfg: EuvCameraConfig =
            config.map_or_else(EuvCameraConfig::default, |c: &EuvCameraConfig| c.clone());
        Some(Rc::new(move |_: Event| {
            self.switch(Some(&cfg));
        }))
    }

    /// Creates a click event handler that opens the camera and starts QR scanning.
    ///
    /// # Arguments
    ///
    /// - `Option<&EuvCameraConfig>` - Optional camera configuration.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click event handler.
    pub fn on_open(self, config: Option<&EuvCameraConfig>) -> Option<Rc<dyn Fn(Event)>> {
        let cfg: EuvCameraConfig =
            config.map_or_else(EuvCameraConfig::default, |c: &EuvCameraConfig| c.clone());
        Some(Rc::new(move |_: Event| {
            self.open_and_scan(Some(&cfg));
        }))
    }

    /// Registers a cleanup callback that closes the camera stream and
    /// stops the QR code scan timer when the component unmounts or
    /// the page route switches away.
    ///
    /// # Arguments
    ///
    /// - `Option<&EuvCameraConfig>` - Optional camera configuration.
    pub fn cleanup(self, config: Option<&EuvCameraConfig>) {
        let cfg: EuvCameraConfig =
            config.map_or_else(EuvCameraConfig::default, |c: &EuvCameraConfig| c.clone());
        App::use_cleanup(move || {
            self.stop_qr_scan();
            Self::close(cfg.video_selector);
            self.get_camera_open().set(false);
            self.get_camera_loading().set(false);
            self.get_error_message().set(String::new());
            self.get_scan_result().set(String::new());
        });
    }
}

/// Default implementation for `EuvCameraConfig`.
impl Default for EuvCameraConfig {
    fn default() -> Self {
        EuvCameraConfig {
            video_selector: CAMERA_VIDEO_SELECTOR,
            scan_interval_millis: CAMERA_SCAN_INTERVAL_MILLIS,
            auto_scan: true,
            on_qr_detected: None,
            on_error: None,
        }
    }
}
