/// The DOM attribute name used to store the unique euv identifier on an element.
///
/// This attribute is set on every element that registers an event listener
/// so the framework can look up the element's identity across re-renders.
pub(crate) const DATA_EUV_ID: &str = "data-euv-id";
