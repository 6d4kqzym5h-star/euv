use crate::*;

/// Represents the name of an HTML attribute or DOM property.
///
/// Covers common HTML attributes, accessibility attributes, and custom data attributes.
#[derive(Clone)]
pub enum Attribute {
    /// The accesskey attribute.
    AccessKey,
    /// The action attribute for forms.
    Action,
    /// The alt attribute for images.
    Alt,
    /// The aria-label accessibility attribute.
    AriaLabel,
    /// The autocomplete attribute.
    AutoComplete,
    /// The autofocus attribute.
    AutoFocus,
    /// The checked attribute for checkboxes/radios.
    Checked,
    /// The class attribute.
    Class,
    /// The cols attribute for textareas.
    Cols,
    /// The contenteditable attribute.
    ContentEditable,
    /// The data-* custom attribute.
    Data(String),
    /// The dir attribute.
    Dir,
    /// The disabled attribute.
    Disabled,
    /// The draggable attribute.
    Draggable,
    /// The enctype attribute for forms.
    EncType,
    /// The for attribute for labels.
    For,
    /// The form attribute.
    Form,
    /// The height attribute.
    Height,
    /// The hidden attribute.
    Hidden,
    /// The href attribute for links.
    Href,
    /// The id attribute.
    Id,
    /// The lang attribute.
    Lang,
    /// The max attribute.
    Max,
    /// The maxlength attribute.
    MaxLength,
    /// The method attribute for forms.
    Method,
    /// The min attribute.
    Min,
    /// The minlength attribute.
    MinLength,
    /// The multiple attribute for selects.
    Multiple,
    /// The name attribute.
    Name,
    /// The pattern attribute for inputs.
    Pattern,
    /// The placeholder attribute.
    Placeholder,
    /// The readonly attribute.
    ReadOnly,
    /// The required attribute.
    Required,
    /// The rows attribute for textareas.
    Rows,
    /// The selected attribute for options.
    Selected,
    /// The size attribute.
    Size,
    /// The spellcheck attribute.
    SpellCheck,
    /// The src attribute for media/images.
    Src,
    /// The step attribute.
    Step,
    /// The style attribute.
    Style,
    /// The tabindex attribute.
    TabIndex,
    /// The target attribute for links/forms.
    Target,
    /// The title attribute.
    Title,
    /// The type attribute for inputs.
    Type,
    /// The value attribute.
    Value,
    /// The width attribute.
    Width,
    /// A custom attribute with an arbitrary name.
    Other(String),
}

/// Represents the value of an HTML attribute.
///
/// Attributes can be static text, reactive signals, event handlers, dynamic expressions,
/// or CSS class references.
#[derive(Clone, Debug)]
pub enum AttributeValue {
    /// A static string value.
    Text(String),
    /// A dynamic signal-backed value.
    Signal(Signal<String>),
    /// An event handler callback.
    Event(NativeEventHandler),
    /// A dynamic expression value of any type (for component props).
    Dynamic(String),
    /// A CSS class reference created by the `class!` macro.
    Css(CssClass),
}
