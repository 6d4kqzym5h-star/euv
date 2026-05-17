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
        match self.children.len() {
            0 => {
                tokens.extend(quote! {
                    euv_core::VirtualNode::Empty
                });
            }
            1 => {
                self.children[0].to_tokens(tokens);
            }
            _ => {
                let child_tokens: TokenStream2 = children_to_tokens(&self.children);
                tokens.extend(quote! {
                    euv_core::VirtualNode::Fragment(#child_tokens)
                });
            }
        }
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
                let text_clone: String = text.clone();
                tokens.extend(quote! {
                    euv_core::VirtualNode::Text(euv_core::TextNode::new(#text_clone.to_string(), None))
                });
            }
            HtmlNode::Expr(expr) => {
                tokens.extend(quote! {
                    euv_core::IntoNode::into_node(#expr)
                });
            }
            HtmlNode::Dynamic(expr) => {
                tokens.extend(quote! {{
                    let mut __euv_hook_context: euv_core::HookContext = euv_core::create_hook_context();
                    let __euv_render_fn: std::rc::Rc<std::cell::RefCell<dyn FnMut() -> euv_core::VirtualNode>> = {
                        let mut __euv_hook_context: euv_core::HookContext = __euv_hook_context;
                        std::rc::Rc::new(std::cell::RefCell::new(move || {
                            __euv_hook_context.reset_hook_index();
                            euv_core::IntoNode::into_node(#expr)
                        }))
                    };
                    let mut __euv_dynamic_node: euv_core::DynamicNode = euv_core::DynamicNode::default();
                    __euv_dynamic_node.set_render_fn(__euv_render_fn);
                    __euv_dynamic_node.set_hook_context(__euv_hook_context);
                    euv_core::VirtualNode::Dynamic(__euv_dynamic_node)
                }});
            }
            HtmlNode::If(html_if) => {
                let mut if_chain: TokenStream2 = TokenStream2::new();
                for (i, (condition, body)) in html_if.branches.iter().enumerate() {
                    let body_tokens: TokenStream2 = children_to_tokens(body);
                    let body_expr: TokenStream2 = quote! {
                        euv_core::VirtualNode::Fragment(#body_tokens)
                    };
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
                tokens.extend(quote! {{
                    let mut __euv_hook_context: euv_core::HookContext = euv_core::create_hook_context();
                    let __euv_render_fn: std::rc::Rc<std::cell::RefCell<dyn FnMut() -> euv_core::VirtualNode>> = {
                        let mut __euv_hook_context: euv_core::HookContext = __euv_hook_context;
                        std::rc::Rc::new(std::cell::RefCell::new(move || {
                            __euv_hook_context.reset_hook_index();
                            #if_chain
                        }))
                    };
                    let mut __euv_dynamic_node: euv_core::DynamicNode = euv_core::DynamicNode::default();
                    __euv_dynamic_node.set_render_fn(__euv_render_fn);
                    __euv_dynamic_node.set_hook_context(__euv_hook_context);
                    euv_core::VirtualNode::Dynamic(__euv_dynamic_node)
                }});
            }
            HtmlNode::Match(html_match) => {
                let scrutinee: &Expr = strip_braces_from_expr(&html_match.scrutinee);
                let arm_tokens: Vec<TokenStream2> = html_match
                    .arms
                    .iter()
                    .enumerate()
                    .map(|(arm_index, (pattern, body))| {
                        let arm_changed: bool = arm_index % 2 == 0;
                        let body_tokens: TokenStream2 = children_to_tokens_inline(body);
                        quote! {
                            #pattern => {
                                __euv_hook_context.set_arm_changed(#arm_changed);
                                euv_core::VirtualNode::Fragment(#body_tokens)
                            }
                        }
                    })
                    .collect();
                tokens.extend(quote! {{
                    let mut __euv_hook_context: euv_core::HookContext = euv_core::create_hook_context();
                    let __euv_render_fn: std::rc::Rc<std::cell::RefCell<dyn FnMut() -> euv_core::VirtualNode>> = {
                        let mut __euv_hook_context: euv_core::HookContext = __euv_hook_context;
                        std::rc::Rc::new(std::cell::RefCell::new(move || {
                            __euv_hook_context.reset_hook_index();
                            match #scrutinee {
                                #(#arm_tokens)*
                            }
                        }))
                    };
                    let mut __euv_dynamic_node: euv_core::DynamicNode = euv_core::DynamicNode::default();
                    __euv_dynamic_node.set_render_fn(__euv_render_fn);
                    __euv_dynamic_node.set_hook_context(__euv_hook_context);
                    euv_core::VirtualNode::Dynamic(__euv_dynamic_node)
                }});
            }
            HtmlNode::For(html_for) => {
                let pattern: &TokenStream2 = &html_for.pattern;
                let iterable: &Expr = &html_for.iterable;
                let body_tokens: TokenStream2 = children_to_tokens(&html_for.body);
                tokens.extend(quote! {{
                    let mut __euv_hook_context: euv_core::HookContext = euv_core::create_hook_context();
                    let __euv_render_fn: std::rc::Rc<std::cell::RefCell<dyn FnMut() -> euv_core::VirtualNode>> = {
                        let mut __euv_hook_context: euv_core::HookContext = __euv_hook_context;
                        std::rc::Rc::new(std::cell::RefCell::new(move || {
                            __euv_hook_context.reset_hook_index();
                            let mut __euv_for_nodes: Vec<euv_core::VirtualNode> = Vec::new();
                            for #pattern in #iterable {
                                __euv_hook_context.reset_hook_index();
                                let __euv_for_body: Vec<euv_core::VirtualNode> = #body_tokens;
                                __euv_for_nodes.extend(__euv_for_body);
                            }
                            euv_core::VirtualNode::Fragment(__euv_for_nodes)
                        }))
                    };
                    let mut __euv_dynamic_node: euv_core::DynamicNode = euv_core::DynamicNode::default();
                    __euv_dynamic_node.set_render_fn(__euv_render_fn);
                    __euv_dynamic_node.set_hook_context(__euv_hook_context);
                    euv_core::VirtualNode::Dynamic(__euv_dynamic_node)
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
            HtmlStylePropValue::If(html_attr_if) => {
                let if_chain: TokenStream2 = attr_if_to_tokens(html_attr_if);
                if_chain.to_tokens(tokens);
            }
        }
    }
}

/// Converts an `HtmlAttrValue` into its token representation.
///
/// For `HtmlAttrValue::If`, generates a `Signal<String>` that reactively re-evaluates
/// the conditional expression whenever any signal changes, mirroring the DOM-level
/// `if {expr} { children }` mechanism that uses `DynamicNode`. The signal value is
/// derived via `IntoReactiveString`, which converts `CssClass`, `String`, `&str`,
/// and other common types to their string representation.
///
/// For `HtmlAttrValue::Style` containing `If` conditions, the same reactive signal
/// pattern is applied to the entire style CSS string.
///
/// For static values (`Expr` and `Style` without `If`), the value is emitted directly.
impl ToTokens for HtmlAttrValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            HtmlAttrValue::Expr(expr) => expr.to_tokens(tokens),
            HtmlAttrValue::If(html_attr_if) => {
                let if_chain: TokenStream2 = attr_if_to_tokens(html_attr_if);
                tokens.extend(quote! {{
                    let __euv_attr_signal: euv_core::Signal<String> = euv_core::Signal::new(
                        euv_core::IntoReactiveString::into_reactive_string(#if_chain)
                    );
                    euv_core::subscribe_attr_signal(__euv_attr_signal, move || {
                        euv_core::IntoReactiveString::into_reactive_string(#if_chain)
                    });
                    euv_core::AttributeValue::Signal(__euv_attr_signal)
                }});
            }
            HtmlAttrValue::Style(props) => {
                let has_if: bool = props
                    .iter()
                    .any(|(_, value)| matches!(value, HtmlStylePropValue::If(_)));
                let prop_tokens: Vec<TokenStream2> = props
                    .iter()
                    .map(|(key, value)| {
                        quote! { .property(#key, #value) }
                    })
                    .collect();
                if has_if {
                    tokens.extend(quote! {{
                        let __euv_attr_signal: euv_core::Signal<String> = euv_core::Signal::new(
                            euv_core::Style::default()#(#prop_tokens)*.to_css_string()
                        );
                        euv_core::subscribe_attr_signal(__euv_attr_signal, move || {
                            euv_core::Style::default()#(#prop_tokens)*.to_css_string()
                        });
                        euv_core::AttributeValue::Signal(__euv_attr_signal)
                    }});
                } else {
                    tokens.extend(quote! {
                        {
                            ::euv_core::Style::default()#(#prop_tokens)*.to_css_string()
                        }
                    });
                }
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
                HtmlAttrValue::Style(props) => {
                    let has_if: bool = props.iter().any(|(_, v)| matches!(v, HtmlStylePropValue::If(_)));
                    if has_if {
                        let style_expr: TokenStream2 = quote! { #value };
                        quote! { #style_expr }
                    } else {
                        let style_expr: TokenStream2 = quote! { #value };
                        quote! { euv_core::AttributeValue::Text(#style_expr) }
                    }
                }
                HtmlAttrValue::If(_) => {
                    let value_expr: TokenStream2 = quote! { #value };
                    quote! { #value_expr }
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
                                let __expr = #value_expr;
                                let __attr_value: ::euv_core::AttributeValue = {
                                    struct __EventWrapper<F>(F);
                                    impl<F> __EventWrapper<F>
                                    where
                                        F: FnMut(euv_core::NativeEvent) + 'static,
                                    {
                                        fn into_attr(self, name: ::euv_core::NativeEventName) -> ::euv_core::AttributeValue {
                                            ::euv_core::AttributeValue::Event(::euv_core::NativeEventHandler::new(name, self.0))
                                        }
                                    }
                                    impl __EventWrapper<::euv_core::NativeEventHandler> {
                                        fn into_attr(self, _name: ::euv_core::NativeEventName) -> ::euv_core::AttributeValue {
                                            ::euv_core::AttributeValue::Event(self.0)
                                        }
                                    }
                                    impl __EventWrapper<Option<::euv_core::NativeEventHandler>> {
                                        fn into_attr(self, _name: ::euv_core::NativeEventName) -> ::euv_core::AttributeValue {
                                            match self.0 {
                                                Some(handler) => ::euv_core::AttributeValue::Event(handler),
                                                None => ::euv_core::AttributeValue::Text(String::new()),
                                            }
                                        }
                                    }
                                    __EventWrapper(__expr).into_attr(::euv_core::NativeEventName::#event_name_ident)
                                };
                                __attr_value
                            }
                        }
                    } else if key_str == "children" {
                        quote! { euv_core::AttributeValue::Dynamic(Box::new(#value_expr)) }
                    } else {
                        quote! {
                            {
                                let __expr = #value_expr;
                                trait __IsClosure {
                                    fn __convert_closure(self) -> euv_core::AttributeValue;
                                }
                                impl __IsClosure for euv_core::NativeEventHandler {
                                    fn __convert_closure(self) -> euv_core::AttributeValue {
                                        euv_core::AttributeValue::Event(self)
                                    }
                                }
                                impl __IsClosure for Option<euv_core::NativeEventHandler> {
                                    fn __convert_closure(self) -> euv_core::AttributeValue {
                                        match self {
                                            Some(handler) => euv_core::AttributeValue::Event(handler),
                                            None => euv_core::AttributeValue::Text(String::new()),
                                        }
                                    }
                                }
                                impl<F: FnMut(euv_core::NativeEvent) + 'static> __IsClosure for F {
                                    fn __convert_closure(self) -> euv_core::AttributeValue {
                                        self.into_callback_attribute()
                                    }
                                }
                                struct __ClosurePicker<T>(T);
                                impl<T: __IsClosure> __ClosurePicker<T> {
                                    fn __pick_closure(self) -> euv_core::AttributeValue {
                                        self.0.__convert_closure()
                                    }
                                }
                                struct __ValuePicker<T>(T);
                                impl<T: ::euv_core::IntoReactiveValue> __ValuePicker<T> {
                                    fn __pick_value(self) -> euv_core::AttributeValue {
                                        self.0.into_reactive_value()
                                    }
                                }
                                trait __FallbackHelper<T> {
                                    fn __pick(self) -> euv_core::AttributeValue;
                                }
                                impl<T: ::euv_core::IntoReactiveValue> __FallbackHelper<T> for __ValuePicker<T> {
                                    fn __pick(self) -> euv_core::AttributeValue {
                                        self.__pick_value()
                                    }
                                }
                                impl<T: __IsClosure> __FallbackHelper<T> for __ClosurePicker<T> {
                                    fn __pick(self) -> euv_core::AttributeValue {
                                        self.__pick_closure()
                                    }
                                }
                                fn __dispatch<T, P: __FallbackHelper<T>>(picker: P) -> euv_core::AttributeValue {
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
                euv_core::AttributeEntry::new(#attr_name_lit.to_string(), #value_tokens)
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
                    let __children: Vec<euv_core::VirtualNode> = vec![#(#child_tokens),*];
                    let __props = euv_core::VirtualNode::Element {
                        tag: euv_core::Tag::Component(#tag_name.to_string()),
                        attributes: vec![#(#attr_tokens),*],
                        children: __children,
                        key: None,
                    };
                    #component_fn(__props)
                }
            });
        } else {
            tokens.extend(quote! {
                euv_core::VirtualNode::Element {
                    tag: euv_core::Tag::Element(#tag_name.to_string()),
                    attributes: vec![#(#attr_tokens),*],
                    children: vec![#(#child_tokens),*],
                    key: None,
                }
            });
        }
    }
}
