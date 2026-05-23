use crate::*;

/// Constructs a `VirtualNode::Dynamic` from a render closure with hook context management.
///
/// # Arguments
///
/// - `FnMut() -> VirtualNode + 'static` - The render closure that produces
///   a virtual node tree. Called on initial render and on every signal update.
///
/// # Returns
///
/// - `VirtualNode` - A `VirtualNode::Dynamic` wrapping the render closure
///   with a fresh `HookContext`.
pub fn create_dynamic_node<F>(mut render_fn: F) -> VirtualNode
where
    F: FnMut() -> VirtualNode + 'static,
{
    let hook_context: HookContext = create_hook_context();
    let mut hook_context_for_closure: HookContext = hook_context.clone();
    let inner: Rc<RefCell<RenderFnInner>> =
        Rc::new(RefCell::new(RenderFnInner::new(Box::new(move || {
            hook_context_for_closure.reset_hook_index();
            render_fn()
        }))));
    let dynamic_node: DynamicNode = DynamicNode::new(inner, hook_context);
    VirtualNode::Dynamic(dynamic_node)
}

/// Constructs a `VirtualNode::Dynamic` for match expressions where arm hook
/// isolation is required. The render closure receives a `&mut HookContext`
/// so it can call `set_arm_changed` before each arm body.
///
/// # Arguments
///
/// - `FnMut(&mut HookContext) -> VirtualNode + 'static` - The render closure
///   that receives a mutable reference to the hook context.
///
/// # Returns
///
/// - `VirtualNode` - A `VirtualNode::Dynamic` wrapping the render closure
///   with a fresh `HookContext`.
pub fn create_dynamic_node_with_context<F>(mut render_fn: F) -> VirtualNode
where
    F: FnMut(&mut HookContext) -> VirtualNode + 'static,
{
    let hook_context: HookContext = create_hook_context();
    let mut hook_context_for_closure: HookContext = hook_context.clone();
    let inner: Rc<RefCell<RenderFnInner>> =
        Rc::new(RefCell::new(RenderFnInner::new(Box::new(move || {
            hook_context_for_closure.reset_hook_index();
            render_fn(&mut hook_context_for_closure)
        }))));
    let dynamic_node: DynamicNode = DynamicNode::new(inner, hook_context);
    VirtualNode::Dynamic(dynamic_node)
}
