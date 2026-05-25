use crate::*;

/// Parses the `class!` macro input and generates `CssClass` function definitions.
///
/// # Arguments
///
/// - `TokenStream` - The raw token stream representing class definitions.
///
/// # Returns
///
/// - `TokenStream` - The generated token stream constructing `CssClass` functions.
pub fn parse_class(input: TokenStream) -> TokenStream {
    let tokens: proc_macro2::TokenStream = match parse::<ClassInput>(input) {
        Ok(class_input) => class_input.into_token_stream(),
        Err(error) => return error.to_compile_error().into(),
    };
    TokenStream::from(tokens)
}

/// Recursively expands `var!(name)` macro calls within an expression tree
/// into the corresponding CSS `var()` string literal.
///
/// # Arguments
///
/// - `&Expr` - The expression to expand.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The expanded token stream with `var!()` calls replaced
///   by `"var(--xxx-yyy)"` string literals.
pub(crate) fn expand_var_macros(expr: &Expr) -> proc_macro2::TokenStream {
    match expr {
        Expr::Macro(expr_macro) => {
            if expr_macro.mac.path.is_ident("var") {
                let body_tokens: &proc_macro2::TokenStream = &expr_macro.mac.tokens;
                let body_str: String = reconstruct_kebab_from_tokens(body_tokens);
                let css_name: String = format!("var(--{})", body_str);
                quote! { #css_name }
            } else if expr_macro.mac.path.is_ident("format") {
                let mac_tokens: &proc_macro2::TokenStream = &expr_macro.mac.tokens;
                let expanded: proc_macro2::TokenStream = expand_var_macros_in_tokens(mac_tokens);
                let path: &Path = &expr_macro.mac.path;
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
/// - `&proc_macro2::TokenStream` - The token stream to scan.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The token stream with `var!(name)` patterns replaced
///   by `"var(--xxx-yyy)"` string literals.
pub(crate) fn expand_var_macros_in_tokens(
    tokens: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let mut result: Vec<proc_macro2::TokenTree> = Vec::new();
    let mut iter: Peekable<proc_macro2::token_stream::IntoIter> =
        tokens.clone().into_iter().peekable();
    while let Some(token) = iter.next() {
        match &token {
            proc_macro2::TokenTree::Ident(ident)
                if *ident == "var"
                    && iter.peek().is_some_and(
                        |t: &proc_macro2::TokenTree| matches!(t, proc_macro2::TokenTree::Punct(p) if p.as_char() == '!'),
                    ) =>
            {
                iter.next();
                if iter
                    .peek()
                    .is_some_and(|t: &proc_macro2::TokenTree| matches!(t, proc_macro2::TokenTree::Group(_)))
                {
                    if let Some(proc_macro2::TokenTree::Group(group)) = iter.next() {
                        let inner: proc_macro2::TokenStream = group.stream();
                        let var_name: String = reconstruct_kebab_from_tokens(&inner);
                        let css_name: String = format!("var(--{})", var_name);
                        let expanded: proc_macro2::TokenStream = quote! { #css_name };
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
                let expanded_inner: proc_macro2::TokenStream =
                    expand_var_macros_in_tokens(&group.stream());
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

/// Checks whether a `proc_macro2::TokenStream` consists entirely of string literals,
/// meaning its value can be computed at compile time.
///
/// # Arguments
///
/// - `&proc_macro2::TokenStream` - The token stream to check.
///
/// # Returns
///
/// - `bool` - `true` if all tokens are string literals.
pub(crate) fn is_static_string_expr(tokens: &proc_macro2::TokenStream) -> bool {
    for token in tokens.clone() {
        match token {
            proc_macro2::TokenTree::Literal(_) => continue,
            _ => return false,
        }
    }
    true
}

/// Extracts the string value from a token stream that consists entirely of
/// string literals, concatenating them.
///
/// # Arguments
///
/// - `&proc_macro2::TokenStream` - The token stream consisting of string literals only.
///
/// # Returns
///
/// - `String` - The concatenated string value.
pub(crate) fn expr_to_string(tokens: &proc_macro2::TokenStream) -> String {
    let mut result: String = String::new();
    for token in tokens.clone() {
        if let proc_macro2::TokenTree::Literal(lit) = token {
            let literal_token_stream: proc_macro2::TokenStream =
                proc_macro2::TokenTree::Literal(lit).into();
            if let Ok(literal_string) = parse2::<LitStr>(literal_token_stream) {
                result.push_str(&literal_string.value());
            }
        }
    }
    result
}

/// Generates a `Vec<::euv::PseudoRule>` expression from a list of pseudo blocks.
///
/// # Arguments
///
/// - `&[PseudoBlock]` - The pseudo blocks to convert.
///
/// # Returns
///
/// - `Option<proc_macro2::TokenStream>` - The generated token stream, or `None` if empty.
pub(crate) fn pseudo_blocks_to_tokens(
    pseudo_blocks: &[PseudoBlock],
) -> Option<proc_macro2::TokenStream> {
    if pseudo_blocks.is_empty() {
        return None;
    }
    let parts: Vec<proc_macro2::TokenStream> = pseudo_blocks
        .iter()
        .map(|block: &PseudoBlock| {
            let selector: &str = block.get_selector();
            let style_parts: Vec<proc_macro2::TokenStream> = block
                .get_properties()
                .iter()
                .map(|(key, value)| match value {
                    ClassPropValue::Expr(expr) => {
                        quote! { #key.to_string() + ": " + &(#expr).to_string() + "; " }
                    }
                })
                .collect();
            quote! {
                ::euv::PseudoRule::new(
                    #selector.to_string(),
                    [#(#style_parts),*].concat()
                )
            }
        })
        .collect();
    Some(quote! { vec![#(#parts),*] })
}

/// Generates a `Vec<::euv::MediaRule>` expression from a list of media blocks.
///
/// # Arguments
///
/// - `&[MediaBlock]` - The media blocks to convert.
///
/// # Returns
///
/// - `Option<proc_macro2::TokenStream>` - The generated token stream, or `None` if empty.
pub(crate) fn media_blocks_to_tokens(
    media_blocks: &[MediaBlock],
) -> Option<proc_macro2::TokenStream> {
    if media_blocks.is_empty() {
        return None;
    }
    let parts: Vec<proc_macro2::TokenStream> = media_blocks
        .iter()
        .map(|block: &MediaBlock| {
            let query: &str = block.get_query();
            let style_parts: Vec<proc_macro2::TokenStream> = block
                .get_properties()
                .iter()
                .map(|(key, value)| match value {
                    ClassPropValue::Expr(expr) => {
                        quote! { #key.to_string() + ": " + &(#expr).to_string() + "; " }
                    }
                })
                .collect();
            quote! {
                ::euv::MediaRule::new(
                    #query.to_string(),
                    [#(#style_parts),*].concat()
                )
            }
        })
        .collect();
    Some(quote! { vec![#(#parts),*] })
}

/// Generates static pseudo block string for compile-time evaluation.
///
/// # Arguments
///
/// - `&[PseudoBlock]` - The pseudo blocks to serialize.
///
/// # Returns
///
/// - `String` - The serialized pseudo rules string.
pub(crate) fn pseudo_blocks_to_static_string(pseudo_blocks: &[PseudoBlock]) -> String {
    let mut result: String = String::new();
    for block in pseudo_blocks {
        result.push_str(block.get_selector());
        result.push_str(" { ");
        for (key, value) in block.get_properties() {
            let ClassPropValue::Expr(expr) = value;
            result.push_str(key);
            result.push_str(": ");
            result.push_str(&expr_to_string(expr));
            result.push_str("; ");
        }
        result.push('}');
    }
    result
}

/// Generates static media block string for compile-time evaluation.
///
/// # Arguments
///
/// - `&[MediaBlock]` - The media blocks to serialize.
///
/// # Returns
///
/// - `String` - The serialized media rules string.
pub(crate) fn media_blocks_to_static_string(media_blocks: &[MediaBlock]) -> String {
    let mut result: String = String::new();
    for block in media_blocks {
        result.push_str("@media ");
        result.push_str(block.get_query());
        result.push_str(" { ");
        for (key, value) in block.get_properties() {
            let ClassPropValue::Expr(expr) = value;
            result.push_str(key);
            result.push_str(": ");
            result.push_str(&expr_to_string(expr));
            result.push_str("; ");
        }
        result.push('}');
    }
    result
}

/// Looks up a CSS pseudo selector by keyword.
///
/// Returns the corresponding CSS pseudo selector string for known keywords,
/// or `None` if the keyword is not recognized.
///
/// # Arguments
///
/// - `&str` - The keyword to look up (e.g., "hover", "focus", "before").
///
/// # Returns
///
/// - `Option<&'static str>` - The CSS pseudo selector string if found, or `None`.
pub(crate) fn lookup_pseudo_selector(keyword: &str) -> Option<&'static str> {
    match keyword {
        "hover" => Some(":hover"),
        "focus" => Some(":focus"),
        "focus_within" => Some(":focus-within"),
        "focus_visible" => Some(":focus-visible"),
        "active" => Some(":active"),
        "visited" => Some(":visited"),
        "disabled" => Some(":disabled"),
        "enabled" => Some(":enabled"),
        "checked" => Some(":checked"),
        "readonly" => Some(":read-only"),
        "readwrite" => Some(":read-write"),
        "required" => Some(":required"),
        "optional" => Some(":optional"),
        "valid" => Some(":valid"),
        "invalid" => Some(":invalid"),
        "in_range" => Some(":in-range"),
        "out_of_range" => Some(":out-of-range"),
        "placeholder_shown" => Some(":placeholder-shown"),
        "first_child" => Some(":first-child"),
        "last_child" => Some(":last-child"),
        "only_child" => Some(":only-child"),
        "first_of_type" => Some(":first-of-type"),
        "last_of_type" => Some(":last-of-type"),
        "only_of_type" => Some(":only-of-type"),
        "root" => Some(":root"),
        "empty" => Some(":empty"),
        "target" => Some(":target"),
        "link" => Some(":link"),
        "any_link" => Some(":any-link"),
        "before" => Some("::before"),
        "after" => Some("::after"),
        "first_line" => Some("::first-line"),
        "first_letter" => Some("::first-letter"),
        "selection" => Some("::selection"),
        "placeholder" => Some("::placeholder"),
        "backdrop" => Some("::backdrop"),
        "marker" => Some("::marker"),
        "spelling_error" => Some("::spelling-error"),
        "grammar_error" => Some("::grammar-error"),
        _ => None,
    }
}
