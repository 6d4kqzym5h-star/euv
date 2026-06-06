use crate::*;

/// A camera page component that allows the user to open and close
/// the device camera, displaying the live video feed in a `<video>`
/// element.
///
/// Renders a page header, a card with a video preview area, and
/// an Open or Close Camera button depending on the current state.
/// Camera access is handled via the `open_camera` / `close_camera`
/// helper functions which call the browser `getUserMedia` API.
///
/// # Returns
///
/// - `VirtualNode` - The camera page virtual DOM tree.
#[component]
pub(crate) fn page_camera(node: VirtualNode<PageCameraProps>) -> VirtualNode {
    let _page_camera_props: PageCameraProps = node.try_get_props().unwrap_or_default();
    let camera_open: Signal<bool> = use_signal(|| false);
    let camera_loading: Signal<bool> = use_signal(|| false);
    let error_message: Signal<String> = use_signal(String::new);
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Camera"
                subtitle: "Access the device camera and display a live video feed."
            }
            my_card {
                title: "Camera Preview"
                p {
                    class: c_demo_text()
                    "Click the button below to open the device camera. The live video feed will be displayed in the preview area."
                }
                div {
                    class: c_camera_video_container()
                    video {
                        id: CAMERA_VIDEO_ID
                        class: if { camera_open.get() } { c_camera_video_active() } else { c_camera_video_hidden() }
                        autoplay: CAMERA_VIDEO_AUTOPLAY
                        playsinline: CAMERA_VIDEO_PLAYSINLINE
                    }
                    if { camera_loading.get() } {
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
                    } else if { !camera_open.get() } {
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
                if { !error_message.get().is_empty() } {
                    div {
                        class: c_camera_error_box()
                        error_message.get()
                    }
                }
                div {
                    class: c_camera_controls()
                    if { camera_open.get() } {
                        button {
                            class: c_primary_button()
                            onclick: move |_event: Event| {
                                close_camera(CAMERA_VIDEO_SELECTOR);
                                camera_open.set(false);
                            }
                            "Close Camera"
                        }
                    } else if { camera_loading.get() } {
                        button {
                            class: c_primary_button_disabled()
                            disabled: true
                            "Opening..."
                        }
                    } else {
                        button {
                            class: c_primary_button()
                            onclick: move |_event: Event| {
                                camera_loading.set(true);
                                error_message.set(String::new());
                                let result: Result<(), String> = open_camera(CAMERA_VIDEO_SELECTOR);
                                match result {
                                    Ok(()) => {
                                        camera_open.set(true);
                                        camera_loading.set(false);
                                    }
                                    Err(error) => {
                                        error_message.set(error);
                                        camera_loading.set(false);
                                    }
                                }
                            }
                            "Open Camera"
                        }
                    }
                }
            }
        }
    }
}
