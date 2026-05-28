use crate::*;

/// Implementation of `Parse` for `HtmlRoot`, parsing zero or more HTML nodes.
impl Parse for HtmlRoot {
    /// Parses the root of an `html!` macro invocation.
    ///
    /// # Arguments
    ///
    /// - `ParseStream`- The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>`- The parsed `HtmlRoot`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let children: Vec<HtmlNode> = parse_html_children(input)?;
        Ok(Self { children })
    }
}

/// Implementation of `ToTokens` for `HtmlRoot`, converting root HTML nodes into virtual node tokens.
///
/// - 0 children → `VirtualNode::Empty`
/// - 1 child → the child's token stream (no Fragment wrapper)
/// - N children → `VirtualNode::Fragment(vec![...])`
impl ToTokens for HtmlRoot {
    /// Converts the root HTML nodes into a single virtual node token stream.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream`- The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let node_tokens: proc_macro2::TokenStream = children_to_node_tokens(self.get_children());
        tokens.extend(node_tokens);
    }
}

/// Implementation of `Parse` for `HtmlNode`, parsing HTML input into a node.
impl Parse for HtmlNode {
    /// Parses a single HTML node from the token stream.
    ///
    /// # Arguments
    ///
    /// - `ParseStream`- The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>`- The parsed `HtmlNode`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) && input.peek2(Brace) {
            let element: HtmlElement = input.parse()?;
            return Ok(HtmlNode::Element(element));
        }
        if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            return Ok(HtmlNode::Text(lit.value()));
        }
        if input.peek(Token![if]) {
            let html_if: HtmlIf = input.parse()?;
            return Ok(HtmlNode::If(html_if));
        }
        if input.peek(Token![match]) {
            let html_match: HtmlMatch = input.parse()?;
            return Ok(HtmlNode::Match(html_match));
        }
        if input.peek(Token![for]) {
            let html_for: HtmlFor = input.parse()?;
            return Ok(HtmlNode::For(html_for));
        }
        if input.peek(Brace) {
            let content;
            braced!(content in input);
            let expr: Expr = content.parse()?;
            return Ok(HtmlNode::Dynamic(expr));
        }
        if input.peek(Ident) {
            if input.peek2(Brace) {
                let element: HtmlElement = input.parse()?;
                return Ok(HtmlNode::Element(element));
            }
            let expr: Expr = input.parse()?;
            return Ok(HtmlNode::Expr(expr));
        }
        Err(input.error(ERR_EXPECTED_ELEMENT))
    }
}

/// Implementation of `Parse` for `HtmlIf`, parsing reactive `if` conditionals.
impl Parse for HtmlIf {
    /// Parses a reactive `if` conditional into an `HtmlIf` AST.
    ///
    /// # Arguments
    ///
    /// - `ParseStream`- The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>`- The parsed `HtmlIf`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut branches: Vec<(Option<Expr>, Vec<HtmlNode>)> = Vec::new();
        input.parse::<Token![if]>()?;
        let cond_content;
        braced!(cond_content in input);
        let condition: Expr = cond_content.parse()?;
        let body_content;
        braced!(body_content in input);
        let body: Vec<HtmlNode> = parse_html_children(&body_content)?;
        branches.push((Some(condition), body));
        while input.peek(Token![else]) {
            input.parse::<Token![else]>()?;
            if input.peek(Token![if]) {
                input.parse::<Token![if]>()?;
                let cond_content;
                braced!(cond_content in input);
                let condition: Expr = cond_content.parse()?;
                let body_content;
                braced!(body_content in input);
                let body: Vec<HtmlNode> = parse_html_children(&body_content)?;
                branches.push((Some(condition), body));
            } else {
                let body_content;
                braced!(body_content in input);
                let body: Vec<HtmlNode> = parse_html_children(&body_content)?;
                branches.push((None, body));
                break;
            }
        }
        Ok(Self { branches })
    }
}

/// Implementation of `Parse` for `HtmlMatch`, parsing reactive match expressions.
///
/// Each arm body can be any valid HTML content (elements, expressions, if, etc.)
/// without requiring outer braces. Bodies are terminated by `,` or end of the
/// match block.
impl Parse for HtmlMatch {
    /// Parses a reactive `match` expression into an `HtmlMatch` AST.
    ///
    /// # Arguments
    ///
    /// - `ParseStream`- The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>`- The parsed `HtmlMatch`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![match]>()?;
        let scrutinee_content;
        braced!(scrutinee_content in input);
        let scrutinee: Expr = scrutinee_content.parse()?;
        let arms_content;
        braced!(arms_content in input);
        let mut arms: Vec<(proc_macro2::TokenStream, Vec<HtmlNode>)> = Vec::new();
        while !arms_content.is_empty() {
            let mut pattern_tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            while !arms_content.peek(Token![=>]) {
                let token_tree: proc_macro2::TokenTree = arms_content.parse()?;
                pattern_tokens.extend([token_tree]);
            }
            arms_content.parse::<Token![=>]>()?;
            let body: Vec<HtmlNode> = parse_match_arm_body(&arms_content)?;
            arms.push((pattern_tokens, body));
            if arms_content.peek(Token![,]) {
                arms_content.parse::<Token![,]>()?;
            }
        }
        Ok(Self { scrutinee, arms })
    }
}

/// Implementation of `Parse` for `HtmlFor`, parsing reactive for loops.
impl Parse for HtmlFor {
    /// Parses a reactive `for` loop into an `HtmlFor` AST.
    ///
    /// # Arguments
    ///
    /// - `ParseStream`- The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>`- The parsed `HtmlFor`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let mut pattern_tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
        while !input.peek(Token![in]) {
            let token_tree: proc_macro2::TokenTree = input.parse()?;
            pattern_tokens.extend([token_tree]);
        }
        input.parse::<Token![in]>()?;
        let iter_content;
        braced!(iter_content in input);
        let iterable: Expr = iter_content.parse()?;
        let body_content;
        braced!(body_content in input);
        let body: Vec<HtmlNode> = parse_html_children(&body_content)?;
        Ok(Self {
            pattern: pattern_tokens,
            iterable,
            body,
        })
    }
}

/// Implementation of `Parse` for `HtmlElement`, parsing HTML element syntax.
impl Parse for HtmlElement {
    /// Parses an HTML element with its attributes and children.
    ///
    /// # Arguments
    ///
    /// - `ParseStream`- The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>`- The parsed `HtmlElement`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (tag, tag_name, is_ident_tag): (Ident, String, bool) = if input.peek(LitStr) {
            let literal: LitStr = input.parse()?;
            let tag_name: String = literal.value();
            let tag: Ident = Ident::new(
                &tag_name.replace(CHAR_HYPHEN, STR_UNDERSCORE),
                literal.span(),
            );
            (tag, tag_name, false)
        } else {
            let tag: Ident = input.parse()?;
            let tag_str: String = tag.to_string();
            (tag, tag_str, true)
        };
        let content;
        braced!(content in input);
        let mut attributes: Vec<(Ident, HtmlAttrValue)> = Vec::new();
        let mut children: Vec<HtmlNode> = Vec::new();
        while !content.is_empty() {
            if content.peek(Token![if]) {
                let html_if: HtmlIf = content.parse()?;
                children.push(HtmlNode::If(html_if));
            } else if content.peek(Token![match]) {
                let html_match: HtmlMatch = content.parse()?;
                children.push(HtmlNode::Match(html_match));
            } else if content.peek(Token![for]) {
                let html_for: HtmlFor = content.parse()?;
                children.push(HtmlNode::For(html_for));
            } else if content.peek(Brace) {
                let child_content;
                braced!(child_content in content);
                let expr: Expr = child_content.parse()?;
                children.push(HtmlNode::Dynamic(expr));
            } else if content.peek(LitStr) && content.peek2(Brace) {
                let element: HtmlElement = content.parse()?;
                children.push(HtmlNode::Element(element));
            } else if content.peek(LitStr) && content.peek2(Colon) {
                let literal_string: LitStr = content.parse()?;
                let key: Ident = Ident::new(&literal_string.value(), literal_string.span());
                content.parse::<Colon>()?;
                let key_str: String = key.to_string();
                let value: HtmlAttrValue = parse_attr_value(&content, &key_str)?;
                attributes.push((key, value));
            } else if content.peek(Ident) && (content.peek2(Colon) || content.peek2(Token![-])) {
                let key_string: String = parse_kebab_name(&content)?;
                let key_clean: &str = key_string
                    .strip_prefix(RAW_IDENT_PREFIX)
                    .unwrap_or(&key_string);
                let key: Ident = Ident::new(key_clean, content.span());
                content.parse::<Colon>()?;
                let key_str: String = key.to_string();
                let value: HtmlAttrValue = parse_attr_value(&content, &key_str)?;
                attributes.push((key, value));
            } else if content.peek(LitStr) {
                let lit: LitStr = content.parse()?;
                children.push(HtmlNode::Text(lit.value()));
            } else if content.peek(Ident) {
                if content.peek2(Brace) {
                    let element: HtmlElement = content.parse()?;
                    children.push(HtmlNode::Element(element));
                } else {
                    let expr: Expr = content.parse()?;
                    children.push(HtmlNode::Expr(expr));
                }
            } else {
                return Err(content.error(ERR_UNEXPECTED_TOKEN_IN_ELEMENT));
            }
        }
        let merged_attributes: Vec<(Ident, HtmlAttrValue)> = merge_same_key_attributes(attributes);
        Ok(Self {
            tag,
            tag_name,
            is_ident_tag,
            attributes: merged_attributes,
            children,
        })
    }
}

/// Implementation of `ToTokens` for `HtmlNode`, converting HTML nodes into virtual node tokens.
impl ToTokens for HtmlNode {
    /// Converts this HTML node into its corresponding virtual node token stream.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream`- The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlNode::Element(element) => element.to_tokens(tokens),
            HtmlNode::Text(text) => {
                tokens.extend(quote! {
                    ::euv::VirtualNode::Text(::euv::TextNode::new(#text.to_string(), None))
                });
            }
            HtmlNode::Expr(expr) => {
                tokens.extend(quote! {
                    ::euv::IntoNode::into_node(#expr)
                });
            }
            HtmlNode::Dynamic(expr) => {
                tokens.extend(quote! {
                    ::euv::VirtualNode::create_dynamic(move || ::euv::IntoNode::into_node(#expr))
                });
            }
            HtmlNode::If(html_if) => {
                let mut if_chain: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
                for (branch_index, (condition, body)) in html_if.get_branches().iter().enumerate() {
                    let body_expr: proc_macro2::TokenStream = children_to_node_tokens(body);
                    match (branch_index, condition) {
                        (0, Some(cond)) => {
                            let stripped_cond: &Expr = strip_braces_from_expr(cond);
                            if_chain.extend(quote! {
                                if #stripped_cond {
                                    #body_expr
                                }
                            });
                        }
                        (_, Some(cond)) => {
                            let stripped_cond: &Expr = strip_braces_from_expr(cond);
                            if_chain.extend(quote! {
                                else if #stripped_cond {
                                    #body_expr
                                }
                            });
                        }
                        (_, None) => {
                            if_chain.extend(quote! {
                                else {
                                    #body_expr
                                }
                            });
                        }
                    }
                }
                tokens.extend(quote! {
                    ::euv::VirtualNode::create_dynamic(move || { #if_chain })
                });
            }
            HtmlNode::Match(html_match) => {
                let scrutinee: &Expr = strip_braces_from_expr(html_match.get_scrutinee());
                let arm_tokens: Vec<proc_macro2::TokenStream> = html_match
                    .get_arms()
                    .iter()
                    .enumerate()
                    .map(|(arm_index, (pattern, body))| {
                        let body_expr: proc_macro2::TokenStream = children_to_node_tokens(body);
                        quote! {
                            #pattern => {
                                __euv_hook_context.set_arm_changed(#arm_index);
                                #body_expr
                            }
                        }
                    })
                    .collect();
                tokens.extend(quote! {
                    ::euv::VirtualNode::create_dynamic_with_context(move |__euv_hook_context: &mut ::euv::HookContext| {
                        match #scrutinee {
                            #(#arm_tokens)*
                        }
                    })
                });
            }
            HtmlNode::For(html_for) => {
                let pattern: &proc_macro2::TokenStream = html_for.get_pattern();
                let iterable: &Expr = html_for.get_iterable();
                let body_tokens: proc_macro2::TokenStream = children_to_tokens(html_for.get_body());
                tokens.extend(quote! {
                    ::euv::VirtualNode::create_dynamic_with_context(move |__euv_hook_context: &mut ::euv::HookContext| {
                        let mut __euv_for_nodes: Vec<::euv::VirtualNode> = Vec::new();
                        for #pattern in #iterable {
                            __euv_hook_context.reset_hook_index();
                            __euv_for_nodes.extend(#body_tokens);
                        }
                        ::euv::VirtualNode::Fragment(__euv_for_nodes)
                    })
                });
            }
        }
    }
}

/// Implementation of `ToTokens` for `HtmlStylePropValue`, converting style property values into tokens.
impl ToTokens for HtmlStylePropValue {
    /// Converts this style property value into its token stream representation.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream`- The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlStylePropValue::Literal(literal_value) => literal_value.to_tokens(tokens),
            HtmlStylePropValue::Expr(expr) => expr.to_tokens(tokens),
            HtmlStylePropValue::If(html_attr_if) => {
                let if_chain: proc_macro2::TokenStream = attr_if_to_tokens(html_attr_if);
                if_chain.to_tokens(tokens);
            }
        }
    }
}

/// Implementation of `ToTokens` for `HtmlAttrValue`, converting attribute values into tokens.
///
/// For `HtmlAttrValue::If`, generates a reactive signal via `AttributeValue::create_reactive_signal`.
/// For `HtmlAttrValue::Style` containing `If` conditions, generates a reactive
/// signal via `AttributeValue::create_reactive_style`.
/// For static values (`Expr` and `Style` without `If`), the value is emitted directly.
impl ToTokens for HtmlAttrValue {
    /// Converts this attribute value into its token stream representation.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream`- The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlAttrValue::Expr(expr) => expr.to_tokens(tokens),
            HtmlAttrValue::If(html_attr_if) => {
                let if_chain: proc_macro2::TokenStream = attr_if_to_tokens(html_attr_if);
                tokens.extend(quote! {
                    ::euv::AttributeValue::create_reactive_signal(move || ::euv::IntoReactiveString::into_reactive_string(#if_chain))
                });
            }
            HtmlAttrValue::Style(props) => {
                let has_if: bool = props
                    .iter()
                    .any(|(_, value)| matches!(value, HtmlStylePropValue::If(_)));
                let all_literal: bool = props
                    .iter()
                    .all(|(_, value)| matches!(value, HtmlStylePropValue::Literal(_)));
                if has_if {
                    let prop_tokens: Vec<proc_macro2::TokenStream> = props
                        .iter()
                        .map(|(key, value)| {
                            quote! { .property(#key, #value) }
                        })
                        .collect();
                    tokens.extend(quote! {
                        ::euv::AttributeValue::create_reactive_style(move || ::euv::Style::default()#(#prop_tokens)*.to_css_string())
                    });
                } else if all_literal {
                    let mut css_string: String = String::new();
                    for (key, value) in props {
                        if !css_string.is_empty() {
                            css_string.push(CHAR_SPACE);
                        }
                        css_string.push_str(&key.replace(CHAR_UNDERSCORE, STR_HYPHEN));
                        css_string.push_str(CSS_PROP_SEPARATOR);
                        if let HtmlStylePropValue::Literal(literal_value) = value {
                            css_string.push_str(literal_value);
                        }
                        css_string.push(CHAR_CSS_DECL_TERMINATOR);
                    }
                    tokens.extend(quote! {
                        #css_string.to_string()
                    });
                } else {
                    let kv_tokens: Vec<proc_macro2::TokenStream> = props
                        .iter()
                        .map(|(key, value)| {
                            quote! { (#key, #value) }
                        })
                        .collect();
                    tokens.extend(quote! {
                        ::euv::Style::create_style_string(&[#(#kv_tokens), *])
                    });
                }
            }
            HtmlAttrValue::Classes(values) => {
                let value_tokens: Vec<proc_macro2::TokenStream> = values
                    .iter()
                    .map(|value: &HtmlAttrValue| {
                        attr_value_to_attribute_value_tokens(value, ATTR_KEY_CLASS, false)
                    })
                    .collect();
                tokens.extend(quote! {
                    ::euv::AttributeValue::merge_class(&[#(#value_tokens), *])
                });
            }
            HtmlAttrValue::Styles(values) => {
                let value_tokens: Vec<proc_macro2::TokenStream> = values
                    .iter()
                    .map(style_value_to_attribute_value_tokens)
                    .collect();
                tokens.extend(quote! {
                    ::euv::AttributeValue::merge_style(&[#(#value_tokens), *])
                });
            }
        }
    }
}

/// Implementation of `ToTokens` for `HtmlElement`, converting HTML elements into virtual element tokens.
///
/// For identifier tags, the macro checks whether the tag name corresponds to a
/// user-defined function in the project source. If it does, the tag is treated
/// as a component function call with typed Props struct initialization and
/// children passed as a separate `Vec<VirtualNode>` argument.
///
/// For non-component tags (HTML elements), the existing `VirtualNode::Element`
/// construction with `Tag::Element` is preserved.
///
/// String literal tags always produce `Tag::Element`, supporting custom
/// HTML5 elements (Web Components).
impl ToTokens for HtmlElement {
    /// Converts this HTML element into its virtual element token stream.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream`- The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag_name: String = self.get_tag_name().clone();
        let tag_ident: Ident = self.get_tag().clone();
        let tag_span: Span = self.get_tag().span();
        let tag_literal: proc_macro2::TokenStream =
            quote_spanned!(tag_span=> #tag_name.to_string());
        let is_component: bool = self.get_is_ident_tag() && is_user_fn(&tag_name);
        if is_component {
            let props_type_name: &str = get_user_fn_props_type(&tag_name).unwrap_or("");
            let props_type_ident: Ident = Ident::new(props_type_name, tag_span);
            let prop_field_tokens: Vec<proc_macro2::TokenStream> = self
                .get_attributes()
                .iter()
                .map(|(key, value)| {
                    let field_ident: Ident = key.clone();
                    match value {
                        HtmlAttrValue::Expr(expr) => {
                            quote! { #field_ident: #expr }
                        }
                        HtmlAttrValue::If(html_attr_if) => {
                            let if_chain: proc_macro2::TokenStream =
                                attr_if_to_tokens(html_attr_if);
                            quote! { #field_ident: #if_chain }
                        }
                        HtmlAttrValue::Style(props) => {
                            let has_if: bool = props.iter().any(|(_, style_value)| {
                                matches!(style_value, HtmlStylePropValue::If(_))
                            });
                            if has_if {
                                quote! { #field_ident: #value }
                            } else {
                                quote! { #field_ident: (#value).to_string() }
                            }
                        }
                        _ => {
                            quote! { #field_ident: #value }
                        }
                    }
                })
                .collect();
            let children: &[HtmlNode] = self.get_children();
            let children_field_token: Option<proc_macro2::TokenStream> = if children.is_empty() {
                None
            } else {
                let children_node_tokens: proc_macro2::TokenStream =
                    children_to_node_tokens(children);
                Some(quote! { children: #children_node_tokens })
            };
            let all_fields: Vec<&proc_macro2::TokenStream> = prop_field_tokens
                .iter()
                .chain(children_field_token.as_ref())
                .collect();
            tokens.extend(quote! {
                #tag_ident(
                    #props_type_ident { #(#all_fields), *, ..Default::default() },
                )
            });
        } else {
            let mut key_expr: Option<proc_macro2::TokenStream> = None;
            let attr_tokens: Vec<proc_macro2::TokenStream> = self.get_attributes().iter().filter_map(|(key, value)| {
                let key_str: String = key.to_string();
                if key_str == "key" {
                    if let HtmlAttrValue::Expr(expr) = value {
                        key_expr = Some(quote! { Some(::euv::IntoReactiveString::into_reactive_string(#expr)) });
                    }
                    return None;
                }
                let value_tokens: proc_macro2::TokenStream = match value {
                    HtmlAttrValue::Style(props) => {
                        let has_if: bool = props.iter().any(|(_, style_value)| matches!(style_value, HtmlStylePropValue::If(_)));
                        if has_if {
                            quote! { #value }
                        } else {
                            quote! { ::euv::AttributeValue::Text(#value) }
                        }
                    }
                    HtmlAttrValue::If(_) => {
                        quote! { #value }
                    }
                    HtmlAttrValue::Classes(_) => {
                        quote! { #value }
                    }
                    HtmlAttrValue::Styles(_) => {
                        quote! { #value }
                    }
                    HtmlAttrValue::Expr(expr) => {
                        if let Some(event_name_str) = key_str.strip_prefix(EVENT_ATTR_PREFIX) {
                            quote! {
                                ::euv::EventAdapter::new(#expr).into_attribute(#event_name_str.parse::<::euv::NativeEventName>().unwrap())
                            }
                        } else if key_str == ATTR_KEY_CHILDREN {
                            quote! { ::euv::AttributeValue::Dynamic(Box::new(#expr)) }
                        } else {
                            quote! {
                                ::euv::AttrValueAdapter::new(#expr).into_reactive_attribute_value()
                            }
                        }
                    }
                };
                let is_event: bool = key_str.starts_with(EVENT_ATTR_PREFIX);
                let raw_key: String = if is_event {
                    key_str.clone()
                } else {
                    key_str.replace(CHAR_UNDERSCORE, STR_HYPHEN)
                };
                let attr_name_str: String = raw_key.strip_prefix(RAW_IDENT_PREFIX).unwrap_or(&raw_key).to_string();
                Some(quote! {
                    ::euv::AttributeEntry::new(#attr_name_str.to_string(), #value_tokens)
                })
            }).collect();
            let key_token: proc_macro2::TokenStream = key_expr.unwrap_or_else(|| quote! { None });
            let child_tokens: Vec<proc_macro2::TokenStream> = self
                .get_children()
                .iter()
                .map(|child: &HtmlNode| {
                    let mut token_stream: proc_macro2::TokenStream =
                        proc_macro2::TokenStream::new();
                    child.to_tokens(&mut token_stream);
                    token_stream
                })
                .collect();
            tokens.extend(quote! {
                ::euv::VirtualNode::Element {
                    tag: ::euv::Tag::Element(#tag_literal),
                    attributes: vec![#(#attr_tokens), *],
                    children: vec![#(#child_tokens), *],
                    key: #key_token,
                }
            });
        }
    }
}
