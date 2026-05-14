use crate::*;

/// Trait defining the lifecycle of a component.
///
/// Components are the primary building blocks of euv applications.
/// Each component has its own state, message type, and rendering logic.
pub trait Component {
    /// The type of messages this component can receive.
    type Message;
    /// The type of properties passed to this component.
    type Properties;

    /// Creates a new component instance with the given properties and context.
    fn create(properties: Self::Properties, context: ComponentContext) -> Self;

    /// Updates the component state in response to a message.
    fn update(&mut self, message: Self::Message) -> bool;

    /// Renders the component to a virtual DOM node.
    fn view(&self) -> VirtualNode;
}
