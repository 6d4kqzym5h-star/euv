use crate::*;

/// Parses the `class!` macro input and generates `Css` function definitions.
///
/// # Arguments
///
/// - `TokenStream` - The raw token stream representing class definitions.
///
/// # Returns
///
/// - `TokenStream` - The generated token stream constructing `Css` functions.
pub fn parse_class(input: TokenStream) -> TokenStream {
    let tokens: proc_macro2::TokenStream = match parse::<ClassInput>(input) {
        Ok(class_input) => class_input.into_token_stream(),
        Err(error) => return error.to_compile_error().into(),
    };
    TokenStream::from(tokens)
}

/// Parses a CSS property key from the token stream.
///
/// Supports two forms:
/// - Static: A kebab-case identifier or string literal (e.g., `font_size`, `"background"`).
/// - Dynamic: A braced expression (e.g., `{key_var}`) that evaluates to a string at runtime.
///
/// # Arguments
///
/// - `ParseStream` - The syn parse stream to read from.
///
/// # Returns
///
/// - `syn::Result<ClassPropKey>` - The parsed property key.
pub(crate) fn parse_class_prop_key(input: ParseStream) -> syn::Result<ClassPropKey> {
    if input.peek(Brace) {
        let content: ParseBuffer<'_>;
        braced!(content in input);
        let expr: Expr = content.parse()?;
        Ok(ClassPropKey::Dynamic(expr.to_token_stream()))
    } else {
        let key: String = parse_kebab_name(input)?;
        Ok(ClassPropKey::Static(key))
    }
}

/// Converts a `ClassPropKey` into a token stream that evaluates to a `String`.
///
/// # Arguments
///
/// - `&ClassPropKey` - The property key to convert.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Token stream that evaluates to a `String`.
pub(crate) fn class_prop_key_to_tokens(key: &ClassPropKey) -> proc_macro2::TokenStream {
    match key {
        ClassPropKey::Static(static_key) => {
            quote! { #static_key.to_string() }
        }
        ClassPropKey::Dynamic(expr) => {
            quote! { (#expr).to_string() }
        }
    }
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
            if expr_macro.mac.path.is_ident(VAR) {
                let body_tokens: &proc_macro2::TokenStream = &expr_macro.mac.tokens;
                let body_str: String = reconstruct_kebab_from_tokens(body_tokens);
                let css_name: String = format!("{CSS_VAR_PREFIX}{body_str}{CSS_VAR_SUFFIX}");
                quote! { #css_name }
            } else if expr_macro.mac.path.is_ident(FORMAT_MACRO) {
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
                if *ident == VAR
                    && iter.peek().is_some_and(
                        |token: &proc_macro2::TokenTree| matches!(token, proc_macro2::TokenTree::Punct(punct) if punct.as_char() == '!'),
                    ) =>
            {
                iter.next();
                if iter
                    .peek()
                    .is_some_and(|token: &proc_macro2::TokenTree| matches!(token, proc_macro2::TokenTree::Group(_)))
                {
                    if let Some(proc_macro2::TokenTree::Group(group)) = iter.next() {
                        let inner: proc_macro2::TokenStream = group.stream();
                        let var_name: String = reconstruct_kebab_from_tokens(&inner);
                        let css_name: String = format!("{CSS_VAR_PREFIX}{var_name}{CSS_VAR_SUFFIX}");
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
        if let proc_macro2::TokenTree::Literal(literal) = token {
            let literal_token_stream: proc_macro2::TokenStream =
                proc_macro2::TokenTree::Literal(literal).into();
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
                .map(|(key, value): &(ClassPropKey, ClassPropValue)| match value {
                    ClassPropValue::Expr(expr) => {
                        let key_token: proc_macro2::TokenStream = class_prop_key_to_tokens(key);
                        quote! { #key_token + #CSS_PROP_SEPARATOR + &(#expr).to_string() + #CSS_DECL_TERMINATOR }
                    }
                })
                .collect();
            quote! {
                ::euv::PseudoRule::new(
                    #selector.to_string(),
                    [#(#style_parts), *].concat()
                )
            }
        })
        .collect();
    Some(quote! { vec![#(#parts), *] })
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
                .map(|(key, value): &(ClassPropKey, ClassPropValue)| match value {
                    ClassPropValue::Expr(expr) => {
                        let key_token: proc_macro2::TokenStream = class_prop_key_to_tokens(key);
                        quote! { #key_token + #CSS_PROP_SEPARATOR + &(#expr).to_string() + #CSS_DECL_TERMINATOR }
                    }
                })
                .collect();
            let pseudo_expr: proc_macro2::TokenStream =
                pseudo_blocks_to_tokens(block.get_pseudo_blocks())
                    .unwrap_or_else(|| quote! { vec![] });
            quote! {
                ::euv::MediaRule::new(
                    #query.to_string(),
                    [#(#style_parts), *].concat(),
                    #pseudo_expr
                )
            }
        })
        .collect();
    Some(quote! { vec![#(#parts), *] })
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
        result.push_str(CSS_RULE_OPEN);
        for (key, value) in block.get_properties() {
            let ClassPropValue::Expr(expr) = value;
            let ClassPropKey::Static(key_str) = key else {
                continue;
            };
            result.push_str(key_str);
            result.push_str(CSS_PROP_SEPARATOR);
            result.push_str(&expr_to_string(expr));
            result.push_str(CSS_DECL_TERMINATOR);
        }
        result.push(CHAR_CSS_RULE_CLOSE);
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
        result.push_str(CSS_MEDIA_PREFIX);
        result.push_str(block.get_query());
        result.push_str(CSS_RULE_OPEN);
        for (key, value) in block.get_properties() {
            let ClassPropValue::Expr(expr) = value;
            let ClassPropKey::Static(key_str) = key else {
                continue;
            };
            result.push_str(key_str);
            result.push_str(CSS_PROP_SEPARATOR);
            result.push_str(&expr_to_string(expr));
            result.push_str(CSS_DECL_TERMINATOR);
        }
        for pseudo_block in block.get_pseudo_blocks() {
            result.push_str(pseudo_block.get_selector());
            result.push_str(CSS_RULE_OPEN);
            for (key, value) in pseudo_block.get_properties() {
                let ClassPropValue::Expr(expr) = value;
                let ClassPropKey::Static(key_str) = key else {
                    continue;
                };
                result.push_str(key_str);
                result.push_str(CSS_PROP_SEPARATOR);
                result.push_str(&expr_to_string(expr));
                result.push_str(CSS_DECL_TERMINATOR);
            }
            result.push(CHAR_CSS_RULE_CLOSE);
        }
        result.push(CHAR_CSS_RULE_CLOSE);
    }
    result
}

/// Generates the `OnceLock`-based static function body for a no-param class.
///
/// Shared by both the all-static and dynamic paths in `ClassDef::to_tokens`.
///
/// # Arguments
///
/// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
/// - `OnceLockParams` - The parameters for the OnceLock function generation.
pub(crate) fn emit_once_lock_fn(
    tokens: &mut proc_macro2::TokenStream,
    once_lock_params: OnceLockParams<'_>,
) {
    let OnceLockParams {
        visibility,
        fn_name_token,
        const_name_token,
        class_name_str,
        style_expr,
        pseudo_expr,
        media_expr,
    } = once_lock_params;
    tokens.extend(quote! {
        #visibility fn #fn_name_token() -> &'static ::euv::Css {
            static #const_name_token: ::std::sync::OnceLock<::euv::Css> = ::std::sync::OnceLock::new();
            #const_name_token.get_or_init(|| {
                let css: ::euv::Css = ::euv::Css::new(#class_name_str.to_string(), #style_expr, #pseudo_expr, #media_expr);
                css.inject_style();
                css
            })
        }
    });
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
        KEYWORD_HOVER => Some(PSEUDO_HOVER),
        KEYWORD_FOCUS => Some(PSEUDO_FOCUS),
        KEYWORD_FOCUS_WITHIN => Some(PSEUDO_FOCUS_WITHIN),
        KEYWORD_FOCUS_VISIBLE => Some(PSEUDO_FOCUS_VISIBLE),
        KEYWORD_ACTIVE => Some(PSEUDO_ACTIVE),
        KEYWORD_VISITED => Some(PSEUDO_VISITED),
        KEYWORD_DISABLED => Some(PSEUDO_DISABLED),
        KEYWORD_ENABLED => Some(PSEUDO_ENABLED),
        KEYWORD_CHECKED => Some(PSEUDO_CHECKED),
        KEYWORD_READONLY => Some(PSEUDO_READONLY),
        KEYWORD_READWRITE => Some(PSEUDO_READWRITE),
        KEYWORD_REQUIRED => Some(PSEUDO_REQUIRED),
        KEYWORD_OPTIONAL => Some(PSEUDO_OPTIONAL),
        KEYWORD_VALID => Some(PSEUDO_VALID),
        KEYWORD_INVALID => Some(PSEUDO_INVALID),
        KEYWORD_IN_RANGE => Some(PSEUDO_IN_RANGE),
        KEYWORD_OUT_OF_RANGE => Some(PSEUDO_OUT_OF_RANGE),
        KEYWORD_PLACEHOLDER_SHOWN => Some(PSEUDO_PLACEHOLDER_SHOWN),
        KEYWORD_FIRST_CHILD => Some(PSEUDO_FIRST_CHILD),
        KEYWORD_LAST_CHILD => Some(PSEUDO_LAST_CHILD),
        KEYWORD_ONLY_CHILD => Some(PSEUDO_ONLY_CHILD),
        KEYWORD_FIRST_OF_TYPE => Some(PSEUDO_FIRST_OF_TYPE),
        KEYWORD_LAST_OF_TYPE => Some(PSEUDO_LAST_OF_TYPE),
        KEYWORD_ONLY_OF_TYPE => Some(PSEUDO_ONLY_OF_TYPE),
        KEYWORD_ROOT => Some(PSEUDO_ROOT),
        KEYWORD_EMPTY => Some(PSEUDO_EMPTY),
        KEYWORD_TARGET => Some(PSEUDO_TARGET),
        KEYWORD_LINK => Some(PSEUDO_LINK),
        KEYWORD_ANY_LINK => Some(PSEUDO_ANY_LINK),
        KEYWORD_BEFORE => Some(PSEUDO_BEFORE),
        KEYWORD_AFTER => Some(PSEUDO_AFTER),
        KEYWORD_FIRST_LINE => Some(PSEUDO_FIRST_LINE),
        KEYWORD_FIRST_LETTER => Some(PSEUDO_FIRST_LETTER),
        KEYWORD_SELECTION => Some(PSEUDO_SELECTION),
        KEYWORD_PLACEHOLDER => Some(PSEUDO_PLACEHOLDER),
        KEYWORD_BACKDROP => Some(PSEUDO_BACKDROP),
        KEYWORD_MARKER => Some(PSEUDO_MARKER),
        KEYWORD_SPELLING_ERROR => Some(PSEUDO_SPELLING_ERROR),
        KEYWORD_GRAMMAR_ERROR => Some(PSEUDO_GRAMMAR_ERROR),
        _ => None,
    }
}
