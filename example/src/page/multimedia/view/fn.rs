use crate::*;

/// A multimedia page component displaying a QR code of the current page URL.
///
/// Generates a QR code encoding the current browser address (without query
/// parameters and hash fragment) and renders it as an inline SVG image.
///
/// # Returns
///
/// - `VirtualNode` - The multimedia page virtual DOM tree.
#[component]
pub(crate) fn page_multimedia(node: VirtualNode<PageMultimediaProps>) -> VirtualNode {
    let PageMultimediaProps = node.try_get_props().unwrap_or_default();
    let current_url: String = current_url_without_params();
    let qr_code_data_url: String = generate_qr_code_data_url(&current_url);
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Multimedia"
                subtitle: "QR code and media content showcase."
            }
            my_card {
                title: "QR Code"
                p {
                    class: c_demo_text()
                    "Scan the QR code below to get the current page URL."
                }
                div {
                    class: c_multimedia_image_section()
                    img {
                        class: c_multimedia_qr_code()
                        src: qr_code_data_url
                        alt: "QR Code"
                    }
                }
                p {
                    class: c_multimedia_url_text()
                    current_url
                }
            }
        }
    }
}
