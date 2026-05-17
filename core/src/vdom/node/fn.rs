use crate::*;

/// Constructs a `VirtualNode::Dynamic` from a render closure with hook context management.
///
/// This function replaces the boilerplate that was previously generated inline
/// by the `html!` macro for every `Dynamic`, `If`, `Match`, and `For` node.
/// The macro now emits `euv_core::create_dynamic_node(render_fn)` instead of
/// the full `HookContext` + `DynamicNode` + `render_fn` setup block.
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
    let render_fn: Rc<RefCell<dyn FnMut() -> VirtualNode>> = {
        let mut hook_context: HookContext = hook_context;
        Rc::new(RefCell::new(move || {
            hook_context.reset_hook_index();
            render_fn()
        }))
    };
    let mut dynamic_node: DynamicNode = DynamicNode::default();
    dynamic_node.set_render_fn(render_fn);
    dynamic_node.set_hook_context(hook_context);
    VirtualNode::Dynamic(dynamic_node)
}

/// Constructs a `VirtualNode::Dynamic` for match expressions where arm hook
/// isolation is required. The render closure receives a `&mut HookContext`
/// so it can call `set_arm_changed` before each arm body.
///
/// This function replaces the inline `HookContext` + `DynamicNode` setup that
/// was previously generated for match nodes, where the hook context had to be
/// accessible inside the render closure for arm switching.
///
/// # Arguments
///
/// - `FnMut(&mut HookContext) -> VirtualNode + 'static` - The render closure
///   that receives a mutable reference to the hook context. The closure is
///   responsible for calling `set_arm_changed` before each match arm.
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
    let render_fn: Rc<RefCell<dyn FnMut() -> VirtualNode>> = {
        let mut hook_context: HookContext = hook_context;
        Rc::new(RefCell::new(move || {
            hook_context.reset_hook_index();
            render_fn(&mut hook_context)
        }))
    };
    let mut dynamic_node: DynamicNode = DynamicNode::default();
    dynamic_node.set_render_fn(render_fn);
    dynamic_node.set_hook_context(hook_context);
    VirtualNode::Dynamic(dynamic_node)
}
