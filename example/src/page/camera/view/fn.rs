use crate::*;

/// A camera page component that opens the device camera with QR code
/// scanning on user action. When a QR code containing a valid HTTP or
/// HTTPS URL is detected, the app navigates to that URL.
///
/// For same-origin URLs the hash fragment is extracted and an internal
/// route navigation is performed; for external URLs a full page
/// navigation occurs via `location.href`.
///
/// Renders a page header, a card with a video preview area, camera
/// control buttons (open/close and switch), and a QR code scan result
/// display area.
///
/// # Returns
///
/// - `VirtualNode` - The camera page virtual DOM tree.
#[component]
pub(crate) fn page_camera(node: VirtualNode<PageCameraProps>) -> VirtualNode {
    let _page_camera_props: PageCameraProps = node.try_get_props().unwrap_or_default();
    let state: UseCamera = use_camera();
    camera_cleanup(state);
    let on_close_camera = move |_: Event| {
        stop_qr_scan(state);
        close_camera(CAMERA_VIDEO_SELECTOR);
        state.get_camera_open().set(false);
        state.get_scan_result().set(String::new());
    };
    let on_switch_camera = move |_: Event| {
        switch_camera(state);
    };
    let on_open_camera = move |_: Event| {
        open_camera_and_scan(state);
    };
    html! {
        div {
            class: c_page_container()
            page_header {
                icon: "📷"
                title: "Camera"
                subtitle: "Open the camera to scan QR codes. Navigates on valid URL detection."
            }
            my_card {
                title: "Camera Preview"
                p {
                    class: c_demo_text()
                    "Open the camera to start scanning QR codes. When a valid URL is detected, you will be redirected immediately."
                }
                div {
                    class: c_camera_video_container()
                    video {
                        id: CAMERA_VIDEO_ID
                        class: if { state.get_camera_open().get() } { c_camera_video_active() } else { c_camera_video_hidden() }
                        autoplay: CAMERA_VIDEO_AUTOPLAY
                        playsinline: CAMERA_VIDEO_PLAYSINLINE
                    }
                    if { state.get_camera_loading().get() } {
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
                    } else if { !state.get_camera_open().get() } {
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
                if { !state.get_error_message().get().is_empty() } {
                    div {
                        class: c_camera_error_box()
                        state.get_error_message().get()
                    }
                }
                div {
                    class: c_camera_controls()
                    if { state.get_camera_open().get() } {
                        button {
                            class: c_primary_button()
                            onclick: on_close_camera
                            "Close"
                        }
                        button {
                            class: c_primary_button()
                            onclick: on_switch_camera
                            if { matches!(state.get_facing().get(), CameraFacing::User) } { CAMERA_SWITCH_TO_REAR_LABEL } else { CAMERA_SWITCH_TO_FRONT_LABEL }
                        }
                    } else if { state.get_camera_loading().get() } {
                        button {
                            class: c_primary_button()
                            disabled: true
                            "Opening..."
                        }
                    } else {
                        button {
                            class: c_primary_button()
                            onclick: on_open_camera
                            "Open"
                        }
                    }
                }
                if { !state.get_scan_result().get().is_empty() } {
                    div {
                        class: c_camera_scan_result_box()
                        div {
                            class: c_camera_scan_result_label()
                            "QR Code Result:"
                        }
                        div {
                            class: c_camera_scan_result_value()
                            state.get_scan_result().get()
                        }
                    }
                }
            }
        }
    }
}
