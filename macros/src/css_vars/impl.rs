use crate::*;

/// Generates the `OnceLock`-based static function body for a no-param CSS variable block.
///
/// # Arguments
///
/// - `tokens` - The target token stream to append to.
/// - `vis` - The visibility modifier.
/// - `fn_name_token` - The function name token stream.
/// - `const_name_token` - The `OnceLock` constant name token stream.
/// - `class_name_str` - The class name string literal.
/// - `style_expr` - Token stream that evaluates to the CSS style string.
fn emit_css_var_once_lock_fn(
    tokens: &mut proc_macro2::TokenStream,
    vis: &Visibility,
    fn_name_token: &proc_macro2::TokenStream,
    const_name_token: &proc_macro2::TokenStream,
    class_name_str: &str,
    style_expr: &proc_macro2::TokenStream,
) {
    tokens.extend(quote! {
        #vis fn #fn_name_token() -> &'static ::euv::Css {
            static #const_name_token: ::std::sync::OnceLock<::euv::Css> = ::std::sync::OnceLock::new();
            #const_name_token.get_or_init(|| {
                let css = ::euv::Css::new(#class_name_str.to_string(), #style_expr, vec![], vec![]);
                css.inject_style();
                css
            })
        }
    });
}

/// Implementation of `Parse` for `CssVarInput`, parsing the `css_vars!` macro input.
impl Parse for CssVarInput {
    /// Parses the `css_vars!` macro input into a `CssVarInput` AST.
    ///
    /// # Arguments
    ///
    /// - `ParseStream`- The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>`- The parsed `CssVarInput`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut defs: Vec<CssVarDef> = Vec::new();
        while !input.is_empty() {
            let visibility: Visibility = input.parse()?;
            let name: Ident = input.parse()?;
            let params: Option<Vec<CssVarParam>> = if input.peek(Paren) {
                let param_content;
                syn::parenthesized!(param_content in input);
                let mut param_list: Vec<CssVarParam> = Vec::new();
                while !param_content.is_empty() {
                    let param_name: Ident = param_content.parse()?;
                    param_content.parse::<Token![:]>()?;
                    let ty: Type = param_content.parse()?;
                    param_list.push(CssVarParam {
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
            let mut vars: Vec<(String, CssVarValue)> = Vec::new();
            while !content.is_empty() {
                let var_name: String = parse_kebab_name(&content)?;
                let css_key: String = format!("{CSS_CUSTOM_PROPERTY_PREFIX}{var_name}");
                content.parse::<Token![:]>()?;
                let var_value: CssVarValue = {
                    let expr: Expr = content.parse()?;
                    CssVarValue::Expr(expr.into_token_stream())
                };
                vars.push((css_key, var_value));
                if content.peek(Semi) {
                    content.parse::<Semi>()?;
                }
            }
            defs.push(CssVarDef {
                visibility,
                name,
                params,
                vars,
            });
        }
        Ok(Self { defs })
    }
}

/// Implementation of `ToTokens` for `CssVarDef`, converting a CSS variable block into `Css` function tokens.
///
/// Each CSS variable block becomes a `Css` function that, when called, injects
/// the CSS custom properties into the DOM and returns a reference to the class.
/// The CSS key names are prefixed with `--`.
impl ToTokens for CssVarDef {
    /// Converts this CSS variable definition into token stream constructing a `Css`.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream`- The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let vis: &Visibility = self.get_visibility();
        let name: &Ident = self.get_name();
        let class_name_str: String = name.to_string();
        match self.try_get_params() {
            Some(params) => {
                let param_defs: Vec<proc_macro2::TokenStream> = params
                    .iter()
                    .map(|param: &CssVarParam| {
                        let param_name: &Ident = param.get_name();
                        let ty: &Type = param.get_ty();
                        quote! { #param_name: #ty }
                    })
                    .collect();
                let param_names: Vec<&Ident> = params
                    .iter()
                    .map(|param: &CssVarParam| param.get_name())
                    .collect();
                let css_string_parts: Vec<proc_macro2::TokenStream> = self
                    .get_vars()
                    .iter()
                    .map(|(key, value)| match value {
                        CssVarValue::Expr(expr) => {
                            quote! { #key.to_string() + #CSS_PROP_SEPARATOR + &(#expr).to_string() + #CSS_DECL_TERMINATOR }
                        }
                    })
                    .collect();
                let str_hyphen: &str = STR_HYPHEN;
                let name_format: String = format!("{{}}{str_hyphen}{{}}");
                tokens.extend(quote! {
                    #vis fn #name(#(#param_defs), *) -> ::euv::Css {
                        let css = ::euv::Css::new(format!(#name_format, #class_name_str, [#(format!("{:?}", #param_names)), *].join(#str_hyphen)), [#(#css_string_parts), *].concat(), vec![], vec![]);
                        css.inject_style();
                        css
                    }
                });
            }
            None => {
                let name_span: Span = name.span();
                let const_name: Ident = Ident::new(&class_name_str.to_uppercase(), name.span());
                let const_name_token: proc_macro2::TokenStream =
                    quote_spanned!(name_span=> #const_name);
                let fn_name_token: proc_macro2::TokenStream = quote_spanned!(name_span=> #name);
                let all_static: bool = self.get_vars().iter().all(|(_, value)| {
                    let CssVarValue::Expr(expr) = value;
                    is_static_string_expr(expr)
                });
                let style_expr = if all_static {
                    let mut css_string: String = String::new();
                    for (key, value) in self.get_vars() {
                        let CssVarValue::Expr(expr) = value;
                        css_string.push_str(key);
                        css_string.push_str(CSS_PROP_SEPARATOR);
                        css_string.push_str(&expr_to_string(expr));
                        css_string.push_str(CSS_DECL_TERMINATOR);
                    }
                    quote! { #css_string.to_string() }
                } else {
                    let css_string_parts: Vec<proc_macro2::TokenStream> = self
                        .get_vars()
                        .iter()
                        .map(|(key, value)| match value {
                            CssVarValue::Expr(expr) => {
                                quote! { #key.to_string() + #CSS_PROP_SEPARATOR + &(#expr).to_string() + #CSS_DECL_TERMINATOR }
                            }
                        })
                        .collect();
                    quote! { [#(#css_string_parts), *].concat() }
                };
                emit_css_var_once_lock_fn(
                    tokens, vis, &fn_name_token, &const_name_token,
                    &class_name_str, &style_expr,
                );
            }
        }
    }
}

/// Implementation of `ToTokens` for `CssVarInput`, converting CSS variable definitions into token streams.
impl ToTokens for CssVarInput {
    /// Converts all CSS variable definitions into token streams.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream`- The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for css_var_def in self.get_defs() {
            css_var_def.to_tokens(tokens);
        }
    }
}
