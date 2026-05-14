use crate::*;

/// Parses the input tokens into a euv VNode expression.
///
/// # Arguments
///
/// - `TokenStream`: The raw token stream representing RSX markup.
///
/// # Returns
///
/// - `TokenStream`: The generated token stream constructing the corresponding virtual node.
pub fn parse_rsx(input: TokenStream) -> TokenStream {
    let root: RsxNode = parse_macro_input!(input as RsxNode);
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

/// Parses a stream of tokens into a list of RSX child nodes.
pub(crate) fn parse_rsx_children(content: ParseStream) -> SynResult<Vec<RsxNode>> {
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
            break;
        } else if content.peek(Ident) {
            if content.peek2(syn::token::Brace) {
                let element: RsxElement = content.parse()?;
                children.push(RsxNode::Element(element));
            } else {
                let expr: Expr = content.parse()?;
                children.push(RsxNode::Expr(expr));
            }
        } else {
            return Err(content.error("unexpected token in RSX"));
        }
    }
    Ok(children)
}

/// Converts a list of `RsxNode` children into a `Vec<VirtualNode>` token stream.
pub(crate) fn children_to_tokens(children: &[RsxNode]) -> TokenStream2 {
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
