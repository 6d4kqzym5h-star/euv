use crate::*;

/// Parses zero or more HTML nodes from the macro input stream.
impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let children: Vec<HtmlNode> = parse_html_children(input)?;
        Ok(HtmlRoot { children })
    }
}

/// Converts an `HtmlRoot` into token stream based on the number of children.
///
/// - 0 children → `VirtualNode::Empty`
/// - 1 child → the child's token stream (no Fragment wrapper)
/// - N children → `VirtualNode::Fragment(vec![...])`
impl ToTokens for HtmlRoot {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let node_tokens: TokenStream2 = children_to_node_tokens(&self.children);
        tokens.extend(node_tokens);
    }
}

/// Parses HTML input into an `HtmlNode` from a token stream.
impl Parse for HtmlNode {
    fn parse(input: ParseStream) -> SynResult<Self> {
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
        if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            let expr: Expr = content.parse()?;
            return Ok(HtmlNode::Dynamic(expr));
        }
        if input.peek(Ident) {
            if input.peek2(syn::token::Brace) {
                let element: HtmlElement = input.parse()?;
                return Ok(HtmlNode::Element(element));
            }
            let expr: Expr = input.parse()?;
            return Ok(HtmlNode::Expr(expr));
        }
        Err(input.error("expected an element, string literal, if, match, for, or expression"))
    }
}

/// Parses reactive `if {expr} { children } [else if {expr} { children }]* [else { children }]`.
impl Parse for HtmlIf {
    fn parse(input: ParseStream) -> SynResult<Self> {
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
        Ok(HtmlIf { branches })
    }
}

/// Parses reactive `match {expr} { pattern => { children } ... }`.
impl Parse for HtmlMatch {
    fn parse(input: ParseStream) -> SynResult<Self> {
        input.parse::<Token![match]>()?;
        let scrutinee_content;
        braced!(scrutinee_content in input);
        let scrutinee: Expr = scrutinee_content.parse()?;
        let arms_content;
        braced!(arms_content in input);
        let mut arms: Vec<(TokenStream2, Vec<HtmlNode>)> = Vec::new();
        while !arms_content.is_empty() {
            let mut pattern_tokens: TokenStream2 = TokenStream2::new();
            while !arms_content.peek(Token![=>]) {
                let tt: proc_macro2::TokenTree = arms_content.parse()?;
                pattern_tokens.extend([tt]);
            }
            arms_content.parse::<Token![=>]>()?;
            let arm_content;
            braced!(arm_content in arms_content);
            let body: Vec<HtmlNode> = parse_html_children(&arm_content)?;
            arms.push((pattern_tokens, body));
            if arms_content.peek(Token![,]) {
                arms_content.parse::<Token![,]>()?;
            }
        }
        Ok(HtmlMatch { scrutinee, arms })
    }
}

/// Parses reactive `for pattern in {expr} { children }`.
impl Parse for HtmlFor {
    fn parse(input: ParseStream) -> SynResult<Self> {
        input.parse::<Token![for]>()?;
        let mut pattern_tokens: TokenStream2 = TokenStream2::new();
        while !input.peek(Token![in]) {
            let tt: proc_macro2::TokenTree = input.parse()?;
            pattern_tokens.extend([tt]);
        }
        input.parse::<Token![in]>()?;
        let iter_content;
        braced!(iter_content in input);
        let iterable: Expr = iter_content.parse()?;
        let body_content;
        braced!(body_content in input);
        let body: Vec<HtmlNode> = parse_html_children(&body_content)?;
        Ok(HtmlFor {
            pattern: pattern_tokens,
            iterable,
            body,
        })
    }
}

/// Parses HTML element syntax including tag, attributes, and children.
impl Parse for HtmlElement {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let tag: Ident = input.parse()?;
        let tag_str: String = tag.to_string();
        let is_component: bool = tag_str.contains('_');
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
            } else if content.peek(syn::token::Brace) {
                let child_content;
                braced!(child_content in content);
                let expr: Expr = child_content.parse()?;
                children.push(HtmlNode::Dynamic(expr));
            } else if content.peek(LitStr) && content.peek2(Colon) {
                let lit_str: LitStr = content.parse()?;
                let key: Ident = syn::Ident::new(&lit_str.value(), lit_str.span());
                content.parse::<Colon>()?;
                let key_str: String = key.to_string();
                let value: HtmlAttrValue = parse_attr_value(&content, &key_str)?;
                attributes.push((key, value));
            } else if content.peek(Ident) && (content.peek2(Colon) || content.peek2(Token![-])) {
                let key_string: String = parse_kebab_name(&content)?;
                let key_clean: &str = key_string.strip_prefix("r#").unwrap_or(&key_string);
                let key: Ident = syn::Ident::new(key_clean, content.span());
                content.parse::<Colon>()?;
                let key_str: String = key.to_string();
                let value: HtmlAttrValue = parse_attr_value(&content, &key_str)?;
                attributes.push((key, value));
            } else if content.peek(LitStr) {
                let lit: LitStr = content.parse()?;
                children.push(HtmlNode::Text(lit.value()));
            } else if content.peek(Ident) {
                if content.peek2(syn::token::Brace) {
                    let element: HtmlElement = content.parse()?;
                    children.push(HtmlNode::Element(element));
                } else {
                    let expr: Expr = content.parse()?;
                    children.push(HtmlNode::Expr(expr));
                }
            } else {
                return Err(content.error("unexpected token in HTML element"));
            }
        }
        Ok(HtmlElement {
            tag,
            attributes,
            children,
            is_component,
        })
    }
}

/// Converts an `HtmlNode` into the corresponding euv virtual node tokens.
impl ToTokens for HtmlNode {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            HtmlNode::Element(element) => element.to_tokens(tokens),
            HtmlNode::Text(text) => {
                tokens.extend(quote! {
                    euv_core::VirtualNode::Text(euv_core::TextNode::new(#text.to_string(), None))
                });
            }
            HtmlNode::Expr(expr) => {
                tokens.extend(quote! {
                    euv_core::IntoNode::into_node(#expr)
                });
            }
            HtmlNode::Dynamic(expr) => {
                tokens.extend(quote! {
                    euv_core::create_dynamic_node(move || euv_core::IntoNode::into_node(#expr))
                });
            }
            HtmlNode::If(html_if) => {
                let mut if_chain: TokenStream2 = TokenStream2::new();
                for (i, (condition, body)) in html_if.branches.iter().enumerate() {
                    let body_expr: TokenStream2 = children_to_node_tokens(body);
                    match (i, condition) {
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
                    euv_core::create_dynamic_node(move || { #if_chain })
                });
            }
            HtmlNode::Match(html_match) => {
                let scrutinee: &Expr = strip_braces_from_expr(&html_match.scrutinee);
                let arm_tokens: Vec<TokenStream2> = html_match
                    .arms
                    .iter()
                    .enumerate()
                    .map(|(arm_index, (pattern, body))| {
                        let arm_changed: bool = arm_index % 2 == 0;
                        let body_expr: TokenStream2 = children_to_node_tokens_inline(body);
                        quote! {
                            #pattern => {
                                __euv_hook_context.set_arm_changed(#arm_changed);
                                #body_expr
                            }
                        }
                    })
                    .collect();
                tokens.extend(quote! {
                    euv_core::create_dynamic_node_with_context(move |__euv_hook_context: &mut euv_core::HookContext| {
                        match #scrutinee {
                            #(#arm_tokens)*
                        }
                    })
                });
            }
            HtmlNode::For(html_for) => {
                let pattern: &TokenStream2 = &html_for.pattern;
                let iterable: &Expr = &html_for.iterable;
                let body_tokens: TokenStream2 = children_to_tokens(&html_for.body);
                tokens.extend(quote! {
                    euv_core::create_dynamic_node_with_context(move |__euv_hook_context: &mut euv_core::HookContext| {
                        let mut __euv_for_nodes: Vec<euv_core::VirtualNode> = Vec::new();
                        for #pattern in #iterable {
                            __euv_hook_context.reset_hook_index();
                            let __euv_for_body: Vec<euv_core::VirtualNode> = #body_tokens;
                            __euv_for_nodes.extend(__euv_for_body);
                        }
                        euv_core::VirtualNode::Fragment(__euv_for_nodes)
                    })
                });
            }
        }
    }
}

/// Converts a `HtmlStylePropValue` into its token representation.
impl ToTokens for HtmlStylePropValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            HtmlStylePropValue::Literal(s) => s.to_tokens(tokens),
            HtmlStylePropValue::Expr(expr) => expr.to_tokens(tokens),
            HtmlStylePropValue::If(html_attr_if) => {
                let if_chain: TokenStream2 = attr_if_to_tokens(html_attr_if);
                if_chain.to_tokens(tokens);
            }
        }
    }
}

/// Converts an `HtmlAttrValue` into its token representation.
///
/// For `HtmlAttrValue::If`, generates a reactive signal via `create_reactive_attr_signal`.
/// For `HtmlAttrValue::Style` containing `If` conditions, generates a reactive
/// signal via `create_reactive_style_attribute`.
/// For static values (`Expr` and `Style` without `If`), the value is emitted directly.
impl ToTokens for HtmlAttrValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            HtmlAttrValue::Expr(expr) => expr.to_tokens(tokens),
            HtmlAttrValue::If(html_attr_if) => {
                let if_chain: TokenStream2 = attr_if_to_tokens(html_attr_if);
                tokens.extend(quote! {
                    euv_core::create_reactive_attr_signal(move || euv_core::IntoReactiveString::into_reactive_string(#if_chain))
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
                    let prop_tokens: Vec<TokenStream2> = props
                        .iter()
                        .map(|(key, value)| {
                            quote! { .property(#key, #value) }
                        })
                        .collect();
                    tokens.extend(quote! {
                        euv_core::create_reactive_style_attribute(move || euv_core::Style::default()#(#prop_tokens)*.to_css_string())
                    });
                } else if all_literal {
                    let mut css_string: String = String::new();
                    for (key, value) in props {
                        if !css_string.is_empty() {
                            css_string.push(' ');
                        }
                        css_string.push_str(&key.replace('_', "-"));
                        css_string.push_str(": ");
                        if let HtmlStylePropValue::Literal(lit) = value {
                            css_string.push_str(lit);
                        }
                        css_string.push(';');
                    }
                    tokens.extend(quote! {
                        #css_string.to_string()
                    });
                } else {
                    let kv_tokens: Vec<TokenStream2> = props
                        .iter()
                        .map(|(key, value)| {
                            quote! { (#key, #value) }
                        })
                        .collect();
                    tokens.extend(quote! {
                        euv_core::Style::create_style_string(&[#(#kv_tokens),*])
                    });
                }
            }
        }
    }
}

/// Converts an `HtmlElement` into the corresponding euv virtual element tokens.
///
/// Only the tag name uses `quote_spanned!` so that errors related to the tag
/// (e.g., unknown element) point to the tag identifier in the user's source code.
/// All other generated code uses `quote!` with default spans, preventing the
/// entire macro invocation from being highlighted when a type error occurs in
/// generated framework code.
impl ToTokens for HtmlElement {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let tag_name: String = self.tag.to_string();
        let is_component: bool = self.is_component;
        let tag_span: Span = self.tag.span();
        let tag_literal: TokenStream2 = quote_spanned!(tag_span=> #tag_name.to_string());
        let attr_tokens: Vec<TokenStream2> = self.attributes.iter().map(|(key, value)| {
            let key_str: String = key.to_string();
            let value_tokens: TokenStream2 = match value {
                HtmlAttrValue::Style(props) => {
                    let has_if: bool = props.iter().any(|(_, v)| matches!(v, HtmlStylePropValue::If(_)));
                    if has_if {
                        quote! { #value }
                    } else {
                        quote! { euv_core::AttributeValue::Text(#value) }
                    }
                }
                HtmlAttrValue::If(_) => {
                    quote! { #value }
                }
                HtmlAttrValue::Expr(expr) => {
                    if let Some(event_name_str) = key_str.strip_prefix("on") {
                        if is_component {
                            let callback_name: String = key_str.replace('_', "-");
                            quote! {
                                euv_core::AttrValueAdapter::new(#expr).into_callback_attribute_value_with_name(#callback_name.to_string())
                            }
                        } else {
                            let event_name_ident: Ident = syn::Ident::new(
                                &camel_case_event_name(event_name_str),
                                key.span(),
                            );
                            quote! {
                                euv_core::EventAdapter::new(#expr).into_attribute(euv_core::NativeEventName::#event_name_ident)
                            }
                        }
                    } else if key_str == "children" {
                        quote! { euv_core::AttributeValue::Dynamic(Box::new(#expr)) }
                    } else {
                        quote! {
                            euv_core::AttrValueAdapter::new(#expr).into_reactive_attribute_value()
                        }
                    }
                }
            };
            let raw_key: String = if is_component {
                key_str.replace('_', "-")
            } else {
                key_str.strip_prefix("on").unwrap_or(&key_str).replace('_', "-")
            };
            let attr_name_str: String = raw_key.strip_prefix("r#").unwrap_or(&raw_key).to_string();
            quote! {
                euv_core::AttributeEntry::new(#attr_name_str.to_string(), #value_tokens)
            }
        }).collect();
        let child_tokens: Vec<TokenStream2> = self
            .children
            .iter()
            .map(|child| {
                let mut ts: TokenStream2 = TokenStream2::new();
                child.to_tokens(&mut ts);
                ts
            })
            .collect();
        if is_component {
            let component_fn: Ident = self.tag.clone();
            let component_span: Span = self.tag.span();
            let component_call: TokenStream2 =
                quote_spanned!(component_span=> #component_fn(__props));
            tokens.extend(quote! {
                {
                    let __children: Vec<euv_core::VirtualNode> = vec![#(#child_tokens),*];
                    let __props = euv_core::VirtualNode::Element {
                        tag: euv_core::Tag::Component(#tag_literal),
                        attributes: vec![#(#attr_tokens),*],
                        children: __children,
                        key: None,
                    };
                    #component_call
                }
            });
        } else {
            tokens.extend(quote! {
                euv_core::VirtualNode::Element {
                    tag: euv_core::Tag::Element(#tag_literal),
                    attributes: vec![#(#attr_tokens),*],
                    children: vec![#(#child_tokens),*],
                    key: None,
                }
            });
        }
    }
}
