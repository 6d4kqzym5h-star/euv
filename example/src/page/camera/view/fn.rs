use crate::*;

/// A camera page component that allows the user to open and close
/// the device camera, switch between front and rear cameras, and
/// scan QR codes. When a QR code containing an HTTP URL is detected,
/// the browser navigates directly to that URL.
///
/// Renders a page header, a card with a video preview area, camera
/// control buttons, and a QR code scan result display area.
///
/// # Returns
///
/// - `VirtualNode` - The camera page virtual DOM tree.
#[component]
pub(crate) fn page_camera(node: VirtualNode<PageCameraProps>) -> VirtualNode {
    let _page_camera_props: PageCameraProps = node.try_get_props().unwrap_or_default();
    let state: UseCamera = use_camera();
    camera_cleanup(state);
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Camera"
                subtitle: "Access the device camera, switch between front and rear, and scan QR codes."
            }
            my_card {
                title: "Camera Preview"
                p {
                    class: c_demo_text()
                    "Click the button below to open the device camera. Switch between front and rear cameras, or enable QR code scanning."
                }
                div {
                    class: c_camera_video_container()
                    video {
                        id: CAMERA_VIDEO_ID
                        class: if { state.camera_open.get() } { c_camera_video_active() } else { c_camera_video_hidden() }
                        autoplay: CAMERA_VIDEO_AUTOPLAY
                        playsinline: CAMERA_VIDEO_PLAYSINLINE
                    }
                    if { state.camera_loading.get() } {
                        div {
                            class: c_camera_video_placeholder()
                            div {
                                class: c_camera_placeholder_content()
                                div {
                                    class: c_spinner()
                                }
                                p {
                                    class: c_camera_placeholder_text()
                                    "Opening camera..."
                                }
                            }
                        }
                    } else if { !state.camera_open.get() } {
                        div {
                            class: c_camera_video_placeholder()
                            div {
                                class: c_camera_placeholder_content()
                                span {
                                    class: c_camera_placeholder_icon()
                                    CAMERA_PLACEHOLDER_ICON
                                }
                                p {
                                    class: c_camera_placeholder_text()
                                    "Camera preview will appear here"
                                }
                            }
                        }
                    }
                }
                if { !state.error_message.get().is_empty() } {
                    div {
                        class: c_camera_error_box()
                        state.error_message.get()
                    }
                }
                div {
                    class: c_camera_controls()
                    if { state.camera_open.get() } {
                        button {
                            class: c_primary_button()
                            onclick: move |_event: Event| {
                                stop_qr_scan(state);
                                close_camera(CAMERA_VIDEO_SELECTOR);
                                state.camera_open.set(false);
                                state.scan_result.set(String::new());
                            }
                            "Close Camera"
                        }
                        button {
                            class: c_primary_button()
                            onclick: move |_event: Event| {
                                switch_camera(state);
                            }
                            if { matches!(state.facing.get(), CameraFacing::User) } { CAMERA_SWITCH_TO_REAR_LABEL } else { CAMERA_SWITCH_TO_FRONT_LABEL }
                        }
                        if { state.scan_handle.get().is_none() } {
                            button {
                                class: c_primary_button()
                                onclick: move |_event: Event| {
                                    start_qr_scan(state);
                                }
                                "Scan QR Code"
                            }
                        } else {
                            button {
                                class: c_primary_button()
                                onclick: move |_event: Event| {
                                    stop_qr_scan(state);
                                }
                                "Stop Scanning"
                            }
                        }
                    } else if { state.camera_loading.get() } {
                        button {
                            class: c_primary_button_disabled()
                            disabled: true
                            "Opening..."
                        }
                    } else {
                        button {
                            class: c_primary_button()
                            onclick: move |_event: Event| {
                                state.camera_loading.set(true);
                                state.error_message.set(String::new());
                                state.scan_result.set(String::new());
                                let facing: CameraFacing = state.facing.get();
                                let result: Result<(), String> = open_camera(CAMERA_VIDEO_SELECTOR, facing);
                                match result {
                                    Ok(()) => {
                                        state.camera_open.set(true);
                                        state.camera_loading.set(false);
                                    }
                                    Err(error) => {
                                        state.error_message.set(error);
                                        state.camera_loading.set(false);
                                    }
                                }
                            }
                            "Open Camera"
                        }
                    }
                }
                if { !state.scan_result.get().is_empty() } {
                    div {
                        class: c_camera_scan_result_box()
                        div {
                            class: c_camera_scan_result_label()
                            "QR Code Result:"
                        }
                        div {
                            class: c_camera_scan_result_value()
                            state.scan_result.get()
                        }
                    }
                }
            }
        }
    }
}
