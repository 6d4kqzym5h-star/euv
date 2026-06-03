use crate::*;

/// Checks whether a DOM node is currently connected to the document.
///
/// Uses the `isConnected` JavaScript property to determine if the node
/// is still attached to the live DOM tree.
///
/// # Arguments
///
/// - `&T` - A reference to any type that can be converted to `&Node`.
///
/// # Returns
///
/// - `bool` - `true` if the node is connected to the document, `false` otherwise.
pub(crate) fn is_node_connected<T>(node: &T) -> bool
where
    T: AsRef<Node>,
{
    let node_ref: &Node = node.as_ref();
    let result: Result<JsValue, JsValue> =
        Reflect::get(node_ref.as_ref(), &JsValue::from_str(IS_CONNECTED));
    match result {
        Ok(value) => value.is_truthy(),
        Err(_) => false,
    }
}
