use crate::*;

/// Parses the input tokens into a euv VNode expression.
///
/// # Arguments
///
/// - `TokenStream`: The raw token stream representing HTML markup.
///
/// # Returns
///
/// - `TokenStream`: The generated token stream constructing the corresponding virtual node.
pub fn parse_html(input: TokenStream) -> TokenStream {
    let root: HtmlNode = parse_macro_input!(input as HtmlNode);
    let tokens: TokenStream2 = root.into_token_stream();
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
