use crate::*;

/// Reads the current browser URL without query parameters and hash fragment.
///
/// Constructs the full address from `origin` + `pathname` only,
/// stripping any `?search` and `#hash` parts.
///
/// # Returns
///
/// - `String` - The current complete URL without parameters and hash.
pub(crate) fn current_url_without_params() -> String {
    let window: Window = window().expect("no global window exists");
    let location: Location = window.location();
    let origin: String = location
        .origin()
        .unwrap_or_else(|_error: JsValue| "Unknown".to_string());
    let pathname: String = location
        .pathname()
        .unwrap_or_else(|_error: JsValue| "/".to_string());
    format!("{origin}{pathname}")
}

/// URL-encodes special characters in a string for safe embedding in a data URL.
///
/// Replaces `%`, `#`, `"`, `'`, `<`, `>`, `&`, `{`, and `}` with their
/// percent-encoded equivalents.
///
/// # Arguments
///
/// - `&str` - The raw string to encode.
///
/// # Returns
///
/// - `String` - The URL-encoded string.
fn encode_svg_for_data_url(raw: &str) -> String {
    raw.chars()
        .map(|character: char| match character {
            '%' | '#' | '"' | '\'' | '<' | '>' | '&' | '{' | '}' => {
                format!("%{:02X}", character as u8)
            }
            _ => character.to_string(),
        })
        .collect()
}

/// Generates a QR code SVG data URL for the given content string.
///
/// Encodes the provided string into a QR code using the `qrcode` crate
/// with SVG rendering, then wraps the SVG XML into a `data:image/svg+xml`
/// URI suitable for use as an `img` `src` attribute.
///
/// # Arguments
///
/// - `&str` - The content to encode into the QR code.
///
/// # Returns
///
/// - `String` - A URL-encoded SVG data URL of the QR code.
pub(crate) fn generate_qr_code_data_url(content: &str) -> String {
    let code: QrCode = QrCode::new(content).unwrap_or_else(|_| QrCode::new("error").unwrap());
    let svg_string: String = code
        .render::<svg::Color>()
        .min_dimensions(QR_CODE_MIN_DIMENSION, QR_CODE_MIN_DIMENSION)
        .quiet_zone(false)
        .build();
    let encoded_svg: String = encode_svg_for_data_url(&svg_string);
    format!("{SVG_DATA_URL_PREFIX}{encoded_svg}")
}
