use super::*;

/// Type alias for navigation item click callback.
pub type NavItemClickCallback = Rc<dyn Fn(&str)>;

/// Type alias for navigation event callback.
pub type NavEventCallback = Rc<dyn Fn()>;

/// Type alias for click event handler.
pub type ClickEventHandler = Rc<dyn Fn(Event)>;
