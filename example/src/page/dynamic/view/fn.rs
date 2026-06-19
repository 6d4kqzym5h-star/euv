use crate::*;

/// A dynamic tag demo page showcasing runtime tag switching.
///
/// Demonstrates the `{tag_expr} { content }` syntax for dynamic tags where
/// the tag name is determined at runtime based on a signal value. When the
/// signal changes, the dynamic tag automatically re-renders as either a
/// native HTML element or a user component.
///
/// # Returns
///
/// - `VirtualNode` - The dynamic component demo page virtual DOM tree.
#[component]
pub(crate) fn page_dynamic_component(node: VirtualNode<PageDynamicComponentProps>) -> VirtualNode {
    let PageDynamicComponentProps = node.try_get_props().unwrap_or_default();
    let tag_name_opt: Signal<String> = use_signal(|| DEFAULT_TAG_NAME.to_string());
    let content: Signal<String> = use_signal(|| "Hello, dynamic tag!".to_string());
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🏷️"
                title: "Dynamic Tag"
                subtitle: "Switch tags at runtime using the {tag} { content } syntax."
            }
            euv_card {
                title: "Tag Type Selection"
                p {
                    class: c_demo_text()
                    "Select a tag type below. Native HTML elements (div, span) and user components (euv_card, badge) are both supported."
                }
                div {
                    class: c_dynamic_component_tab_bar()
                    euv_button {
                        variant: if { tag_name_opt.get() == TAG_NAME_DIV } { EuvButtonVariant::Primary } else { EuvButtonVariant::Outline }
                        label: TAG_OPTION_DIV_LABEL
                        onclick: tag_on_select(tag_name_opt, TAG_NAME_DIV)
                        TAG_OPTION_DIV_LABEL
                    }
                    euv_button {
                        variant: if { tag_name_opt.get() == TAG_NAME_SPAN } { EuvButtonVariant::Primary } else { EuvButtonVariant::Outline }
                        label: TAG_OPTION_SPAN_LABEL
                        onclick: tag_on_select(tag_name_opt, TAG_NAME_SPAN)
                        TAG_OPTION_SPAN_LABEL
                    }
                    euv_button {
                        variant: if { tag_name_opt.get() == TAG_NAME_EUV_CARD } { EuvButtonVariant::Primary } else { EuvButtonVariant::Outline }
                        label: TAG_OPTION_EUV_CARD_LABEL
                        onclick: tag_on_select(tag_name_opt, TAG_NAME_EUV_CARD)
                        TAG_OPTION_EUV_CARD_LABEL
                    }
                    euv_button {
                        variant: if { tag_name_opt.get() == TAG_NAME_BADGE } { EuvButtonVariant::Primary } else { EuvButtonVariant::Outline }
                        label: TAG_OPTION_BADGE_LABEL
                        onclick: tag_on_select(tag_name_opt, TAG_NAME_BADGE)
                        TAG_OPTION_BADGE_LABEL
                    }
                }
            }
            euv_card {
                title: "Dynamic Tag Content"
                div {
                    class: c_euv_input_wrapper()
                    label {
                        for: TAG_CONTENT_INPUT_ID
                        class: c_form_label()
                        "Content Text"
                    }
                    input {
                        id: TAG_CONTENT_INPUT_ID
                        name: TAG_CONTENT_INPUT_ID
                        type: "text"
                        placeholder: TAG_CONTENT_PLACEHOLDER
                        value: content
                        autocomplete: TAG_AUTOCOMPLETE_OFF
                        class: c_euv_input()
                        oninput: content_on_input(content)
                    }
                }
            }
            euv_card {
                title: "Result"
                p {
                    class: c_demo_text_muted()
                    { format!("Current tag: {}", tag_name_opt.get()) }
                }
                div {
                    class: c_dynamic_component_panel()
                    { tag_name_opt.get() } {
                        title: "Dynamic euv_card"
                        onclick: badge_on_click("Dynamic Badge", LogLevel::Log)
                        { content.get() }
                    }
                }
            }
        }
    }
}
