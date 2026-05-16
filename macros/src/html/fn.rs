use crate::*;

/// Parses the input tokens into a euv VNode expression.
///
/// Supports zero, one, or multiple root-level HTML nodes:
/// - `html! {}` → `VirtualNode::Empty`
/// - `html! { div { ... } }` → single `VirtualNode`
/// - `html! { div { ... } span { ... } }` → `VirtualNode::Fragment(vec![...])`
///
/// # Arguments
///
/// - `TokenStream`: The raw token stream representing HTML markup.
///
/// # Returns
///
/// - `TokenStream`: The generated token stream constructing the corresponding virtual node.
pub fn parse_html(input: TokenStream) -> TokenStream {
    let nodes: HtmlRoot = parse_macro_input!(input as HtmlRoot);
    let tokens: TokenStream2 = nodes.into_token_stream();
    TokenStream::from(tokens)
}

/// Converts a snake_case event name (e.g., "click", "mouse_enter") to CamelCase
/// for use as an enum variant identifier.
///
/// # Arguments
///
/// - `&str`: The snake_case event name.
///
/// # Returns
///
/// - `String`: The CamelCase event name.
pub(crate) fn camel_case_event_name(name: &str) -> String {
    let mut result: String = String::new();
    let mut capitalize_next: bool = true;
    for ch in name.chars() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }
    result
}

/// Parses a stream of tokens into a list of HTML child nodes.
///
/// # Arguments
///
/// - `ParseStream`: The parse stream containing HTML child content.
///
/// # Returns
///
/// - `SynResult<Vec<HtmlNode>>`: The parsed list of HTML child nodes, or a syntax error.
pub(crate) fn parse_html_children(content: ParseStream) -> SynResult<Vec<HtmlNode>> {
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
            break;
        } else if content.peek(Ident) {
            if content.peek2(syn::token::Brace) {
                let element: HtmlElement = content.parse()?;
                children.push(HtmlNode::Element(element));
            } else {
                let expr: Expr = content.parse()?;
                children.push(HtmlNode::Expr(expr));
            }
        } else {
            return Err(content.error("unexpected token in HTML"));
        }
    }
    Ok(children)
}

/// Converts a list of `HtmlNode` children into a `Vec<VirtualNode>` token stream.
///
/// # Arguments
///
/// - `&[HtmlNode]`: The slice of HTML child nodes to convert.
///
/// # Returns
///
/// - `TokenStream2`: The generated token stream representing a `Vec<VirtualNode>`.
pub(crate) fn children_to_tokens(children: &[HtmlNode]) -> TokenStream2 {
    let child_tokens: Vec<TokenStream2> = children
        .iter()
        .map(|child| {
            let mut ts: TokenStream2 = TokenStream2::new();
            child.to_tokens(&mut ts);
            ts
        })
        .collect();
    quote! { vec![#(#child_tokens),*] }
}

/// Parses a reactive `if {expr} { value } [else if {expr} { value }]* [else { value }]` in attribute value position.
///
/// Unlike `HtmlIf` (which contains HTML child nodes), each branch body here is a Rust expression.
///
/// # Arguments
///
/// - `ParseStream`: The parse stream positioned at the `if` keyword.
///
/// # Returns
///
/// - `SynResult<HtmlAttrIf>`: The parsed attribute-level reactive conditional.
pub(crate) fn parse_attr_if(content: ParseStream) -> SynResult<HtmlAttrIf> {
    let mut branches: Vec<(Option<Expr>, Expr)> = Vec::new();
    content.parse::<Token![if]>()?;
    let cond_content;
    braced!(cond_content in content);
    let condition: Expr = cond_content.parse()?;
    let body_content;
    braced!(body_content in content);
    let body: Expr = body_content.parse()?;
    branches.push((Some(condition), body));
    while content.peek(Token![else]) {
        content.parse::<Token![else]>()?;
        if content.peek(Token![if]) {
            content.parse::<Token![if]>()?;
            let cond_content;
            braced!(cond_content in content);
            let condition: Expr = cond_content.parse()?;
            let body_content;
            braced!(body_content in content);
            let body: Expr = body_content.parse()?;
            branches.push((Some(condition), body));
        } else {
            let body_content;
            braced!(body_content in content);
            let body: Expr = body_content.parse()?;
            branches.push((None, body));
            break;
        }
    }
    Ok(HtmlAttrIf { branches })
}

/// Strips outer braces from an `Expr` if it is an `Expr::Block` with a single expression,
/// avoiding Rust `unused_braces` warnings in generated `if` conditions.
///
/// # Arguments
///
/// - `&Expr`: The expression to potentially strip.
///
/// # Returns
///
/// - `&Expr`: The inner expression if the input was a braced single-expression block, otherwise the original.
pub(crate) fn strip_braces_from_expr(expr: &Expr) -> &Expr {
    if let Expr::Block(expr_block) = expr {
        let stmts: &Vec<syn::Stmt> = &expr_block.block.stmts;
        if stmts.len() == 1
            && let syn::Stmt::Expr(inner, None) = &stmts[0]
        {
            return inner;
        }
    }
    expr
}

/// Generates a token stream for an `HtmlAttrIf` as a Rust `if` expression.
///
/// The generated code is used inside a reactive closure so that when signals
/// change, the conditional is re-evaluated.
///
/// # Arguments
///
/// - `&HtmlAttrIf`: The parsed attribute-level reactive conditional.
///
/// # Returns
///
/// - `TokenStream2`: The generated `if ... { ... } else if ... { ... } else { ... }` token stream.
pub(crate) fn attr_if_to_tokens(html_attr_if: &HtmlAttrIf) -> TokenStream2 {
    let mut if_chain: TokenStream2 = TokenStream2::new();
    for (i, (condition, body)) in html_attr_if.branches.iter().enumerate() {
        match (i, condition) {
            (0, Some(cond)) => {
                let stripped_cond: &Expr = strip_braces_from_expr(cond);
                let stripped_body: &Expr = strip_braces_from_expr(body);
                if_chain.extend(quote! {
                    if #stripped_cond { #stripped_body }
                });
            }
            (_, Some(cond)) => {
                let stripped_cond: &Expr = strip_braces_from_expr(cond);
                let stripped_body: &Expr = strip_braces_from_expr(body);
                if_chain.extend(quote! {
                    else if #stripped_cond { #stripped_body }
                });
            }
            (_, None) => {
                let stripped_body: &Expr = strip_braces_from_expr(body);
                if_chain.extend(quote! {
                    else { #stripped_body }
                });
            }
        }
    }
    if_chain
}

/// Parses the value side of an attribute, handling the special `style:` attribute.
///
/// If the key is `"style"` and the value is a braced expression that looks like
/// a style object (key-value pairs separated by `;`), it is parsed as
/// `HtmlAttrValue::Style`. Otherwise, the value is parsed as a normal expression
/// or a reactive `if` conditional.
///
/// # Arguments
///
/// - `ParseStream`: The parse stream positioned after the `:` token.
/// - `&str`: The attribute key string (e.g., `"style"`, `"class"`).
///
/// # Returns
///
/// - `SynResult<HtmlAttrValue>`: The parsed attribute value.
pub(crate) fn parse_attr_value(content: ParseStream, key_str: &str) -> SynResult<HtmlAttrValue> {
    if content.peek(Token![if]) {
        let html_attr_if: HtmlAttrIf = parse_attr_if(content)?;
        return Ok(HtmlAttrValue::If(html_attr_if));
    }
    if key_str == "style" && content.peek(syn::token::Brace) {
        let style_content;
        braced!(style_content in content);
        let is_style_object: bool = style_content.peek(LitStr) || style_content.peek(Ident);
        if is_style_object {
            let mut style_props: Vec<(String, HtmlStylePropValue)> = Vec::new();
            while !style_content.is_empty() {
                let css_key: String = parse_kebab_name(&style_content)?;
                style_content.parse::<Colon>()?;
                let prop_value: HtmlStylePropValue = if style_content.peek(Token![if]) {
                    let html_attr_if: HtmlAttrIf = parse_attr_if(&style_content)?;
                    HtmlStylePropValue::If(html_attr_if)
                } else if style_content.peek(LitStr) {
                    let lit: LitStr = style_content.parse()?;
                    HtmlStylePropValue::Literal(lit.value())
                } else if style_content.peek(syn::token::Brace) {
                    let expr_content;
                    braced!(expr_content in style_content);
                    if expr_content.peek(Token![if]) {
                        let html_attr_if: HtmlAttrIf = parse_attr_if(&expr_content)?;
                        HtmlStylePropValue::If(html_attr_if)
                    } else {
                        let expr: Expr = expr_content.parse()?;
                        HtmlStylePropValue::Expr(expr)
                    }
                } else {
                    let expr: Expr = style_content.parse()?;
                    HtmlStylePropValue::Expr(expr)
                };
                style_props.push((css_key, prop_value));
                if style_content.peek(Semi) {
                    style_content.parse::<Semi>()?;
                }
            }
            Ok(HtmlAttrValue::Style(style_props))
        } else {
            Ok(HtmlAttrValue::Expr(style_content.parse()?))
        }
    } else {
        Ok(HtmlAttrValue::Expr(content.parse()?))
    }
}
