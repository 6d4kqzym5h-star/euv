use crate::*;

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
            if content.peek(LitStr) {
                let lit: LitStr = content.parse()?;
                children.push(HtmlNode::Text(lit.value()));
            } else if content.peek(Token![if]) {
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
            } else if (content.peek(Ident) || content.peek(syn::LitStr)) && content.peek2(Colon) {
                let key: Ident = if content.peek(Ident) {
                    content.parse()?
                } else {
                    let lit_str: syn::LitStr = content.parse()?;
                    syn::Ident::new(&lit_str.value(), lit_str.span())
                };
                content.parse::<Colon>()?;
                let key_str: String = key.to_string();
                let value: HtmlAttrValue = if key_str == "style" && content.peek(syn::token::Brace)
                {
                    let style_content;
                    braced!(style_content in content);
                    let is_style_object: bool =
                        style_content.peek(Ident) && style_content.peek2(Colon);
                    if is_style_object {
                        let mut style_props: Vec<(Ident, HtmlStylePropValue)> = Vec::new();
                        while !style_content.is_empty() {
                            let prop_key: Ident = style_content.parse()?;
                            style_content.parse::<Colon>()?;
                            let prop_value: HtmlStylePropValue = if style_content.peek(LitStr) {
                                let lit: LitStr = style_content.parse()?;
                                HtmlStylePropValue::Literal(lit.value())
                            } else if style_content.peek(syn::token::Brace) {
                                let expr_content;
                                braced!(expr_content in style_content);
                                let expr: Expr = expr_content.parse()?;
                                HtmlStylePropValue::Expr(expr)
                            } else {
                                let expr: Expr = style_content.parse()?;
                                HtmlStylePropValue::Expr(expr)
                            };
                            style_props.push((prop_key, prop_value));
                            if style_content.peek(Semi) {
                                style_content.parse::<Semi>()?;
                            }
                        }
                        HtmlAttrValue::Style(style_props)
                    } else {
                        HtmlAttrValue::Expr(style_content.parse()?)
                    }
                } else {
                    HtmlAttrValue::Expr(content.parse()?)
                };
                attributes.push((key, value));
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
                let text_clone: String = text.clone();
                tokens.extend(quote! {
                    euv_core::vdom::VirtualNode::Text(euv_core::vdom::TextNode::new(#text_clone.to_string(), None))
                });
            }
            HtmlNode::Expr(expr) => {
                tokens.extend(quote! {
                    euv_core::vdom::IntoNode::into_node(#expr)
                });
            }
            HtmlNode::Dynamic(expr) => {
                tokens.extend(quote! {{
                    let mut __euv_hook_context: euv_core::reactive::HookContext = euv_core::reactive::create_hook_context();
                    let __euv_render_fn: std::rc::Rc<std::cell::RefCell<dyn FnMut() -> euv_core::vdom::VirtualNode>> = {
                        let mut __euv_hook_context: euv_core::reactive::HookContext = __euv_hook_context;
                        std::rc::Rc::new(std::cell::RefCell::new(move || {
                            __euv_hook_context.reset_hook_index();
                            euv_core::vdom::IntoNode::into_node(#expr)
                        }))
                    };
                    euv_core::vdom::VirtualNode::Dynamic(euv_core::vdom::DynamicNode {
                        render_fn: __euv_render_fn,
                        hook_context: __euv_hook_context,
                    })
                }});
            }
            HtmlNode::If(html_if) => {
                let mut if_chain: TokenStream2 = TokenStream2::new();
                for (i, (condition, body)) in html_if.branches.iter().enumerate() {
                    let body_tokens: TokenStream2 = children_to_tokens(body);
                    let body_expr: TokenStream2 = quote! {
                        euv_core::vdom::VirtualNode::Fragment(#body_tokens)
                    };
                    match (i, condition) {
                        (0, Some(cond)) => {
                            if_chain.extend(quote! {
                                if #cond {
                                    #body_expr
                                }
                            });
                        }
                        (_, Some(cond)) => {
                            if_chain.extend(quote! {
                                else if #cond {
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
                tokens.extend(quote! {{
                    let mut __euv_hook_context: euv_core::reactive::HookContext = euv_core::reactive::create_hook_context();
                    let __euv_render_fn: std::rc::Rc<std::cell::RefCell<dyn FnMut() -> euv_core::vdom::VirtualNode>> = {
                        let mut __euv_hook_context: euv_core::reactive::HookContext = __euv_hook_context;
                        std::rc::Rc::new(std::cell::RefCell::new(move || {
                            __euv_hook_context.reset_hook_index();
                            #if_chain
                        }))
                    };
                    euv_core::vdom::VirtualNode::Dynamic(euv_core::vdom::DynamicNode {
                        render_fn: __euv_render_fn,
                        hook_context: __euv_hook_context,
                    })
                }});
            }
            HtmlNode::Match(html_match) => {
                let scrutinee: &Expr = &html_match.scrutinee;
                let arm_tokens: Vec<TokenStream2> = html_match
                    .arms
                    .iter()
                    .map(|(pattern, body)| {
                        let body_tokens: TokenStream2 = children_to_tokens(body);
                        quote! {
                            #pattern => euv_core::vdom::VirtualNode::Fragment(#body_tokens),
                        }
                    })
                    .collect();
                tokens.extend(quote! {{
                    let mut __euv_hook_context: euv_core::reactive::HookContext = euv_core::reactive::create_hook_context();
                    let __euv_render_fn: std::rc::Rc<std::cell::RefCell<dyn FnMut() -> euv_core::vdom::VirtualNode>> = {
                        let mut __euv_hook_context: euv_core::reactive::HookContext = __euv_hook_context;
                        std::rc::Rc::new(std::cell::RefCell::new(move || {
                            __euv_hook_context.reset_hook_index();
                            match #scrutinee {
                                #(#arm_tokens)*
                            }
                        }))
                    };
                    euv_core::vdom::VirtualNode::Dynamic(euv_core::vdom::DynamicNode {
                        render_fn: __euv_render_fn,
                        hook_context: __euv_hook_context,
                    })
                }});
            }
            HtmlNode::For(html_for) => {
                let pattern: &TokenStream2 = &html_for.pattern;
                let iterable: &Expr = &html_for.iterable;
                let body_tokens: TokenStream2 = children_to_tokens(&html_for.body);
                tokens.extend(quote! {{
                    let mut __euv_hook_context: euv_core::reactive::HookContext = euv_core::reactive::create_hook_context();
                    let __euv_render_fn: std::rc::Rc<std::cell::RefCell<dyn FnMut() -> euv_core::vdom::VirtualNode>> = {
                        let mut __euv_hook_context: euv_core::reactive::HookContext = __euv_hook_context;
                        std::rc::Rc::new(std::cell::RefCell::new(move || {
                            __euv_hook_context.reset_hook_index();
                            let mut __euv_for_nodes: Vec<euv_core::vdom::VirtualNode> = Vec::new();
                            for #pattern in #iterable {
                                __euv_hook_context.reset_hook_index();
                                let __euv_for_body: Vec<euv_core::vdom::VirtualNode> = #body_tokens;
                                __euv_for_nodes.extend(__euv_for_body);
                            }
                            euv_core::vdom::VirtualNode::Fragment(__euv_for_nodes)
                        }))
                    };
                    euv_core::vdom::VirtualNode::Dynamic(euv_core::vdom::DynamicNode {
                        render_fn: __euv_render_fn,
                        hook_context: __euv_hook_context,
                    })
                }});
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
        }
    }
}

/// Converts an `HtmlAttrValue` into its token representation.
impl ToTokens for HtmlAttrValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            HtmlAttrValue::Expr(expr) => expr.to_tokens(tokens),
            HtmlAttrValue::Style(props) => {
                let prop_tokens: Vec<TokenStream2> = props
                    .iter()
                    .map(|(key, value)| {
                        let key_str: String = key.to_string();
                        quote! { .property(#key_str, #value) }
                    })
                    .collect();
                tokens.extend(quote! {
                    {
                        use ::euv_core::vdom::Style;
                        Style::default()#(#prop_tokens)*.to_css_string()
                    }
                });
            }
        }
    }
}

/// Converts an `HtmlElement` into the corresponding euv virtual element tokens.
impl ToTokens for HtmlElement {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let tag_name: String = self.tag.to_string();
        let is_component: bool = self.is_component;

        let attr_tokens: Vec<TokenStream2> = self.attributes.iter().map(|(key, value)| {
            let key_str: String = key.to_string();
            let value_tokens: TokenStream2 = match value {
                HtmlAttrValue::Style(_) => {
                    let style_expr: TokenStream2 = quote! { #value };
                    quote! { euv_core::vdom::AttributeValue::Text(#style_expr) }
                }
                HtmlAttrValue::Expr(expr) => {
                    let value_expr: TokenStream2 = quote! { #expr };
                    if let Some(event_name_str) = key_str.strip_prefix("on") {
                        let event_name_ident: Ident = syn::Ident::new(
                            &camel_case_event_name(event_name_str),
                            proc_macro2::Span::call_site(),
                        );
                        quote! {
                            {
                                use ::euv_core::{event::{NativeEventHandler, NativeEventName}, vdom::AttributeValue};
                                let __expr = #value_expr;
                                let __attr_value: AttributeValue = {
                                    struct __EventWrapper<F>(F);
                                    impl<F> __EventWrapper<F>
                                    where
                                        F: FnMut(euv_core::NativeEvent) + 'static,
                                    {
                                        fn into_attr(self, name: NativeEventName) -> AttributeValue {
                                            AttributeValue::Event(NativeEventHandler::new(name, self.0))
                                        }
                                    }
                                    impl __EventWrapper<NativeEventHandler> {
                                        fn into_attr(self, _name: NativeEventName) -> AttributeValue {
                                            AttributeValue::Event(self.0)
                                        }
                                    }
                                    impl __EventWrapper<Option<NativeEventHandler>> {
                                        fn into_attr(self, _name: NativeEventName) -> AttributeValue {
                                            match self.0 {
                                                Some(handler) => AttributeValue::Event(handler),
                                                None => AttributeValue::Text(String::new()),
                                            }
                                        }
                                    }
                                    __EventWrapper(__expr).into_attr(NativeEventName::#event_name_ident)
                                };
                                __attr_value
                            }
                        }
                    } else if key_str == "children" {
                        quote! { euv_core::vdom::AttributeValue::Dynamic(Box::new(#value_expr)) }
                    } else {
                        quote! {
                            {
                                use ::euv_core::reactive::{IntoReactiveValue, IntoCallbackAttribute};
                                let __expr = #value_expr;
                                trait __IsClosure {
                                    fn __convert_closure(self) -> euv_core::vdom::AttributeValue;
                                }
                                impl __IsClosure for euv_core::NativeEventHandler {
                                    fn __convert_closure(self) -> euv_core::vdom::AttributeValue {
                                        euv_core::vdom::AttributeValue::Event(self)
                                    }
                                }
                                impl __IsClosure for Option<euv_core::NativeEventHandler> {
                                    fn __convert_closure(self) -> euv_core::vdom::AttributeValue {
                                        match self {
                                            Some(handler) => euv_core::vdom::AttributeValue::Event(handler),
                                            None => euv_core::vdom::AttributeValue::Text(String::new()),
                                        }
                                    }
                                }
                                impl<F: FnMut(euv_core::NativeEvent) + 'static> __IsClosure for F {
                                    fn __convert_closure(self) -> euv_core::vdom::AttributeValue {
                                        self.into_callback_attribute()
                                    }
                                }
                                struct __ClosurePicker<T>(T);
                                impl<T: __IsClosure> __ClosurePicker<T> {
                                    fn __pick_closure(self) -> euv_core::vdom::AttributeValue {
                                        self.0.__convert_closure()
                                    }
                                }
                                struct __ValuePicker<T>(T);
                                impl<T: IntoReactiveValue> __ValuePicker<T> {
                                    fn __pick_value(self) -> euv_core::vdom::AttributeValue {
                                        self.0.into_reactive_value()
                                    }
                                }
                                trait __FallbackHelper<T> {
                                    fn __pick(self) -> euv_core::vdom::AttributeValue;
                                }
                                impl<T: IntoReactiveValue> __FallbackHelper<T> for __ValuePicker<T> {
                                    fn __pick(self) -> euv_core::vdom::AttributeValue {
                                        self.__pick_value()
                                    }
                                }
                                impl<T: __IsClosure> __FallbackHelper<T> for __ClosurePicker<T> {
                                    fn __pick(self) -> euv_core::vdom::AttributeValue {
                                        self.__pick_closure()
                                    }
                                }
                                fn __dispatch<T, P: __FallbackHelper<T>>(picker: P) -> euv_core::vdom::AttributeValue {
                                    picker.__pick()
                                }
                                __dispatch::<_, __ValuePicker<_>>(__ValuePicker(__expr))
                            }
                        }
                    }
                }
            };
            let raw_key: String = key_str.strip_prefix("on").unwrap_or(&key_str).replace('_', "-");
            let attr_name_str: String = raw_key.strip_prefix("r#").unwrap_or(&raw_key).to_string();
            let attr_name_lit: LitStr = syn::LitStr::new(
                &attr_name_str,
                proc_macro2::Span::call_site(),
            );
            quote! {
                euv_core::vdom::AttributeEntry::new(#attr_name_lit.to_string(), #value_tokens)
            }
        }).collect();

        let child_tokens: Vec<TokenStream2> = self
            .children
            .iter()
            .map(|child| {
                let child_stream: TokenStream2 = {
                    let mut ts: TokenStream2 = TokenStream2::new();
                    child.to_tokens(&mut ts);
                    ts
                };
                quote! { #child_stream }
            })
            .collect();

        if is_component {
            let component_fn: Ident = self.tag.clone();
            tokens.extend(quote! {
                {
                    let __children: Vec<euv_core::vdom::VirtualNode> = vec![#(#child_tokens),*];
                    let __props = euv_core::vdom::VirtualNode::Element {
                        tag: euv_core::vdom::Tag::Component(#tag_name.to_string()),
                        attributes: vec![#(#attr_tokens),*],
                        children: __children,
                        key: None,
                    };
                    #component_fn(__props)
                }
            });
        } else {
            tokens.extend(quote! {
                euv_core::vdom::VirtualNode::Element {
                    tag: euv_core::vdom::Tag::Element(#tag_name.to_string()),
                    attributes: vec![#(#attr_tokens),*],
                    children: vec![#(#child_tokens),*],
                    key: None,
                }
            });
        }
    }
}
