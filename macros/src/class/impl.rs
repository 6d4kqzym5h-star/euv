use crate::*;

/// Parameters for `emit_once_lock_fn`.
struct OnceLockParams<'a> {
    vis: &'a Visibility,
    fn_name_token: &'a proc_macro2::TokenStream,
    const_name_token: &'a proc_macro2::TokenStream,
    class_name_str: &'a str,
    style_expr: &'a proc_macro2::TokenStream,
    pseudo_expr: &'a proc_macro2::TokenStream,
    media_expr: &'a proc_macro2::TokenStream,
}

/// Generates the `OnceLock`-based static function body for a no-param class.
///
/// Shared by both the all-static and dynamic paths in `ClassDef::to_tokens`.
fn emit_once_lock_fn(tokens: &mut proc_macro2::TokenStream, p: OnceLockParams<'_>) {
    let OnceLockParams {
        vis,
        fn_name_token,
        const_name_token,
        class_name_str,
        style_expr,
        pseudo_expr,
        media_expr,
    } = p;
    tokens.extend(quote! {
        #vis fn #fn_name_token() -> &'static ::euv::Css {
            static #const_name_token: ::std::sync::OnceLock<::euv::Css> = ::std::sync::OnceLock::new();
            #const_name_token.get_or_init(|| {
                let css = ::euv::Css::new(#class_name_str.to_string(), #style_expr, #pseudo_expr, #media_expr);
                css.inject_style();
                css
            })
        }
    });
}

/// Implementation of `Parse` for `ClassInput`, parsing the `class!` macro input.
impl Parse for ClassInput {
    /// Parses the `class!` macro input into a `ClassInput` AST.
    ///
    /// # Arguments
    ///
    /// - `ParseStream`- The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>`- The parsed `ClassInput`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut classes: Vec<ClassDef> = Vec::new();
        while !input.is_empty() {
            let visibility: Visibility = input.parse()?;
            let name: Ident = input.parse()?;
            let params: Option<Vec<ClassParam>> = if input.peek(Paren) {
                let param_content;
                parenthesized!(param_content in input);
                let mut param_list: Vec<ClassParam> = Vec::new();
                while !param_content.is_empty() {
                    let param_name: Ident = param_content.parse()?;
                    param_content.parse::<Token![:]>()?;
                    let ty: Type = param_content.parse()?;
                    param_list.push(ClassParam {
                        name: param_name,
                        ty,
                    });
                    if param_content.peek(Token![,]) {
                        param_content.parse::<Token![,]>()?;
                    }
                }
                if param_list.is_empty() {
                    None
                } else {
                    Some(param_list)
                }
            } else {
                None
            };
            let content;
            braced!(content in input);
            let mut extends: Vec<ClassExtend> = Vec::new();
            let mut properties: Vec<(String, ClassPropValue)> = Vec::new();
            let mut pseudo_blocks: Vec<PseudoBlock> = Vec::new();
            let mut media_blocks: Vec<MediaBlock> = Vec::new();
            while !content.is_empty() {
                if content.peek(Ident) {
                    let forked: ParseBuffer<'_> = content.fork();
                    let keyword: Ident = forked.parse()?;
                    let keyword_str: String = keyword.to_string();
                    let is_extends: bool =
                        forked.peek(Paren) && !keyword_str.starts_with(KEYWORD_MEDIA) && {
                            let forked_extends_buffer: ParseBuffer<'_> = content.fork();
                            let _ = forked_extends_buffer.parse::<Ident>();
                            if forked_extends_buffer.peek(Paren) {
                                let _paren_content;
                                parenthesized!(_paren_content in forked_extends_buffer);
                                forked_extends_buffer.peek(Semi) || forked_extends_buffer.is_empty()
                            } else {
                                false
                            }
                        };
                    if is_extends {
                        content.parse::<Ident>()?;
                        let paren_content;
                        parenthesized!(paren_content in content);
                        let mut args: Vec<proc_macro2::TokenStream> = Vec::new();
                        while !paren_content.is_empty() {
                            let arg_tokens: proc_macro2::TokenStream = paren_content.parse()?;
                            args.push(arg_tokens);
                            if paren_content.peek(Token![,]) {
                                paren_content.parse::<Token![,]>()?;
                            } else {
                                break;
                            }
                        }
                        extends.push(ClassExtend {
                            name: keyword,
                            args,
                        });
                        if content.peek(Semi) {
                            content.parse::<Semi>()?;
                        }
                        continue;
                    }
                    if let Some(selector) = lookup_pseudo_selector(&keyword_str)
                        && forked.peek(Brace)
                    {
                        content.parse::<Ident>()?;
                        let selector: &'static str = selector;
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
                    } else if (keyword_str == KEYWORD_NTH_CHILD
                        || keyword_str == KEYWORD_NTH_LAST_CHILD)
                        && forked.peek(Paren)
                    {
                        let forked_nth_check: ParseBuffer<'_> = forked;
                        let _paren_content;
                        parenthesized!(_paren_content in forked_nth_check);
                        if forked_nth_check.peek(Brace) {
                            content.parse::<Ident>()?;
                            let paren_content;
                            parenthesized!(paren_content in content);
                            let arg_tokens: proc_macro2::TokenStream = paren_content.parse()?;
                            let arg_str: String = arg_tokens.to_string().replace(CHAR_SPACE, "");
                            let selector: String = format!(
                                ":{keyword_kebab}({arg_str})",
                                keyword_kebab = keyword_str.replace(CHAR_UNDERSCORE, STR_HYPHEN)
                            );
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
                    } else if keyword_str == KEYWORD_MEDIA && forked.peek(Paren) {
                        let forked_media_check: ParseBuffer<'_> = forked;
                        let paren_content2;
                        parenthesized!(paren_content2 in forked_media_check);
                        if forked_media_check.peek(Brace) {
                            content.parse::<Ident>()?;
                            let query_content;
                            parenthesized!(query_content in content);
                            let query_expr: Expr = query_content.parse()?;
                            let query_str: String = match &query_expr {
                                Expr::Lit(ExprLit {
                                    lit: Lit::Str(literal_string),
                                    ..
                                }) => literal_string.value(),
                                _ => query_expr
                                    .to_token_stream()
                                    .to_string()
                                    .replace(CHAR_SPACE, ""),
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
                extends,
                properties,
                pseudo_blocks,
                media_blocks,
            });
        }
        Ok(Self { classes })
    }
}

/// Implementation of `ToTokens` for `ClassDef`, converting a class definition into `Css` function tokens.
impl ToTokens for ClassDef {
    /// Converts this class definition into token stream constructing a `Css`.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream`- The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let vis: &Visibility = self.get_visibility();
        let name: &Ident = self.get_name();
        let class_name_str: String = name.to_string();
        let has_extra: bool =
            !self.get_pseudo_blocks().is_empty() || !self.get_media_blocks().is_empty();
        let has_extends: bool = !self.get_extends().is_empty();
        match self.try_get_params() {
            Some(params) => {
                let param_defs: Vec<proc_macro2::TokenStream> = params
                    .iter()
                    .map(|param: &ClassParam| {
                        let param_name: &Ident = param.get_name();
                        let ty: &Type = param.get_ty();
                        quote! { #param_name: #ty }
                    })
                    .collect();
                let param_names: Vec<&Ident> = params
                    .iter()
                    .map(|param: &ClassParam| param.get_name())
                    .collect();
                let mut all_css_parts: Vec<proc_macro2::TokenStream> = self
                    .get_extends()
                    .iter()
                    .map(|parent: &ClassExtend| {
                        let parent_name: &Ident = parent.get_name();
                        let parent_args: &Vec<proc_macro2::TokenStream> = parent.get_args();
                        if parent_args.is_empty() {
                            quote! { #parent_name().get_style().to_string() + #STR_SPACE }
                        } else {
                            quote! { #parent_name(#(#parent_args), *).get_style().to_string() + #STR_SPACE }
                        }
                    })
                    .collect();
                for (key, value) in self.get_properties() {
                    let ClassPropValue::Expr(expr) = value;
                    all_css_parts.push(quote! { #key.to_string() + #CSS_PROP_SEPARATOR + &(#expr) + #CSS_DECL_TERMINATOR });
                }
                let unique_name_expr: proc_macro2::TokenStream = if param_names.is_empty() {
                    quote! { #class_name_str.to_string() }
                } else {
                    let str_hyphen: &str = STR_HYPHEN;
                    let name_format: String = format!("{{}}{str_hyphen}{{}}");
                    quote! { format!(#name_format, #class_name_str, [#(format!("{:?}", #param_names)), *].join(#str_hyphen)) }
                };
                if has_extra {
                    let pseudo_expr: proc_macro2::TokenStream =
                        pseudo_blocks_to_tokens(self.get_pseudo_blocks())
                            .unwrap_or_else(|| quote! { vec![] });
                    let media_expr: proc_macro2::TokenStream =
                        media_blocks_to_tokens(self.get_media_blocks())
                            .unwrap_or_else(|| quote! { vec![] });
                    tokens.extend(quote! {
                        #vis fn #name(#(#param_defs), *) -> ::euv::Css {
                            ::euv::Css::new(#unique_name_expr, [#(#all_css_parts), *].concat(), #pseudo_expr, #media_expr)
                        }
                    });
                } else {
                    tokens.extend(quote! {
                        #vis fn #name(#(#param_defs), *) -> ::euv::Css {
                            ::euv::Css::new(#unique_name_expr, [#(#all_css_parts), *].concat(), vec![], vec![])
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
                let all_static: bool = !has_extends
                    && self.get_properties().iter().all(|(_, value)| {
                        let ClassPropValue::Expr(expr) = value;
                        is_static_string_expr(expr)
                    })
                    && self.get_pseudo_blocks().iter().all(|block: &PseudoBlock| {
                        block.get_properties().iter().all(|(_, value)| {
                            let ClassPropValue::Expr(expr) = value;
                            is_static_string_expr(expr)
                        })
                    })
                    && self.get_media_blocks().iter().all(|block: &MediaBlock| {
                        block.get_properties().iter().all(|(_, value)| {
                            let ClassPropValue::Expr(expr) = value;
                            is_static_string_expr(expr)
                        })
                    });
                let (pseudo_expr, media_expr) = if has_extra {
                    let p = pseudo_blocks_to_tokens(self.get_pseudo_blocks())
                        .unwrap_or_else(|| quote! { vec![] });
                    let m = media_blocks_to_tokens(self.get_media_blocks())
                        .unwrap_or_else(|| quote! { vec![] });
                    (p, m)
                } else {
                    (quote! { vec![] }, quote! { vec![] })
                };
                if all_static {
                    let mut css_string: String = String::new();
                    for (key, value) in self.get_properties() {
                        let ClassPropValue::Expr(expr) = value;
                        css_string.push_str(key);
                        css_string.push_str(CSS_PROP_SEPARATOR);
                        css_string.push_str(&expr_to_string(expr));
                        css_string.push_str(CSS_DECL_TERMINATOR);
                    }
                    if has_extra {
                        let pseudo_static: String =
                            pseudo_blocks_to_static_string(self.get_pseudo_blocks());
                        let media_static: String = media_blocks_to_static_string(self.get_media_blocks());
                        emit_once_lock_fn(
                            tokens,
                            OnceLockParams {
                                vis,
                                fn_name_token: &fn_name_token,
                                const_name_token: &const_name_token,
                                class_name_str: &class_name_str,
                                style_expr: &quote! { #css_string.to_string() },
                                pseudo_expr: &quote! { ::euv::Css::parse_pseudo_rules(#pseudo_static) },
                                media_expr: &quote! { ::euv::Css::parse_media_rules(#media_static) },
                            },
                        );
                    } else {
                        emit_once_lock_fn(
                            tokens,
                            OnceLockParams {
                                vis,
                                fn_name_token: &fn_name_token,
                                const_name_token: &const_name_token,
                                class_name_str: &class_name_str,
                                style_expr: &quote! { #css_string.to_string() },
                                pseudo_expr: &pseudo_expr,
                                media_expr: &media_expr,
                            },
                        );
                    }
                } else {
                    let mut all_css_parts: Vec<proc_macro2::TokenStream> = self
                        .get_extends()
                        .iter()
                        .map(|parent: &ClassExtend| {
                            let parent_name: &Ident = parent.get_name();
                            let parent_args: &Vec<proc_macro2::TokenStream> = parent.get_args();
                            if parent_args.is_empty() {
                                quote! { #parent_name().get_style().to_string() + #STR_SPACE }
                            } else {
                                quote! { #parent_name(#(#parent_args), *).get_style().to_string() + #STR_SPACE }
                            }
                        })
                        .collect();
                    for (key, value) in self.get_properties() {
                        let ClassPropValue::Expr(expr) = value;
                        all_css_parts
                            .push(quote! { #key.to_string() + #CSS_PROP_SEPARATOR + &(#expr).to_string() + #CSS_DECL_TERMINATOR });
                    }
                    emit_once_lock_fn(
                        tokens,
                        OnceLockParams {
                            vis,
                            fn_name_token: &fn_name_token,
                            const_name_token: &const_name_token,
                            class_name_str: &class_name_str,
                            style_expr: &quote! { [#(#all_css_parts), *].concat() },
                            pseudo_expr: &pseudo_expr,
                            media_expr: &media_expr,
                        },
                    );
                }
            }
        }
    }
}

/// Implementation of `ToTokens` for `ClassInput`, converting class definitions into token streams.
impl ToTokens for ClassInput {
    /// Converts all class definitions into token streams.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream`- The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for class_def in self.get_classes() {
            class_def.to_tokens(tokens);
        }
    }
}
