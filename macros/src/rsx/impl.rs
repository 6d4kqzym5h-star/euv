use crate::*;

/// Parses RSX input into an `RsxNode` from a token stream.
impl Parse for RsxNode {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            return Ok(RsxNode::Text(lit.value()));
        }
        if input.peek(Token![if]) {
            let rsx_if: RsxIf = input.parse()?;
            return Ok(RsxNode::If(rsx_if));
        }
        if input.peek(Token![match]) {
            let rsx_match: RsxMatch = input.parse()?;
            return Ok(RsxNode::Match(rsx_match));
        }
        if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            let expr: Expr = content.parse()?;
            return Ok(RsxNode::Dynamic(expr));
        }
        if input.peek(Ident) {
            if input.peek2(syn::token::Brace) {
                let element: RsxElement = input.parse()?;
                return Ok(RsxNode::Element(element));
            }
            let expr: Expr = input.parse()?;
            return Ok(RsxNode::Expr(expr));
        }
        Err(input.error("expected an element, string literal, if, match, or expression"))
    }
}

/// Parses reactive `if {expr} { children } [else if {expr} { children }]* [else { children }]`.
impl Parse for RsxIf {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut branches: Vec<(Option<Expr>, Vec<RsxNode>)> = Vec::new();
        input.parse::<Token![if]>()?;
        let cond_content;
        braced!(cond_content in input);
        let condition: Expr = cond_content.parse()?;
        let body_content;
        braced!(body_content in input);
        let body: Vec<RsxNode> = parse_rsx_children(&body_content)?;
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
                let body: Vec<RsxNode> = parse_rsx_children(&body_content)?;
                branches.push((Some(condition), body));
            } else {
                let body_content;
                braced!(body_content in input);
                let body: Vec<RsxNode> = parse_rsx_children(&body_content)?;
                branches.push((None, body));
                break;
            }
        }
        Ok(RsxIf { branches })
    }
}

/// Parses reactive `match {expr} { pattern => { children } ... }`.
impl Parse for RsxMatch {
    fn parse(input: ParseStream) -> SynResult<Self> {
        input.parse::<Token![match]>()?;
        let scrutinee_content;
        braced!(scrutinee_content in input);
        let scrutinee: Expr = scrutinee_content.parse()?;
        let arms_content;
        braced!(arms_content in input);
        let mut arms: Vec<(TokenStream2, Vec<RsxNode>)> = Vec::new();
        while !arms_content.is_empty() {
            let mut pattern_tokens: TokenStream2 = TokenStream2::new();
            while !arms_content.peek(Token![=>]) {
                let tt: proc_macro2::TokenTree = arms_content.parse()?;
                pattern_tokens.extend(std::iter::once(tt));
            }
            arms_content.parse::<Token![=>]>()?;
            let arm_content;
            braced!(arm_content in arms_content);
            let body: Vec<RsxNode> = parse_rsx_children(&arm_content)?;
            arms.push((pattern_tokens, body));
            if arms_content.peek(Token![,]) {
                arms_content.parse::<Token![,]>()?;
            }
        }
        Ok(RsxMatch { scrutinee, arms })
    }
}

/// Parses RSX element syntax including tag, attributes, and children.
impl Parse for RsxElement {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let tag: Ident = input.parse()?;
        let tag_str: String = tag.to_string();
        let is_component: bool = tag_str.contains('_');

        let content;
        braced!(content in input);

        let mut attributes: Vec<(Ident, RsxAttrValue)> = Vec::new();
        let mut children: Vec<RsxNode> = Vec::new();

        while !content.is_empty() {
            if content.peek(LitStr) {
                let lit: LitStr = content.parse()?;
                children.push(RsxNode::Text(lit.value()));
            } else if content.peek(Token![if]) {
                let rsx_if: RsxIf = content.parse()?;
                children.push(RsxNode::If(rsx_if));
            } else if content.peek(Token![match]) {
                let rsx_match: RsxMatch = content.parse()?;
                children.push(RsxNode::Match(rsx_match));
            } else if content.peek(syn::token::Brace) {
                let child_content;
                braced!(child_content in content);
                let expr: Expr = child_content.parse()?;
                children.push(RsxNode::Dynamic(expr));
            } else if (content.peek(Ident) || content.peek(syn::LitStr)) && content.peek2(Colon) {
                let key: Ident = if content.peek(Ident) {
                    content.parse()?
                } else {
                    let lit_str: syn::LitStr = content.parse()?;
                    syn::Ident::new(&lit_str.value(), lit_str.span())
                };
                content.parse::<Colon>()?;
                let key_str: String = key.to_string();
                let value: RsxAttrValue = if key_str == "style" && content.peek(syn::token::Brace) {
                    let style_content;
                    braced!(style_content in content);
                    let is_style_object: bool =
                        style_content.peek(Ident) && style_content.peek2(Colon);
                    if is_style_object {
                        let mut style_props: Vec<(Ident, StylePropValue)> = Vec::new();
                        while !style_content.is_empty() {
                            let prop_key: Ident = style_content.parse()?;
                            style_content.parse::<Colon>()?;
                            let prop_value: StylePropValue = if style_content.peek(LitStr) {
                                let lit: LitStr = style_content.parse()?;
                                StylePropValue::Literal(lit.value())
                            } else if style_content.peek(syn::token::Brace) {
                                let expr_content;
                                braced!(expr_content in style_content);
                                let expr: Expr = expr_content.parse()?;
                                StylePropValue::Expr(expr)
                            } else {
                                let expr: Expr = style_content.parse()?;
                                StylePropValue::Expr(expr)
                            };
                            style_props.push((prop_key, prop_value));
                            if style_content.peek(Semi) {
                                style_content.parse::<Semi>()?;
                            }
                        }
                        RsxAttrValue::Style(style_props)
                    } else {
                        RsxAttrValue::Expr(style_content.parse()?)
                    }
                } else {
                    RsxAttrValue::Expr(content.parse()?)
                };
                attributes.push((key, value));
            } else if content.peek(Ident) {
                if content.peek2(syn::token::Brace) {
                    let element: RsxElement = content.parse()?;
                    children.push(RsxNode::Element(element));
                } else {
                    let expr: Expr = content.parse()?;
                    children.push(RsxNode::Expr(expr));
                }
            } else {
                return Err(content.error("unexpected token in RSX element"));
            }
        }

        Ok(RsxElement {
            tag,
            attributes,
            children,
            is_component,
        })
    }
}

/// Converts an `RsxNode` into the corresponding euv virtual node tokens.
impl ToTokens for RsxNode {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            RsxNode::Element(element) => element.to_tokens(tokens),
            RsxNode::Text(text) => {
                let text_clone: String = text.clone();
                tokens.extend(quote! {
                    euv::vdom::VirtualNode::Text(euv::vdom::TextNode::new(#text_clone.to_string(), None))
                });
            }
            RsxNode::Expr(expr) => {
                tokens.extend(quote! {
                    euv::vdom::IntoNode::into_node(#expr)
                });
            }
            RsxNode::Dynamic(expr) => {
                tokens.extend(quote! {{
                    let mut __euv_hook_context: euv::reactive::HookContext = euv::reactive::create_hook_context();
                    let __euv_render_fn: std::rc::Rc<std::cell::RefCell<dyn FnMut() -> euv::vdom::VirtualNode>> = {
                        let mut __euv_hook_context: euv::reactive::HookContext = __euv_hook_context;
                        std::rc::Rc::new(std::cell::RefCell::new(move || {
                            __euv_hook_context.reset_hook_index();
                            euv::vdom::IntoNode::into_node(#expr)
                        }))
                    };
                    euv::vdom::VirtualNode::Dynamic(euv::vdom::DynamicNode {
                        render_fn: __euv_render_fn,
                        hook_context: __euv_hook_context,
                    })
                }});
            }
            RsxNode::If(rsx_if) => {
                let mut if_chain: TokenStream2 = TokenStream2::new();
                for (i, (condition, body)) in rsx_if.branches.iter().enumerate() {
                    let body_tokens: TokenStream2 = children_to_tokens(body);
                    let body_expr: TokenStream2 = quote! {
                        euv::vdom::VirtualNode::Fragment(#body_tokens)
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
                    let mut __euv_hook_context: euv::reactive::HookContext = euv::reactive::create_hook_context();
                    let __euv_render_fn: std::rc::Rc<std::cell::RefCell<dyn FnMut() -> euv::vdom::VirtualNode>> = {
                        let mut __euv_hook_context: euv::reactive::HookContext = __euv_hook_context;
                        std::rc::Rc::new(std::cell::RefCell::new(move || {
                            __euv_hook_context.reset_hook_index();
                            #if_chain
                        }))
                    };
                    euv::vdom::VirtualNode::Dynamic(euv::vdom::DynamicNode {
                        render_fn: __euv_render_fn,
                        hook_context: __euv_hook_context,
                    })
                }});
            }
            RsxNode::Match(rsx_match) => {
                let scrutinee: &Expr = &rsx_match.scrutinee;
                let arm_tokens: Vec<TokenStream2> = rsx_match
                    .arms
                    .iter()
                    .map(|(pattern, body)| {
                        let body_tokens: TokenStream2 = children_to_tokens(body);
                        quote! {
                            #pattern => euv::vdom::VirtualNode::Fragment(#body_tokens),
                        }
                    })
                    .collect();
                tokens.extend(quote! {{
                    let mut __euv_hook_context: euv::reactive::HookContext = euv::reactive::create_hook_context();
                    let __euv_render_fn: std::rc::Rc<std::cell::RefCell<dyn FnMut() -> euv::vdom::VirtualNode>> = {
                        let mut __euv_hook_context: euv::reactive::HookContext = __euv_hook_context;
                        std::rc::Rc::new(std::cell::RefCell::new(move || {
                            __euv_hook_context.reset_hook_index();
                            match #scrutinee {
                                #(#arm_tokens)*
                            }
                        }))
                    };
                    euv::vdom::VirtualNode::Dynamic(euv::vdom::DynamicNode {
                        render_fn: __euv_render_fn,
                        hook_context: __euv_hook_context,
                    })
                }});
            }
        }
    }
}

/// Converts a `StylePropValue` into its token representation.
impl ToTokens for StylePropValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            StylePropValue::Literal(s) => s.to_tokens(tokens),
            StylePropValue::Expr(expr) => expr.to_tokens(tokens),
        }
    }
}

/// Converts an `RsxAttrValue` into its token representation.
impl ToTokens for RsxAttrValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            RsxAttrValue::Expr(expr) => expr.to_tokens(tokens),
            RsxAttrValue::Style(props) => {
                let prop_tokens: Vec<TokenStream2> = props
                    .iter()
                    .map(|(key, value)| {
                        let key_str: String = key.to_string();
                        quote! { .property(#key_str, #value) }
                    })
                    .collect();
                tokens.extend(quote! {
                    {
                        use ::euv::vdom::Style;
                        Style::default()#(#prop_tokens)*.to_css_string()
                    }
                });
            }
        }
    }
}

/// Converts an `RsxElement` into the corresponding euv virtual element tokens.
impl ToTokens for RsxElement {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let tag_name: String = self.tag.to_string();
        let is_component: bool = self.is_component;

        let attr_tokens: Vec<TokenStream2> = self.attributes.iter().map(|(key, value)| {
            let key_str: String = key.to_string();
            let value_tokens: TokenStream2 = match value {
                RsxAttrValue::Style(_) => {
                    let style_expr: TokenStream2 = quote! { #value };
                    quote! { euv::vdom::AttributeValue::Text(#style_expr) }
                }
                RsxAttrValue::Expr(expr) => {
                    let value_expr: TokenStream2 = quote! { #expr };
                    if let Some(event_name_str) = key_str.strip_prefix("on") {
                        let event_name_ident: Ident = syn::Ident::new(
                            &camel_case_event_name(event_name_str),
                            proc_macro2::Span::call_site(),
                        );
                        quote! {
                            {
                                use ::euv::{event::{NativeEventHandler, NativeEventName}, vdom::AttributeValue};
                                let __expr = #value_expr;
                                let __attr_value: AttributeValue = {
                                    struct __EventWrapper<F>(F);
                                    impl<F> __EventWrapper<F>
                                    where
                                        F: FnMut(euv::NativeEvent) + 'static,
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
                        quote! { euv::vdom::AttributeValue::Dynamic(Box::new(#value_expr)) }
                    } else {
                        quote! {
                            {
                                use ::euv::reactive::{IntoReactiveValue, IntoCallbackAttribute};
                                let __expr = #value_expr;
                                trait __IsClosure {
                                    fn __convert_closure(self) -> euv::vdom::AttributeValue;
                                }
                                impl __IsClosure for euv::NativeEventHandler {
                                    fn __convert_closure(self) -> euv::vdom::AttributeValue {
                                        euv::vdom::AttributeValue::Event(self)
                                    }
                                }
                                impl __IsClosure for Option<euv::NativeEventHandler> {
                                    fn __convert_closure(self) -> euv::vdom::AttributeValue {
                                        match self {
                                            Some(handler) => euv::vdom::AttributeValue::Event(handler),
                                            None => euv::vdom::AttributeValue::Text(String::new()),
                                        }
                                    }
                                }
                                impl<F: FnMut(euv::NativeEvent) + 'static> __IsClosure for F {
                                    fn __convert_closure(self) -> euv::vdom::AttributeValue {
                                        self.into_callback_attribute()
                                    }
                                }
                                struct __ClosurePicker<T>(T);
                                impl<T: __IsClosure> __ClosurePicker<T> {
                                    fn __pick_closure(self) -> euv::vdom::AttributeValue {
                                        self.0.__convert_closure()
                                    }
                                }
                                struct __ValuePicker<T>(T);
                                impl<T: IntoReactiveValue> __ValuePicker<T> {
                                    fn __pick_value(self) -> euv::vdom::AttributeValue {
                                        self.0.into_reactive_value()
                                    }
                                }
                                trait __FallbackHelper<T> {
                                    fn __pick(self) -> euv::vdom::AttributeValue;
                                }
                                impl<T: IntoReactiveValue> __FallbackHelper<T> for __ValuePicker<T> {
                                    fn __pick(self) -> euv::vdom::AttributeValue {
                                        self.__pick_value()
                                    }
                                }
                                impl<T: __IsClosure> __FallbackHelper<T> for __ClosurePicker<T> {
                                    fn __pick(self) -> euv::vdom::AttributeValue {
                                        self.__pick_closure()
                                    }
                                }
                                fn __dispatch<T, P: __FallbackHelper<T>>(picker: P) -> euv::vdom::AttributeValue {
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
                euv::vdom::AttributeEntry::new(#attr_name_lit.to_string(), #value_tokens)
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
                    let __children: Vec<euv::vdom::VirtualNode> = vec![#(#child_tokens),*];
                    let __props = euv::vdom::VirtualNode::Element {
                        tag: euv::vdom::Tag::Component(#tag_name.to_string()),
                        attributes: vec![#(#attr_tokens),*],
                        children: __children,
                        key: None,
                    };
                    #component_fn(__props)
                }
            });
        } else {
            tokens.extend(quote! {
                euv::vdom::VirtualNode::Element {
                    tag: euv::vdom::Tag::Element(#tag_name.to_string()),
                    attributes: vec![#(#attr_tokens),*],
                    children: vec![#(#child_tokens),*],
                    key: None,
                }
            });
        }
    }
}
