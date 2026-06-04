use crate::*;

/// Implementation of `Parse` for `HtmlRoot`, parsing zero or more HTML nodes.
impl Parse for HtmlRoot {
    /// Parses the root of an `html!` macro invocation.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `HtmlRoot`, or a syntax error.
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
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(children_to_node_tokens(self.get_children()));
    }
}

/// Implementation of `Parse` for `HtmlNode`, parsing HTML input into a node.
impl Parse for HtmlNode {
    /// Parses a single HTML node from the token stream.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `HtmlNode`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) && input.peek2(Brace) {
            let element: HtmlElement = input.parse()?;
            return Ok(HtmlNode::Element(element));
        }
        if input.peek(LitStr) {
            let literal_string: LitStr = input.parse()?;
            return Ok(HtmlNode::Text(literal_string.value()));
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
        if input.peek(Brace) && input.peek2(Brace) {
            let forked: ParseBuffer<'_> = input.fork();
            let _first_brace: ParseBuffer<'_>;
            braced!(_first_brace in forked);
            let second_brace: ParseBuffer<'_>;
            braced!(second_brace in forked);
            if is_dynamic_tag_pattern(&second_brace, input) {
                let tag_content: ParseBuffer<'_>;
                braced!(tag_content in input);
                let tag_expr: Expr = tag_content.parse()?;
                let body_content: ParseBuffer<'_>;
                braced!(body_content in input);
                let (attributes, children): (HtmlAttrs, Vec<HtmlNode>) =
                    parse_dynamic_component_children(&body_content)?;
                return Ok(HtmlNode::DynamicTag(HtmlDynamicTag::new(
                    tag_expr, attributes, children,
                )));
            }
        }
        if input.peek(Brace) {
            let content: ParseBuffer<'_>;
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
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `HtmlIf`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut branches: Vec<(Option<Expr>, Vec<HtmlNode>)> = Vec::new();
        input.parse::<Token![if]>()?;
        let cond_content: ParseBuffer<'_>;
        braced!(cond_content in input);
        let condition: Expr = cond_content.parse()?;
        let body_content: ParseBuffer<'_>;
        braced!(body_content in input);
        let body: Vec<HtmlNode> = parse_html_children(&body_content)?;
        branches.push((Some(condition), body));
        while input.peek(Token![else]) {
            input.parse::<Token![else]>()?;
            if input.peek(Token![if]) {
                input.parse::<Token![if]>()?;
                let cond_content: ParseBuffer<'_>;
                braced!(cond_content in input);
                let condition: Expr = cond_content.parse()?;
                let body_content: ParseBuffer<'_>;
                braced!(body_content in input);
                let body: Vec<HtmlNode> = parse_html_children(&body_content)?;
                branches.push((Some(condition), body));
            } else {
                let body_content: ParseBuffer<'_>;
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
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `HtmlMatch`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![match]>()?;
        let scrutinee_content: ParseBuffer<'_>;
        braced!(scrutinee_content in input);
        let scrutinee: Expr = scrutinee_content.parse()?;
        let arms_content: ParseBuffer<'_>;
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
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `HtmlFor`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let mut pattern_tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
        while !input.peek(Token![in]) {
            let token_tree: proc_macro2::TokenTree = input.parse()?;
            pattern_tokens.extend([token_tree]);
        }
        input.parse::<Token![in]>()?;
        let iter_content: ParseBuffer<'_>;
        braced!(iter_content in input);
        let iterable: Expr = iter_content.parse()?;
        let body_content: ParseBuffer<'_>;
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
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `HtmlElement`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (tag, tag_name, is_ident_tag): (Ident, String, bool) = if input.peek(LitStr) {
            let literal: LitStr = input.parse()?;
            let tag_name: String = literal.value();
            let tag: Ident = Ident::new(&tag_name, literal.span());
            (tag, tag_name, false)
        } else {
            let tag: Ident = input.parse()?;
            let tag_str: String = tag.to_string();
            (tag, tag_str, true)
        };
        let content: ParseBuffer<'_>;
        braced!(content in input);
        let mut attributes: HtmlAttrs = Vec::new();
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
            } else if content.peek(Brace) && content.peek2(Brace) {
                let forked: ParseBuffer<'_> = content.fork();
                let _first_brace: ParseBuffer<'_>;
                braced!(_first_brace in forked);
                let second_brace: ParseBuffer<'_>;
                braced!(second_brace in forked);
                if is_dynamic_tag_pattern(&second_brace, &content) {
                    let tag_content: ParseBuffer<'_>;
                    braced!(tag_content in content);
                    let tag_expr: Expr = tag_content.parse()?;
                    let body_content: ParseBuffer<'_>;
                    braced!(body_content in content);
                    let (dynamic_attrs, dynamic_children): (HtmlAttrs, Vec<HtmlNode>) =
                        parse_dynamic_component_children(&body_content)?;
                    children.push(HtmlNode::DynamicTag(HtmlDynamicTag::new(
                        tag_expr,
                        dynamic_attrs,
                        dynamic_children,
                    )));
                } else {
                    let child_content: ParseBuffer<'_>;
                    braced!(child_content in content);
                    let expr: Expr = child_content.parse()?;
                    children.push(HtmlNode::Dynamic(expr));
                }
            } else if content.peek(Brace) && content.peek2(Colon) {
                let key_content: ParseBuffer<'_>;
                braced!(key_content in content);
                let key_expr: Expr = key_content.parse()?;
                content.parse::<Colon>()?;
                let value: HtmlAttrValue = parse_attr_value(&content, STR_EMPTY)?;
                attributes.push((key_expr.to_token_stream(), value));
            } else if content.peek(Brace) {
                let child_content: ParseBuffer<'_>;
                braced!(child_content in content);
                let expr: Expr = child_content.parse()?;
                children.push(HtmlNode::Dynamic(expr));
            } else if content.peek(LitStr) && content.peek2(Brace) {
                let element: HtmlElement = content.parse()?;
                children.push(HtmlNode::Element(element));
            } else if content.peek(LitStr) && content.peek2(Colon) {
                let key_literal: LitStr = content.parse()?;
                let key_str: String = key_literal.value();
                content.parse::<Colon>()?;
                let value: HtmlAttrValue = parse_attr_value(&content, &key_str)?;
                attributes.push((key_literal.to_token_stream(), value));
            } else if content.peek(Ident)
                && (content.peek2(Colon) || content.peek2(Token![-]))
                && !is_double_colon(&content)
            {
                let key_string: String = parse_kebab_name(&content)?;
                let key_literal: LitStr = LitStr::new(&key_string, content.span());
                content.parse::<Colon>()?;
                let key_str: String = key_string
                    .strip_prefix(RAW_IDENT_PREFIX)
                    .unwrap_or(&key_string)
                    .to_string();
                let value: HtmlAttrValue = parse_attr_value(&content, &key_str)?;
                attributes.push((key_literal.to_token_stream(), value))
            } else if content.peek(LitStr) {
                let literal_string: LitStr = content.parse()?;
                children.push(HtmlNode::Text(literal_string.value()));
            } else if content.peek(Ident) {
                if content.peek2(Brace) {
                    let element: HtmlElement = content.parse()?;
                    children.push(HtmlNode::Element(element));
                } else if is_ident_tag
                    && is_user_fn(&tag_name)
                    && !content.peek2(Paren)
                    && get_user_fn_props_fields(&tag_name).is_some_and(|fields: &Vec<String>| {
                        let forked: ParseBuffer<'_> = content.fork();
                        forked
                            .parse::<Ident>()
                            .map(|ident: Ident| fields.contains(&ident.to_string()))
                            .unwrap_or_default()
                    })
                {
                    let key: Ident = content.parse()?;
                    let value_expr: Expr = syn::parse_quote!(#key);
                    attributes.push((key.to_token_stream(), HtmlAttrValue::Expr(value_expr)));
                } else {
                    let expr: Expr = content.parse()?;
                    children.push(HtmlNode::Expr(expr));
                }
            } else {
                return Err(content.error(ERR_UNEXPECTED_TOKEN_IN_ELEMENT));
            }
        }
        let merged_attributes: HtmlAttrs = merge_same_key_attributes(attributes);
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
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
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
                let if_chain: proc_macro2::TokenStream =
                    build_html_if_chain(html_if.get_branches());
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
                    .map(
                        |(arm_index, (pattern, body)): (
                            usize,
                            &(proc_macro2::TokenStream, Vec<HtmlNode>),
                        )| {
                            let body_expr: proc_macro2::TokenStream = children_to_node_tokens(body);
                            quote! {
                                #pattern => {
                                    __euv_hook_context.set_arm_changed(#arm_index);
                                    #body_expr
                                }
                            }
                        },
                    )
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
                    {
                        let mut __euv_for_nodes: Vec<::euv::VirtualNode> = Vec::new();
                        for #pattern in #iterable {
                            __euv_for_nodes.extend(#body_tokens);
                        }
                        ::euv::VirtualNode::Fragment(__euv_for_nodes)
                    }
                });
            }
            HtmlNode::DynamicTag(dynamic_tag) => {
                dynamic_tag.to_tokens(tokens);
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
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlStylePropValue::Literal(literal_value) => literal_value.to_tokens(tokens),
            HtmlStylePropValue::Expr(expr) => expr.to_tokens(tokens),
            HtmlStylePropValue::If(html_attr_if) => {
                let if_chain: proc_macro2::TokenStream =
                    attr_if_to_tokens(html_attr_if, quote! { #STR_EMPTY });
                if_chain.to_tokens(tokens);
            }
        }
    }
}

/// Implementation of `ToTokens` for `HtmlAttrValue`, converting attribute values into tokens.
///
/// For `HtmlAttrValue::If`, generates a reactive signal via `AttributeValue::create_reactive_signal`.
/// For `HtmlAttrValue::Style` containing `If` conditions, generates a reactive signal via
/// `AttributeValue::create_reactive_signal`.
/// For static values (`Expr` and `Style` without `If`), the value is emitted directly.
impl ToTokens for HtmlAttrValue {
    /// Converts this attribute value into its token stream representation.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlAttrValue::Expr(expr) => expr.to_tokens(tokens),
            HtmlAttrValue::If(html_attr_if) => {
                let if_chain: proc_macro2::TokenStream =
                    attr_if_to_tokens(html_attr_if, quote! { #STR_EMPTY });
                tokens.extend(quote! {
                    ::euv::AttributeValue::create_reactive_signal(move || ::euv::IntoReactiveString::into_reactive_string(#if_chain))
                });
            }
            HtmlAttrValue::Style(props) => {
                let has_if: bool = props
                    .iter()
                    .any(|(_, value): &(String, HtmlStylePropValue)| {
                        matches!(value, HtmlStylePropValue::If(_))
                    });
                let all_literal: bool =
                    props
                        .iter()
                        .all(|(_, value): &(String, HtmlStylePropValue)| {
                            matches!(value, HtmlStylePropValue::Literal(_))
                        });
                if has_if {
                    let prop_tokens: Vec<proc_macro2::TokenStream> = props
                        .iter()
                        .map(|(key, value): &(String, HtmlStylePropValue)| {
                            quote! { (#key.to_string(), ::euv::IntoReactiveString::into_reactive_string(#value)) }
                        })
                        .collect();
                    tokens.extend(quote! {
                        ::euv::AttributeValue::create_reactive_signal(move || ::euv::Css::create_style_string_owned(&[#(#prop_tokens), *]))
                    });
                } else if all_literal {
                    let mut css_string: String = String::new();
                    for (key, value) in props {
                        if !css_string.is_empty() {
                            css_string.push(CHAR_SPACE);
                        }
                        css_string.push_str(key);
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
                    let key_value_tokens: Vec<proc_macro2::TokenStream> = props
                        .iter()
                        .map(|(key, value): &(String, HtmlStylePropValue)| {
                            quote! { (#key, #value) }
                        })
                        .collect();
                    tokens.extend(quote! {
                        ::euv::Css::create_style_string(&[#(#key_value_tokens), *])
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

/// Implementation of `ToTokens` for `HtmlDynamicTag`, converting dynamic tags
/// into a reactive `DynamicNode` that re-renders when the tag expression changes.
///
/// The generated code wraps the tag expression in a `VirtualNode::create_dynamic`
/// closure. Inside the closure, the expression is evaluated to a tag name string.
/// If the tag name matches a registered user component, the component function
/// is called with default props and attributes/children are injected. Otherwise,
/// a native HTML element is created.
impl ToTokens for HtmlDynamicTag {
    /// Converts this dynamic tag into its virtual node token stream.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag_expr: &Expr = self.get_tag_expr();
        let attributes: &[(proc_macro2::TokenStream, HtmlAttrValue)] = self.get_attributes();
        let children: &[HtmlNode] = self.get_children();
        let attr_tokens: Vec<proc_macro2::TokenStream> = attributes
            .iter()
            .map(|(key, value): &(proc_macro2::TokenStream, HtmlAttrValue)| {
                let key_string: String = extract_attr_key_string(key);
                let attr_name_token: proc_macro2::TokenStream = quote! { #key_string.to_string() };
                let value_tokens: proc_macro2::TokenStream = match value {
                    HtmlAttrValue::Style(props) => {
                        let has_if: bool =
                            props
                                .iter()
                                .any(|(_, style_value): &(String, HtmlStylePropValue)| {
                                    matches!(style_value, HtmlStylePropValue::If(_))
                                });
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
                        if let Some(event_name_str) = key_string.strip_prefix(EVENT_ATTR_PREFIX) {
                            quote! {
                                ::euv::EventAdapter::new(#expr).into_attribute(#event_name_str)
                            }
                        } else if key_string == ATTR_KEY_CHILDREN {
                            quote! { ::euv::AttributeValue::Dynamic(Box::new(#expr)) }
                        } else {
                            quote! {
                                ::euv::AttrValueAdapter::new(#expr).into_reactive_attribute_value()
                            }
                        }
                    }
                };
                quote! {
                    ::euv::AttributeEntry::new(#attr_name_token, #value_tokens)
                }
            })
            .collect();
        let child_tokens: Vec<proc_macro2::TokenStream> = nodes_to_token_vec(children);
        let component_registry: HashMap<String, ComponentInfo> = get_loaded_component_registry();
        let component_match_arms: Vec<proc_macro2::TokenStream> = component_registry
            .iter()
            .map(|(fn_name, component_info): (&String, &ComponentInfo)| {
                let fn_ident: Ident = Ident::new(fn_name, proc_macro2::Span::call_site());
                let props_ident: Ident = Ident::new(component_info.get_props_type(), proc_macro2::Span::call_site());
                let fn_name_str: String = fn_name.clone();
                let props_fields: &Vec<String> = component_info.get_props_fields();
                let props_field_types: &HashMap<String, String> = component_info.get_props_field_types();
                let prop_field_tokens: Vec<proc_macro2::TokenStream> = attributes
                    .iter()
                    .filter_map(|(key, value): &(proc_macro2::TokenStream, HtmlAttrValue)| {
                        let key_string: String = extract_attr_key_string(key);
                        if !props_fields.contains(&key_string) {
                            return None;
                        }
                        let field_ident: Ident = Ident::new(&key_string, proc_macro2::Span::call_site());
                        Some(match value {
                            HtmlAttrValue::Expr(expr) => {
                                quote! { #field_ident: #expr }
                            }
                            HtmlAttrValue::If(html_attr_if) => {
                                let else_default: proc_macro2::TokenStream = match props_field_types.get(&key_string).map(|s: &String| s.as_str()) {
                                    Some(TYPE_VIRTUAL_NODE) => quote! { ::euv::VirtualNode::Empty },
                                    _ => quote! { #STR_EMPTY },
                                };
                                let if_chain: proc_macro2::TokenStream =
                                    attr_if_to_tokens(html_attr_if, else_default);
                                quote! { #field_ident: #if_chain }
                            }
                            HtmlAttrValue::Style(props) => {
                                let has_if: bool = props.iter().any(
                                    |(_, style_value): &(String, HtmlStylePropValue)| {
                                        matches!(style_value, HtmlStylePropValue::If(_))
                                    },
                                );
                                if has_if {
                                    quote! { #field_ident: #value }
                                } else {
                                    quote! { #field_ident: (#value).to_string() }
                                }
                            }
                            _ => {
                                quote! { #field_ident: #value }
                            }
                        })
                    })
                    .collect();
                let has_children_field: bool = props_fields.contains(&ATTR_KEY_CHILDREN.to_string());
                let children_token: proc_macro2::TokenStream = children_to_node_tokens(children);
                let all_prop_fields: Vec<&proc_macro2::TokenStream> = prop_field_tokens
                    .iter()
                    .collect();
                let non_prop_attr_tokens: Vec<proc_macro2::TokenStream> = attributes
                    .iter()
                    .enumerate()
                    .filter(|(_, (key, _)): &(usize, &(proc_macro2::TokenStream, HtmlAttrValue))| {
                        let key_string: String = extract_attr_key_string(key);
                        key_string == ATTR_KEY_CLASS
                            || key_string == ATTR_KEY_STYLE
                            || key_string.starts_with(EVENT_ATTR_PREFIX)
                    })
                    .map(|(index, _): (usize, &(proc_macro2::TokenStream, HtmlAttrValue))| attr_tokens[index].clone())
                    .collect();
                let props_init_tokens: proc_macro2::TokenStream = if all_prop_fields.is_empty() {
                    quote! { #props_ident::default() }
                } else {
                    quote! {
                        #[allow(clippy::needless_update)]
                        #props_ident { #(#all_prop_fields), *, ..Default::default() }
                    }
                };
                let dyn_child_tokens: Vec<proc_macro2::TokenStream> = nodes_to_token_vec(children);
                let component_call_tokens: proc_macro2::TokenStream = if has_children_field {
                    quote! { #fn_ident(::euv::VirtualNode::Element {
                        tag: ::euv::Tag::Component(#fn_name_str.to_string()),
                        attributes: vec![],
                        children: vec![#(#dyn_child_tokens), *],
                        key: None,
                        props: Some(Box::new(#props_ident { children: #children_token, ..#props_init_tokens })),
                    }) }
                } else {
                    quote! { #fn_ident(::euv::VirtualNode::Element {
                        tag: ::euv::Tag::Component(#fn_name_str.to_string()),
                        attributes: vec![],
                        children: vec![#(#dyn_child_tokens), *],
                        key: None,
                        props: Some(Box::new(#props_init_tokens)),
                    }) }
                };
                quote! {
                    #fn_name_str => {
                        let mut __euv_dyn_node: ::euv::VirtualNode = ::euv::IntoNode::into_node(#component_call_tokens);
                        if let ::euv::VirtualNode::Element { ref mut attributes, .. } = __euv_dyn_node {
                            let __euv_dyn_attrs: Vec<::euv::AttributeEntry> = vec![#(#non_prop_attr_tokens), *];
                            for __euv_dyn_attr in __euv_dyn_attrs {
                                if !__euv_dyn_attr.get_name().is_empty() {
                                    attributes.push(__euv_dyn_attr);
                                }
                            }
                        }
                        __euv_dyn_node
                    }
                }
            })
            .collect();
        tokens.extend(quote! {
            ::euv::VirtualNode::create_dynamic(move || {
                let __euv_tag_name: String = (#tag_expr).to_string();
                match __euv_tag_name.as_str() {
                    #(#component_match_arms)*
                    _ => {
                        ::euv::VirtualNode::Element {
                            tag: ::euv::Tag::Element(__euv_tag_name),
                            attributes: vec![#(#attr_tokens), *],
                            children: vec![#(#child_tokens), *],
                            key: None,
                            props: None,
                        }
                    }
                }
            })
        });
    }
}

/// Implementation of `ToTokens` for `HtmlElement`, converting HTML elements into virtual element tokens.
///
/// For identifier tags, the macro checks whether the tag name corresponds to a
/// user-defined function in the project source. If it does, the tag is treated
/// as a component function call with typed Props struct initialization and
/// children passed as a separate `VirtualNode` argument.
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
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag_name: String = self.get_tag_name().clone();
        let tag_ident: Ident = self.get_tag().clone();
        let tag_span: Span = self.get_tag().span();
        let tag_literal: proc_macro2::TokenStream =
            quote_spanned!(tag_span=> #tag_name.to_string());
        let is_component: bool = self.get_is_ident_tag() && is_user_fn(&tag_name);
        if is_component {
            let props_type_name: &str = get_user_fn_props_type(&tag_name).unwrap_or(STR_EMPTY);
            let props_type_ident: Ident = Ident::new(props_type_name, tag_span);
            let props_field_types: HashMap<String, String> =
                get_user_fn_props_field_types(&tag_name)
                    .cloned()
                    .unwrap_or_default();
            let prop_field_tokens: Vec<proc_macro2::TokenStream> = self
                .get_attributes()
                .iter()
                .map(|(key, value): &(proc_macro2::TokenStream, HtmlAttrValue)| {
                    let key_string: String = extract_attr_key_string(key);
                    let field_ident: Ident =
                        Ident::new(&key_string, proc_macro2::Span::call_site());
                    match value {
                        HtmlAttrValue::Expr(expr) => {
                            quote! { #field_ident: #expr }
                        }
                        HtmlAttrValue::If(html_attr_if) => {
                            let else_default: proc_macro2::TokenStream = match props_field_types
                                .get(&key_string)
                                .map(|s: &String| s.as_str())
                            {
                                Some(TYPE_VIRTUAL_NODE) => quote! { ::euv::VirtualNode::Empty },
                                _ => quote! { #STR_EMPTY },
                            };
                            let if_chain: proc_macro2::TokenStream =
                                attr_if_to_tokens(html_attr_if, else_default);
                            quote! { #field_ident: #if_chain }
                        }
                        HtmlAttrValue::Style(props) => {
                            let has_if: bool = props.iter().any(
                                |(_, style_value): &(String, HtmlStylePropValue)| {
                                    matches!(style_value, HtmlStylePropValue::If(_))
                                },
                            );
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
            let child_tokens: Vec<proc_macro2::TokenStream> = nodes_to_token_vec(children);
            let props_init_tokens: proc_macro2::TokenStream = if prop_field_tokens.is_empty() {
                quote! { #props_type_ident::default() }
            } else {
                quote! {
                    #[allow(clippy::needless_update)]
                    #props_type_ident { #(#prop_field_tokens), *, ..Default::default() }
                }
            };
            tokens.extend(quote! {
                #tag_ident(::euv::VirtualNode::Element {
                    tag: ::euv::Tag::Component(#tag_name.to_string()),
                    attributes: vec![],
                    children: vec![#(#child_tokens), *],
                    key: None,
                    props: Some(Box::new(#props_init_tokens)),
                })
            });
        } else {
            let mut key_expr: Option<proc_macro2::TokenStream> = None;
            let attr_tokens: Vec<proc_macro2::TokenStream> = self.get_attributes().iter().filter_map(|(key, value): &(proc_macro2::TokenStream, HtmlAttrValue)| {
                let key_string: String = extract_attr_key_string(key);
                let attr_name_token: proc_macro2::TokenStream = quote! { #key_string.to_string() };
                if key_string == ATTR_KEY_KEY {
                    if let HtmlAttrValue::Expr(expr) = value {
                        key_expr = Some(quote! { Some(::euv::IntoReactiveString::into_reactive_string(#expr)) });
                    }
                    return None;
                }
                let value_tokens: proc_macro2::TokenStream = match value {
                    HtmlAttrValue::Style(props) => {
                        let has_if: bool = props.iter().any(|(_, style_value): &(String, HtmlStylePropValue)| matches!(style_value, HtmlStylePropValue::If(_)));
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
                        if let Some(event_name_str) = key_string.strip_prefix(EVENT_ATTR_PREFIX) {
                            quote! {
                                ::euv::EventAdapter::new(#expr).into_attribute(#event_name_str)
                            }
                        } else if key_string == ATTR_KEY_CHILDREN {
                            quote! { ::euv::AttributeValue::Dynamic(Box::new(#expr)) }
                        } else {
                            quote! {
                                ::euv::AttrValueAdapter::new(#expr).into_reactive_attribute_value()
                            }
                        }
                    }
                };
                Some(quote! {
                    ::euv::AttributeEntry::new(#attr_name_token, #value_tokens)
                })
            }).collect();
            let key_token: proc_macro2::TokenStream = key_expr.unwrap_or_else(|| quote! { None });
            let children_tokens: proc_macro2::TokenStream =
                children_to_flattened_tokens(self.get_children());
            tokens.extend(quote! {
                ::euv::VirtualNode::Element {
                    tag: ::euv::Tag::Element(#tag_literal),
                    attributes: vec![#(#attr_tokens), *],
                    children: #children_tokens,
                    key: #key_token,
                    props: None,
                }
            });
        }
    }
}
