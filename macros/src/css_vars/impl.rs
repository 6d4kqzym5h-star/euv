use crate::*;

/// Parses the `css_vars!` macro input.
impl Parse for CssVarInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
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
                    let ty: SynType = param_content.parse()?;
                    param_list.push(CssVarParam {
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
            let mut vars: Vec<(String, CssVarValue)> = Vec::new();
            while !content.is_empty() {
                let var_name: String = parse_kebab_name(&content)?;
                let css_key: String = format!("--{}", var_name);
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
        Ok(CssVarInput { defs })
    }
}

/// Converts a `CssVarDef` into token stream generating a `euv_core::CssClass` function.
///
/// Each CSS variable block becomes a `CssClass` function that, when called, injects
/// the CSS custom properties into the DOM and returns a reference to the class.
/// The CSS key names are prefixed with `--`.
impl ToTokens for CssVarDef {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let vis: &Visibility = &self.visibility;
        let name: &Ident = &self.name;
        let class_name_str: String = name.to_string();
        match &self.params {
            Some(params) => {
                let param_defs: Vec<TokenStream2> = params
                    .iter()
                    .map(|param| {
                        let param_name: &Ident = &param.name;
                        let ty: &SynType = &param.ty;
                        quote! { #param_name: #ty }
                    })
                    .collect();
                let param_names: Vec<&Ident> = params.iter().map(|p| &p.name).collect();
                let css_string_parts: Vec<TokenStream2> = self
                    .vars
                    .iter()
                    .map(|(key, value)| match value {
                        CssVarValue::Expr(expr) => {
                            quote! { #key.to_string() + ": " + &(#expr).to_string() + "; " }
                        }
                    })
                    .collect();
                tokens.extend(quote! {
                    #vis fn #name(#(#param_defs),*) -> euv_core::CssClass {
                        let __css_string: String = [#(#css_string_parts),*].concat();
                        let __unique_name: String = format!("{}-{}", #class_name_str, [#(format!("{:?}", #param_names)),*].join("-"));
                        euv_core::CssClass::new(__unique_name, __css_string)
                    }
                });
            }
            None => {
                let css_string_parts: Vec<TokenStream2> = self
                    .vars
                    .iter()
                    .map(|(key, value)| match value {
                        CssVarValue::Expr(expr) => {
                            quote! { #key.to_string() + ": " + &(#expr).to_string() + "; " }
                        }
                    })
                    .collect();
                let name_span: Span = name.span();
                let const_name: Ident = Ident::new(&class_name_str.to_uppercase(), name.span());
                let const_name_token: TokenStream2 = quote_spanned!(name_span=> #const_name);
                let fn_name_token: TokenStream2 = quote_spanned!(name_span=> #name);
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

/// Converts a `CssVarInput` into a token stream of `CssClass` function definitions.
impl ToTokens for CssVarInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for css_var_def in &self.defs {
            css_var_def.to_tokens(tokens);
        }
    }
}
