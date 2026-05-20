use crate::*;

/// The set of known CSS pseudo-class and pseudo-element keywords
/// that the `class!` macro can parse as ident+brace blocks.
const PSEUDO_KEYWORDS: &[(&str, &str)] = &[
    ("hover", ":hover"),
    ("focus", ":focus"),
    ("focus_within", ":focus-within"),
    ("focus_visible", ":focus-visible"),
    ("active", ":active"),
    ("visited", ":visited"),
    ("disabled", ":disabled"),
    ("enabled", ":enabled"),
    ("checked", ":checked"),
    ("readonly", ":read-only"),
    ("readwrite", ":read-write"),
    ("required", ":required"),
    ("optional", ":optional"),
    ("valid", ":valid"),
    ("invalid", ":invalid"),
    ("in_range", ":in-range"),
    ("out_of_range", ":out-of-range"),
    ("placeholder_shown", ":placeholder-shown"),
    ("first_child", ":first-child"),
    ("last_child", ":last-child"),
    ("only_child", ":only-child"),
    ("first_of_type", ":first-of-type"),
    ("last_of_type", ":last-of-type"),
    ("only_of_type", ":only-of-type"),
    ("root", ":root"),
    ("empty", ":empty"),
    ("target", ":target"),
    ("link", ":link"),
    ("any_link", ":any-link"),
    ("before", "::before"),
    ("after", "::after"),
    ("first_line", "::first-line"),
    ("first_letter", "::first-letter"),
    ("selection", "::selection"),
    ("placeholder", "::placeholder"),
    ("backdrop", "::backdrop"),
    ("marker", "::marker"),
    ("spelling_error", "::spelling-error"),
    ("grammar_error", "::grammar-error"),
];

/// Looks up a keyword and returns the corresponding CSS pseudo selector.
fn lookup_pseudo_selector(keyword: &str) -> Option<&'static str> {
    PSEUDO_KEYWORDS
        .iter()
        .find(|(kw, _)| *kw == keyword)
        .map(|(_, selector)| *selector)
}

/// Parses the `class!` macro input.
impl Parse for ClassInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut classes: Vec<ClassDef> = Vec::new();
        while !input.is_empty() {
            let visibility: syn::Visibility = input.parse()?;
            let name: Ident = input.parse()?;
            let params: Option<Vec<ClassParam>> = if input.peek(syn::token::Paren) {
                let param_content;
                syn::parenthesized!(param_content in input);
                let mut param_list: Vec<ClassParam> = Vec::new();
                while !param_content.is_empty() {
                    let param_name: Ident = param_content.parse()?;
                    param_content.parse::<Token![:]>()?;
                    let ty: syn::Type = param_content.parse()?;
                    param_list.push(ClassParam {
                        name: param_name,
                        ty,
                    });
                    if param_content.peek(Token![,]) {
                        param_content.parse::<Token![,]>()?;
                    }
                }
                Some(param_list)
            } else {
                None
            };
            let content;
            braced!(content in input);
            let mut properties: Vec<(String, ClassPropValue)> = Vec::new();
            let mut pseudo_blocks: Vec<PseudoBlock> = Vec::new();
            let mut media_blocks: Vec<MediaBlock> = Vec::new();
            while !content.is_empty() {
                if content.peek(syn::Ident) {
                    let forked = content.fork();
                    let keyword: Ident = forked.parse()?;
                    let keyword_str: String = keyword.to_string();
                    if lookup_pseudo_selector(&keyword_str).is_some()
                        && forked.peek(syn::token::Brace)
                    {
                        content.parse::<Ident>()?;
                        let selector: &'static str = lookup_pseudo_selector(&keyword_str)
                            .expect("pseudo selector lookup should succeed after check");
                        let block_content;
                        braced!(block_content in content);
                        let mut block_properties: Vec<(String, ClassPropValue)> = Vec::new();
                        while !block_content.is_empty() {
                            let css_key: String = parse_kebab_name(&block_content)?;
                            block_content.parse::<Token![:]>()?;
                            let expr: Expr = block_content.parse()?;
                            let expanded: proc_macro2::TokenStream = expand_var_macros(&expr);
                            let prop_value: ClassPropValue = ClassPropValue::Expr(expanded);
                            block_properties.push((css_key, prop_value));
                            if block_content.peek(Semi) {
                                block_content.parse::<Semi>()?;
                            }
                        }
                        pseudo_blocks.push(PseudoBlock {
                            selector: selector.to_string(),
                            properties: block_properties,
                        });
                        continue;
                    } else if (keyword_str == "nth_child" || keyword_str == "nth_last_child")
                        && forked.peek(syn::token::Paren)
                    {
                        let forked2 = forked;
                        let _paren_content;
                        syn::parenthesized!(_paren_content in forked2);
                        if forked2.peek(syn::token::Brace) {
                            content.parse::<Ident>()?;
                            let paren_content;
                            syn::parenthesized!(paren_content in content);
                            let arg_tokens: proc_macro2::TokenStream = paren_content.parse()?;
                            let arg_str: String = arg_tokens.to_string().replace(' ', "");
                            let selector: String =
                                format!(":{}({})", keyword_str.replace('_', "-"), arg_str);
                            let block_content;
                            braced!(block_content in content);
                            let mut block_properties: Vec<(String, ClassPropValue)> = Vec::new();
                            while !block_content.is_empty() {
                                let css_key: String = parse_kebab_name(&block_content)?;
                                block_content.parse::<Token![:]>()?;
                                let expr: Expr = block_content.parse()?;
                                let expanded: proc_macro2::TokenStream = expand_var_macros(&expr);
                                let prop_value: ClassPropValue = ClassPropValue::Expr(expanded);
                                block_properties.push((css_key, prop_value));
                                if block_content.peek(Semi) {
                                    block_content.parse::<Semi>()?;
                                }
                            }
                            pseudo_blocks.push(PseudoBlock {
                                selector,
                                properties: block_properties,
                            });
                            continue;
                        }
                    } else if keyword_str == "media" && forked.peek2(syn::token::Paren) {
                        let forked2 = forked;
                        forked2.parse::<Ident>()?;
                        let paren_content2;
                        syn::parenthesized!(paren_content2 in forked2);
                        if forked2.peek(syn::token::Brace) {
                            content.parse::<Ident>()?;
                            let query_content;
                            syn::parenthesized!(query_content in content);
                            let query_expr: Expr = query_content.parse()?;
                            let query_str: String = match &query_expr {
                                Expr::Lit(syn::ExprLit {
                                    lit: syn::Lit::Str(lit_str),
                                    ..
                                }) => lit_str.value(),
                                _ => query_expr.to_token_stream().to_string().replace(' ', ""),
                            };
                            let block_content;
                            braced!(block_content in content);
                            let mut block_properties: Vec<(String, ClassPropValue)> = Vec::new();
                            while !block_content.is_empty() {
                                let css_key: String = parse_kebab_name(&block_content)?;
                                block_content.parse::<Token![:]>()?;
                                let expr: Expr = block_content.parse()?;
                                let expanded: proc_macro2::TokenStream = expand_var_macros(&expr);
                                let prop_value: ClassPropValue = ClassPropValue::Expr(expanded);
                                block_properties.push((css_key, prop_value));
                                if block_content.peek(Semi) {
                                    block_content.parse::<Semi>()?;
                                }
                            }
                            media_blocks.push(MediaBlock {
                                query: query_str,
                                properties: block_properties,
                            });
                            continue;
                        }
                    }
                }
                let css_key: String = parse_kebab_name(&content)?;
                content.parse::<Token![:]>()?;
                let expr: Expr = content.parse()?;
                let expanded: proc_macro2::TokenStream = expand_var_macros(&expr);
                let prop_value: ClassPropValue = ClassPropValue::Expr(expanded);
                properties.push((css_key, prop_value));
                if content.peek(Semi) {
                    content.parse::<Semi>()?;
                }
            }
            classes.push(ClassDef {
                visibility,
                name,
                params,
                properties,
                pseudo_blocks,
                media_blocks,
            });
        }
        Ok(ClassInput { classes })
    }
}

/// Generates a `Vec<euv_core::PseudoRule>` expression from a list of pseudo blocks.
fn pseudo_blocks_to_tokens(pseudo_blocks: &[PseudoBlock]) -> Option<proc_macro2::TokenStream> {
    if pseudo_blocks.is_empty() {
        return None;
    }
    let parts: Vec<proc_macro2::TokenStream> = pseudo_blocks
        .iter()
        .map(|block| {
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
                euv_core::PseudoRule::new(
                    #selector.to_string(),
                    [#(#style_parts),*].concat()
                )
            }
        })
        .collect();
    Some(quote! { vec![#(#parts),*] })
}

/// Generates a `Vec<euv_core::MediaRule>` expression from a list of media blocks.
fn media_blocks_to_tokens(media_blocks: &[MediaBlock]) -> Option<proc_macro2::TokenStream> {
    if media_blocks.is_empty() {
        return None;
    }
    let parts: Vec<proc_macro2::TokenStream> = media_blocks
        .iter()
        .map(|block| {
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
                euv_core::MediaRule::new(
                    #query.to_string(),
                    [#(#style_parts),*].concat()
                )
            }
        })
        .collect();
    Some(quote! { vec![#(#parts),*] })
}

/// Generates static pseudo block string for compile-time evaluation.
fn pseudo_blocks_to_static_string(pseudo_blocks: &[PseudoBlock]) -> String {
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
fn media_blocks_to_static_string(media_blocks: &[MediaBlock]) -> String {
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

/// Converts a `ClassDef` into token stream generating a `euv_core::CssClass` function.
impl ToTokens for ClassDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let vis: &syn::Visibility = self.get_visibility();
        let name: &Ident = self.get_name();
        let class_name_str: String = name.to_string();
        let has_extra: bool =
            !self.get_pseudo_blocks().is_empty() || !self.get_media_blocks().is_empty();
        match self.try_get_params() {
            Some(params) => {
                let param_defs: Vec<proc_macro2::TokenStream> = params
                    .iter()
                    .map(|param| {
                        let param_name: &Ident = param.get_name();
                        let ty: &syn::Type = param.get_ty();
                        quote! { #param_name: #ty }
                    })
                    .collect();
                let param_names: Vec<&Ident> =
                    params.iter().map(|p: &ClassParam| p.get_name()).collect();
                let css_string_parts: Vec<proc_macro2::TokenStream> = self
                    .get_properties()
                    .iter()
                    .map(|(key, value)| match value {
                        ClassPropValue::Expr(expr) => {
                            quote! { #key.to_string() + ": " + &(#expr) + "; " }
                        }
                    })
                    .collect();
                if has_extra {
                    let pseudo_expr: proc_macro2::TokenStream =
                        pseudo_blocks_to_tokens(self.get_pseudo_blocks())
                            .unwrap_or_else(|| quote! { vec![] });
                    let media_expr: proc_macro2::TokenStream =
                        media_blocks_to_tokens(self.get_media_blocks())
                            .unwrap_or_else(|| quote! { vec![] });
                    tokens.extend(quote! {
                        #vis fn #name(#(#param_defs),*) -> euv_core::CssClass {
                            let __css_string: String = [#(#css_string_parts),*].concat();
                            let __unique_name: String = format!("{}-{}", #class_name_str, [#(format!("{:?}", #param_names)),*].join("-"));
                            euv_core::CssClass::new_with_rules(__unique_name, __css_string, #pseudo_expr, #media_expr)
                        }
                    });
                } else {
                    tokens.extend(quote! {
                        #vis fn #name(#(#param_defs),*) -> euv_core::CssClass {
                            let __css_string: String = [#(#css_string_parts),*].concat();
                            let __unique_name: String = format!("{}-{}", #class_name_str, [#(format!("{:?}", #param_names)),*].join("-"));
                            euv_core::CssClass::new(__unique_name, __css_string)
                        }
                    });
                }
            }
            None => {
                let name_span: Span = name.span();
                let const_name: Ident = Ident::new(&class_name_str.to_uppercase(), name.span());
                let const_name_token: proc_macro2::TokenStream =
                    quote_spanned!(name_span=> #const_name);
                let fn_name_token: proc_macro2::TokenStream = quote_spanned!(name_span=> #name);
                let all_static: bool = self.get_properties().iter().all(|(_, value)| {
                    let ClassPropValue::Expr(expr) = value;
                    is_static_string_expr(expr)
                }) && self.get_pseudo_blocks().iter().all(|block| {
                    block.get_properties().iter().all(|(_, value)| {
                        let ClassPropValue::Expr(expr) = value;
                        is_static_string_expr(expr)
                    })
                }) && self.get_media_blocks().iter().all(|block| {
                    block.get_properties().iter().all(|(_, value)| {
                        let ClassPropValue::Expr(expr) = value;
                        is_static_string_expr(expr)
                    })
                });
                if all_static {
                    let mut css_string: String = String::new();
                    for (key, value) in self.get_properties() {
                        let ClassPropValue::Expr(expr) = value;
                        css_string.push_str(key);
                        css_string.push_str(": ");
                        css_string.push_str(&expr_to_string(expr));
                        css_string.push_str("; ");
                    }
                    if has_extra {
                        let pseudo_static: String =
                            pseudo_blocks_to_static_string(self.get_pseudo_blocks());
                        let media_static: String =
                            media_blocks_to_static_string(self.get_media_blocks());
                        tokens.extend(quote! {
                            #vis fn #fn_name_token() -> &'static euv_core::CssClass {
                                static #const_name_token: std::sync::OnceLock<euv_core::CssClass> = std::sync::OnceLock::new();
                                #const_name_token.get_or_init(|| {
                                    euv_core::CssClass::new_with_rules(
                                        #class_name_str.to_string(),
                                        #css_string.to_string(),
                                        euv_core::CssClass::parse_pseudo_rules(#pseudo_static),
                                        euv_core::CssClass::parse_media_rules(#media_static),
                                    )
                                })
                            }
                        });
                    } else {
                        tokens.extend(quote! {
                            #vis fn #fn_name_token() -> &'static euv_core::CssClass {
                                static #const_name_token: std::sync::OnceLock<euv_core::CssClass> = std::sync::OnceLock::new();
                                #const_name_token.get_or_init(|| {
                                    euv_core::CssClass::new(#class_name_str.to_string(), #css_string.to_string())
                                })
                            }
                        });
                    }
                } else {
                    let css_string_parts: Vec<proc_macro2::TokenStream> = self
                        .get_properties()
                        .iter()
                        .map(|(key, value)| match value {
                            ClassPropValue::Expr(expr) => {
                                quote! { #key.to_string() + ": " + &(#expr).to_string() + "; " }
                            }
                        })
                        .collect();
                    if has_extra {
                        let pseudo_expr: proc_macro2::TokenStream =
                            pseudo_blocks_to_tokens(self.get_pseudo_blocks())
                                .unwrap_or_else(|| quote! { vec![] });
                        let media_expr: proc_macro2::TokenStream =
                            media_blocks_to_tokens(self.get_media_blocks())
                                .unwrap_or_else(|| quote! { vec![] });
                        tokens.extend(quote! {
                            #vis fn #fn_name_token() -> &'static euv_core::CssClass {
                                static #const_name_token: std::sync::OnceLock<euv_core::CssClass> = std::sync::OnceLock::new();
                                #const_name_token.get_or_init(|| {
                                    let __css_string: String = [#(#css_string_parts),*].concat();
                                    euv_core::CssClass::new_with_rules(#class_name_str.to_string(), __css_string, #pseudo_expr, #media_expr)
                                })
                            }
                        });
                    } else {
                        tokens.extend(quote! {
                            #vis fn #fn_name_token() -> &'static euv_core::CssClass {
                                static #const_name_token: std::sync::OnceLock<euv_core::CssClass> = std::sync::OnceLock::new();
                                #const_name_token.get_or_init(|| {
                                    let __css_string: String = [#(#css_string_parts),*].concat();
                                    euv_core::CssClass::new(#class_name_str.to_string(), __css_string)
                                })
                            }
                        });
                    }
                }
            }
        }
    }
}

/// Converts a `ClassInput` into a token stream of `CssClass` function definitions.
impl ToTokens for ClassInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for class_def in self.get_classes() {
            class_def.to_tokens(tokens);
        }
    }
}
