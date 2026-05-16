use crate::*;

/// Parses the `class!` macro input and generates `CssClass` function definitions.
///
/// # Arguments
///
/// - `TokenStream`: The raw token stream representing class definitions.
///
/// # Returns
///
/// - `TokenStream`: The generated token stream constructing `CssClass` functions.
pub fn parse_class(input: TokenStream) -> TokenStream {
    let class_input: ClassInput = parse_macro_input!(input as ClassInput);
    let tokens: TokenStream2 = class_input.into_token_stream();
    TokenStream::from(tokens)
}

/// Recursively expands `var!(name)` macro calls within an expression tree
/// into the corresponding CSS `var()` string literal.
///
/// # Arguments
///
/// - `&Expr`: The expression to expand.
///
/// # Returns
///
/// - `TokenStream2`: The expanded token stream with `var!()` calls replaced
///   by `"var(--xxx-yyy)"` string literals.
pub(crate) fn expand_var_macros(expr: &Expr) -> TokenStream2 {
    match expr {
        Expr::Macro(expr_macro) => {
            if expr_macro.mac.path.is_ident("var") {
                let body_tokens: &TokenStream2 = &expr_macro.mac.tokens;
                let body_str: String = reconstruct_kebab_from_tokens(body_tokens);
                let css_name: String = format!("var(--{})", body_str);
                quote! { #css_name }
            } else if expr_macro.mac.path.is_ident("format") {
                let mac_tokens: &TokenStream2 = &expr_macro.mac.tokens;
                let expanded: TokenStream2 = expand_var_macros_in_tokens(mac_tokens);
                let path: &syn::Path = &expr_macro.mac.path;
                quote! { #path!(#expanded) }
            } else {
                expr.into_token_stream()
            }
        }
        _ => expr.into_token_stream(),
    }
}

/// Scans a token stream for `var!(name)` patterns and expands them
/// to the corresponding CSS `var()` string literals.
///
/// This is used to handle `var!()` calls nested inside `format!()` and
/// other macro invocations where syn does not provide structured parsing.
///
/// # Arguments
///
/// - `&TokenStream2`: The token stream to scan.
///
/// # Returns
///
/// - `TokenStream2`: The token stream with `var!(name)` patterns replaced
///   by `"var(--xxx-yyy)"` string literals.
pub(crate) fn expand_var_macros_in_tokens(tokens: &TokenStream2) -> TokenStream2 {
    let mut result: Vec<proc_macro2::TokenTree> = Vec::new();
    let mut iter: std::iter::Peekable<proc_macro2::token_stream::IntoIter> =
        tokens.clone().into_iter().peekable();
    while let Some(token) = iter.next() {
        match &token {
            proc_macro2::TokenTree::Ident(ident)
                if *ident == "var"
                    && iter.peek().is_some_and(
                        |t| matches!(t, proc_macro2::TokenTree::Punct(p) if p.as_char() == '!'),
                    ) =>
            {
                iter.next();
                if iter
                    .peek()
                    .is_some_and(|t| matches!(t, proc_macro2::TokenTree::Group(_)))
                {
                    if let Some(proc_macro2::TokenTree::Group(group)) = iter.next() {
                        let inner: TokenStream2 = group.stream();
                        let var_name: String = reconstruct_kebab_from_tokens(&inner);
                        let css_name: String = format!("var(--{})", var_name);
                        let expanded: TokenStream2 = quote! { #css_name };
                        result.extend(expanded);
                    }
                } else {
                    result.push(proc_macro2::TokenTree::Ident(ident.clone()));
                    result.push(proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                        '!',
                        proc_macro2::Spacing::Alone,
                    )));
                }
            }
            proc_macro2::TokenTree::Group(group) => {
                let expanded_inner: TokenStream2 = expand_var_macros_in_tokens(&group.stream());
                let new_group: proc_macro2::Group =
                    proc_macro2::Group::new(group.delimiter(), expanded_inner);
                result.push(proc_macro2::TokenTree::Group(new_group));
            }
            _ => {
                result.push(token);
            }
        }
    }
    result.into_iter().collect()
}
