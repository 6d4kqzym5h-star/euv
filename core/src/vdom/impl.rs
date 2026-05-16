use crate::*;

/// Visual equality comparison for attribute values.
///
/// Compares values by their visual output rather than identity. `Signal`
/// values are compared by their current resolved string, `Event` values
/// are always considered equal (re-binding is handled by the handler
/// registry), and `CssClass` values are compared by class name.
impl PartialEq for AttributeValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AttributeValue::Text(a_val), AttributeValue::Text(b_val)) => a_val == b_val,
            (AttributeValue::Signal(a_sig), AttributeValue::Signal(b_sig)) => {
                a_sig.get() == b_sig.get()
            }
            (AttributeValue::Signal(a_sig), AttributeValue::Text(b_val)) => a_sig.get() == *b_val,
            (AttributeValue::Text(a_val), AttributeValue::Signal(b_sig)) => *a_val == b_sig.get(),
            (AttributeValue::Event(_), AttributeValue::Event(_)) => true,
            (AttributeValue::Css(a_css), AttributeValue::Css(b_css)) => {
                a_css.get_name() == b_css.get_name()
            }
            (AttributeValue::Dynamic(a_dyn), AttributeValue::Dynamic(b_dyn)) => a_dyn == b_dyn,
            _ => false,
        }
    }
}

/// Visual equality comparison for attribute entries.
///
/// Two attribute entries are equal when their names match and their values
/// are visually equal as defined by `AttributeValue::eq`.
impl PartialEq for AttributeEntry {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name() && self.get_value() == other.get_value()
    }
}

/// Visual equality comparison for text nodes.
///
/// Only compares the text content; the backing signal is not considered
/// because it does not affect visual output.
impl PartialEq for TextNode {
    fn eq(&self, other: &Self) -> bool {
        self.get_content() == other.get_content()
    }
}

/// Visual equality comparison for CSS classes.
///
/// Two CSS classes are considered equal when their class names match,
/// since the name uniquely identifies the visual style rule.
impl PartialEq for CssClass {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

/// Visual equality comparison for virtual DOM nodes.
///
/// Used by DynamicNode re-rendering to skip unnecessary DOM patches when
/// the rendered output has not changed. Event attributes are always
/// considered equal because re-binding event listeners is handled
/// separately by the handler registry and does not affect visual output.
/// Dynamic nodes manage their own subtree re-rendering, so two Dynamic
/// variants are always considered equal — the inner renderer handles
/// patching when the dynamic content actually changes.
impl PartialEq for VirtualNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VirtualNode::Text(a_text), VirtualNode::Text(b_text)) => a_text == b_text,
            (
                VirtualNode::Element {
                    tag: a_tag,
                    attributes: a_attrs,
                    children: a_children,
                    ..
                },
                VirtualNode::Element {
                    tag: b_tag,
                    attributes: b_attrs,
                    children: b_children,
                    ..
                },
            ) => {
                a_tag == b_tag
                    && a_attrs.len() == b_attrs.len()
                    && a_attrs.iter().zip(b_attrs.iter()).all(|(a, b)| a == b)
                    && a_children.len() == b_children.len()
                    && a_children
                        .iter()
                        .zip(b_children.iter())
                        .all(|(a, b)| a == b)
            }
            (VirtualNode::Fragment(a_children), VirtualNode::Fragment(b_children)) => {
                a_children.len() == b_children.len()
                    && a_children
                        .iter()
                        .zip(b_children.iter())
                        .all(|(a, b)| a == b)
            }
            (VirtualNode::Dynamic(_), VirtualNode::Dynamic(_)) => true,
            (VirtualNode::Empty, VirtualNode::Empty) => true,
            _ => false,
        }
    }
}

/// Maps each `Attribute` variant to its corresponding DOM attribute string.
impl Attribute {
    /// Returns the string representation of this attribute name for DOM binding.
    pub fn as_str(&self) -> String {
        match self {
            Attribute::AccessKey => "accesskey".to_string(),
            Attribute::Action => "action".to_string(),
            Attribute::Alt => "alt".to_string(),
            Attribute::AriaLabel => "aria-label".to_string(),
            Attribute::AutoComplete => "autocomplete".to_string(),
            Attribute::AutoFocus => "autofocus".to_string(),
            Attribute::Checked => "checked".to_string(),
            Attribute::Class => "class".to_string(),
            Attribute::Cols => "cols".to_string(),
            Attribute::ContentEditable => "contenteditable".to_string(),
            Attribute::Data(name) => format!("data-{}", name),
            Attribute::Dir => "dir".to_string(),
            Attribute::Disabled => "disabled".to_string(),
            Attribute::Draggable => "draggable".to_string(),
            Attribute::EncType => "enctype".to_string(),
            Attribute::For => "for".to_string(),
            Attribute::Form => "form".to_string(),
            Attribute::Height => "height".to_string(),
            Attribute::Hidden => "hidden".to_string(),
            Attribute::Href => "href".to_string(),
            Attribute::Id => "id".to_string(),
            Attribute::Lang => "lang".to_string(),
            Attribute::Max => "max".to_string(),
            Attribute::MaxLength => "maxlength".to_string(),
            Attribute::Method => "method".to_string(),
            Attribute::Min => "min".to_string(),
            Attribute::MinLength => "minlength".to_string(),
            Attribute::Multiple => "multiple".to_string(),
            Attribute::Name => "name".to_string(),
            Attribute::Pattern => "pattern".to_string(),
            Attribute::Placeholder => "placeholder".to_string(),
            Attribute::ReadOnly => "readonly".to_string(),
            Attribute::Required => "required".to_string(),
            Attribute::Rows => "rows".to_string(),
            Attribute::Selected => "selected".to_string(),
            Attribute::Size => "size".to_string(),
            Attribute::SpellCheck => "spellcheck".to_string(),
            Attribute::Src => "src".to_string(),
            Attribute::Step => "step".to_string(),
            Attribute::Style => "style".to_string(),
            Attribute::TabIndex => "tabindex".to_string(),
            Attribute::Target => "target".to_string(),
            Attribute::Title => "title".to_string(),
            Attribute::Type => "type".to_string(),
            Attribute::Value => "value".to_string(),
            Attribute::Width => "width".to_string(),
            Attribute::Other(name) => name.clone(),
        }
    }
}

/// Provides a default empty dynamic node with a no-op render function.
impl Default for DynamicNode {
    fn default() -> Self {
        let node: DynamicNode = DynamicNode {
            render_fn: Rc::new(RefCell::new(|| VirtualNode::Empty)),
            hook_context: HookContext::default(),
        };
        node
    }
}

/// Clones a `DynamicNode` by cloning its `HookContext` (Copy) and `render_fn` (Rc).
impl Clone for DynamicNode {
    fn clone(&self) -> Self {
        DynamicNode {
            render_fn: Rc::clone(self.get_render_fn()),
            hook_context: self.hook_context,
        }
    }
}

/// Converts a `VirtualNode` reference into an owned node.
impl AsNode for VirtualNode {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(self.clone())
    }
}

/// Converts a `VirtualNode` reference into an owned node.
impl AsNode for &VirtualNode {
    fn as_node(&self) -> Option<VirtualNode> {
        Some((*self).clone())
    }
}

/// Converts a `String` into a text virtual node.
impl AsNode for String {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.clone(), None)))
    }
}

/// Converts a string slice into a text virtual node.
impl AsNode for &str {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts an `i32` into a text virtual node.
impl AsNode for i32 {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts an `i64` into a text virtual node.
impl AsNode for i64 {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts a `usize` into a text virtual node.
impl AsNode for usize {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts an `f32` into a text virtual node.
impl AsNode for f32 {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts an `f64` into a text virtual node.
impl AsNode for f64 {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts a `bool` into a text virtual node.
impl AsNode for bool {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts a signal into a reactive text virtual node.
impl<T> AsNode for Signal<T>
where
    T: Clone + PartialEq + std::fmt::Display + 'static,
{
    fn as_node(&self) -> Option<VirtualNode> {
        Some(self.as_reactive_text())
    }
}

/// Converts a `VirtualNode` into itself via `IntoNode`.
impl IntoNode for VirtualNode {
    fn into_node(self) -> VirtualNode {
        self
    }
}

/// Wraps a `FnMut() -> VirtualNode` closure into a `DynamicNode` via `IntoNode`.
///
/// This enables writing `{move || html! { ... }}` directly in HTML markup
/// without explicit `DynamicNode` construction.
impl<F> IntoNode for F
where
    F: FnMut() -> VirtualNode + 'static,
{
    fn into_node(self) -> VirtualNode {
        VirtualNode::Dynamic(DynamicNode {
            render_fn: Rc::new(RefCell::new(self)),
            hook_context: crate::reactive::create_hook_context(),
        })
    }
}

/// Converts a `String` into a text virtual node via `IntoNode`.
impl IntoNode for String {
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self, None))
    }
}

/// Converts a `&str` into a text virtual node via `IntoNode`.
impl IntoNode for &str {
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self.to_string(), None))
    }
}

/// Converts an `i32` into a text virtual node via `IntoNode`.
impl IntoNode for i32 {
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self.to_string(), None))
    }
}

/// Converts a `usize` into a text virtual node via `IntoNode`.
impl IntoNode for usize {
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self.to_string(), None))
    }
}

/// Converts a `bool` into a text virtual node via `IntoNode`.
impl IntoNode for bool {
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self.to_string(), None))
    }
}

/// Converts a signal into a reactive text virtual node via `IntoNode`.
impl<T> IntoNode for Signal<T>
where
    T: Clone + PartialEq + std::fmt::Display + 'static,
{
    fn into_node(self) -> VirtualNode {
        self.as_reactive_text()
    }
}

/// Implementation of virtual node construction and property extraction.
impl VirtualNode {
    /// Creates a new element node with the given tag name.
    pub fn get_element_node(tag_name: &str) -> Self {
        VirtualNode::Element {
            tag: Tag::Element(tag_name.to_string()),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
        }
    }

    /// Creates a new text node with the given content.
    pub fn get_text_node(content: &str) -> Self {
        VirtualNode::Text(TextNode::new(content.to_string(), None))
    }

    /// Adds an attribute to this node if it is an element.
    pub fn with_attribute(mut self, name: &str, value: AttributeValue) -> Self {
        if let VirtualNode::Element {
            ref mut attributes, ..
        } = self
        {
            attributes.push(AttributeEntry::new(name.to_string(), value));
        }
        self
    }

    /// Adds a child node to this node if it is an element.
    pub fn with_child(mut self, child: VirtualNode) -> Self {
        if let VirtualNode::Element {
            ref mut children, ..
        } = self
        {
            children.push(child);
        }
        self
    }

    /// Returns true if this node is a component node.
    pub fn is_component(&self) -> bool {
        matches!(
            self,
            VirtualNode::Element {
                tag: Tag::Component(_),
                ..
            }
        )
    }

    /// Returns the tag name if this is an element or component node.
    pub fn tag_name(&self) -> Option<String> {
        match self {
            VirtualNode::Element { tag, .. } => match tag {
                Tag::Element(name) => Some(name.clone()),
                Tag::Component(name) => Some(name.clone()),
            },
            _ => None,
        }
    }

    /// Extracts a string property from this node if it is an element with the named attribute.
    pub fn try_get_prop(&self, name: &Attribute) -> Option<String> {
        let name_str: String = name.as_str();
        if let VirtualNode::Element { attributes, .. } = self {
            for attr in attributes {
                if attr.get_name() == &name_str {
                    match attr.get_value() {
                        AttributeValue::Text(value) => return Some(value.clone()),
                        AttributeValue::Signal(signal) => return Some(signal.get()),
                        _ => {}
                    }
                }
            }
        }
        None
    }

    /// Extracts a signal property from this node if it is an element with the named attribute.
    ///
    /// Returns the raw `Signal<String>` so components can reactively read the current value
    /// and subscribe to future changes, rather than receiving a snapshot string.
    pub fn try_get_signal_prop(&self, name: &Attribute) -> Option<Signal<String>> {
        let name_str: String = name.as_str();
        if let VirtualNode::Element { attributes, .. } = self {
            for attr in attributes {
                if attr.get_name() == &name_str
                    && let AttributeValue::Signal(signal) = attr.get_value()
                {
                    return Some(*signal);
                }
            }
        }
        None
    }

    /// Extracts children from this node if it is an element.
    pub fn get_children(&self) -> Vec<VirtualNode> {
        if let VirtualNode::Element { children, .. } = self {
            children.clone()
        } else {
            Vec::new()
        }
    }

    /// Extracts text content from this node.
    pub fn try_get_text(&self) -> Option<String> {
        match self {
            VirtualNode::Text(text_node) => Some(text_node.get_content().clone()),
            VirtualNode::Element { children, .. } => {
                children.first().and_then(VirtualNode::try_get_text)
            }
            _ => None,
        }
    }

    /// Extracts an event handler from this node if it is an element with the named event attribute.
    pub fn try_get_event(
        &self,
        name: &NativeEventName,
    ) -> Option<crate::event::NativeEventHandler> {
        let name_str: String = name.as_str();
        if let VirtualNode::Element { attributes, .. } = self {
            for attr in attributes {
                if attr.get_name() == &name_str
                    && let AttributeValue::Event(handler) = attr.get_value()
                {
                    return Some(handler.clone());
                }
            }
        }
        None
    }

    /// Extracts an event handler from this node by a custom attribute name.
    pub fn try_get_callback(&self, name: &str) -> Option<crate::event::NativeEventHandler> {
        if let VirtualNode::Element { attributes, .. } = self {
            for attr in attributes {
                if attr.get_name() == name
                    && let AttributeValue::Event(handler) = attr.get_value()
                {
                    return Some(handler.clone());
                }
            }
        }
        None
    }
}

/// Converts a signal into a reactive text node with listener wiring.
impl<T> AsReactiveText for Signal<T>
where
    T: Clone + PartialEq + std::fmt::Display + 'static,
{
    fn as_reactive_text(&self) -> VirtualNode {
        let signal: Signal<T> = *self;
        let initial: String = signal.get().to_string();
        let string_signal: Signal<String> = {
            let boxed: Box<SignalInner<String>> = Box::new(SignalInner::new(initial.clone()));
            Signal::from_inner(Box::leak(boxed) as *mut SignalInner<String>)
        };
        let source_signal: Signal<T> = *self;
        let string_signal_clone: Signal<String> = string_signal;
        source_signal.subscribe({
            let source_signal: Signal<T> = source_signal;
            move || {
                let new_value: String = source_signal.get().to_string();
                string_signal_clone.set(new_value);
            }
        });
        VirtualNode::Text(TextNode::new(initial, Some(string_signal)))
    }
}

/// Implementation of style CSS serialization.
impl Style {
    /// Adds a style property.
    ///
    /// Property names are automatically converted from snake_case to kebab-case
    /// (e.g., `flex_direction` becomes `flex-direction`).
    pub fn property<N, V>(mut self, name: N, value: V) -> Self
    where
        N: AsRef<str>,
        V: AsRef<str>,
    {
        self.get_mut_properties().push(StyleProperty::new(
            name.as_ref().replace('_', "-"),
            value.as_ref().to_string(),
        ));
        self
    }

    /// Converts the style to a CSS string.
    pub fn to_css_string(&self) -> String {
        self.get_properties()
            .iter()
            .map(|p| format!("{}: {};", p.get_name(), p.get_value()))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

/// Provides a default empty style.
impl Default for Style {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

/// Implementation of CssClass construction and style injection.
impl CssClass {
    /// Creates a new CSS class with the given name and style declarations.
    ///
    /// Automatically injects the styles into the DOM upon creation.
    pub fn new(name: String, style: String) -> Self {
        let mut css_class: CssClass = CssClass::default();
        css_class.set_name(name);
        css_class.set_style(style);
        css_class.inject_style();
        css_class
    }

    /// Injects this class's styles into the DOM if not already present.
    ///
    /// Creates a `<style>` element with id `euv-css-injected` on first call,
    /// then appends the class rule. Subsequent calls for the same class name
    /// are no-ops. On first creation, also injects global CSS keyframes
    /// required by built-in animations.
    pub fn inject_style(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let style_id: &str = "euv-css-injected";
            let document: web_sys::Document = web_sys::window()
                .expect("no global window exists")
                .document()
                .expect("no document exists");
            let style_element: web_sys::HtmlStyleElement = match document
                .get_element_by_id(style_id)
            {
                Some(el) => el.dyn_into::<web_sys::HtmlStyleElement>().unwrap(),
                None => {
                    let el: web_sys::HtmlStyleElement = document
                        .create_element("style")
                        .unwrap()
                        .dyn_into::<web_sys::HtmlStyleElement>()
                        .unwrap();
                    el.set_id(style_id);
                    let keyframes: &str = "@keyframes euv-spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } } @keyframes euv-fade-in { from { opacity: 0; } to { opacity: 1; } } @keyframes euv-scale-in { from { transform: scale(0.9); opacity: 0; } to { transform: scale(1); opacity: 1; } } @keyframes euv-pulse { 0%, 100% { transform: scale(1); } 50% { transform: scale(1.2); } } @keyframes euv-slide-up { from { transform: translateY(100%); } to { transform: translateY(0); } } @keyframes euv-slide-left { from { transform: translateX(-100%); } to { transform: translateX(0); } } @keyframes euv-fade-in-up { from { opacity: 0; transform: translateY(8px); } to { opacity: 1; transform: translateY(0); } }";
                    let global: &str = "html, body, #app { height: 100%; margin: 0; padding: 0; overflow: hidden; } * { -webkit-tap-highlight-color: transparent; }";
                    let media_queries: &str = "@media (max-width: 767px) { .c_app_nav { display: none; } .c_app_main { padding: 20px 16px; max-width: 100%; } .c_page_title { font-size: 22px; } .c_page_subtitle { font-size: 14px; } .c_card { padding: 16px; margin: 12px 0; border-radius: 10px; } .c_card_title { font-size: 16px; } .c_form_grid { grid-template-columns: 1fr; } .c_browser_api_row { grid-template-columns: 1fr; } .c_modal_content { max-width: 100%; width: calc(100% - 32px); border-radius: 16px 16px 0 0; position: fixed; bottom: 0; left: 16px; height: 80vh; animation: euv-slide-up 0.25s ease; } .c_modal_overlay { align-items: flex-end; } .c_event_stats { gap: 12px; flex-wrap: wrap; } .c_event_section_row { gap: 12px; flex-wrap: wrap; } .c_event_section_col { min-width: 100%; } .c_counter_value { font-size: 20px; } .c_timer_value { font-size: 36px; } .c_not_found_code { font-size: 56px; } .c_not_found_container { padding: 40px 20px; } .c_list_input_row { flex-direction: column; } .c_vconsole_button { bottom: 16px; right: 16px; width: 44px; height: 44px; border-radius: 12px; } .c_tab_bar { flex-wrap: wrap; } .c_primary_button { padding: 10px 18px; font-size: 14px; } .c_badge { padding: 4px 10px; font-size: 11px; } .c_badge_outline { padding: 4px 10px; font-size: 11px; } .c_browser_info_grid { grid-template-columns: 1fr; } .c_anim_spin { font-size: 36px; } .c_anim_spin_stopped { font-size: 36px; } .c_anim_pulse { font-size: 36px; } .c_anim_pulse_stopped { font-size: 36px; } }";
                    el.set_inner_text(&format!("{} {} {}", global, keyframes, media_queries));
                    document.head().unwrap().append_child(&el).unwrap();
                    el
                }
            };
            let existing_css: String = style_element.inner_text();
            let class_rule: String = format!(".{} {{ {} }}", self.get_name(), self.get_style());
            if !existing_css.contains(&class_rule) {
                let new_css: String = if existing_css.is_empty() {
                    class_rule
                } else {
                    format!("{}\n{}", existing_css, class_rule)
                };
                style_element.set_inner_text(&new_css);
            }
        }
    }
}

/// Displays the CSS class name.
///
/// This enables `format!("{}", css_class)` to produce the class name string,
/// which is required for reactive `if` conditions in `class:` attributes.
impl std::fmt::Display for CssClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
