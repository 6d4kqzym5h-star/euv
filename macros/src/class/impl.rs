use crate::*;

/// Parses the `class!` macro input.
impl Parse for ClassInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
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
            while !content.is_empty() {
                let css_key: String = parse_kebab_name(&content)?;
                content.parse::<Token![:]>()?;
                let expr: Expr = content.parse()?;
                let expanded: TokenStream2 = expand_var_macros(&expr);
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
            });
        }
        Ok(ClassInput { classes })
    }
}

/// Converts a `ClassDef` into token stream generating a `euv_core::CssClass` function.
impl ToTokens for ClassDef {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let vis: &syn::Visibility = &self.visibility;
        let name: &Ident = &self.name;
        let class_name_str: String = name.to_string();
        match &self.params {
            Some(params) => {
                let param_defs: Vec<TokenStream2> = params
                    .iter()
                    .map(|param| {
                        let param_name: &Ident = &param.name;
                        let ty: &syn::Type = &param.ty;
                        quote! { #param_name: #ty }
                    })
                    .collect();
                let param_names: Vec<&Ident> = params.iter().map(|p| &p.name).collect();
                let css_string_parts: Vec<TokenStream2> = self
                    .properties
                    .iter()
                    .map(|(key, value)| match value {
                        ClassPropValue::Expr(expr) => {
                            quote! { #key.to_string() + ": " + &(#expr) + "; " }
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
                    .properties
                    .iter()
                    .map(|(key, value)| match value {
                        ClassPropValue::Expr(expr) => {
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

/// Converts a `ClassInput` into a token stream of `CssClass` function definitions.
impl ToTokens for ClassInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for class_def in &self.classes {
            class_def.to_tokens(tokens);
        }
    }
}
