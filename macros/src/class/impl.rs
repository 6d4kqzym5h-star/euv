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
            let mut properties: Vec<(Ident, ClassPropValue)> = Vec::new();
            while !content.is_empty() {
                let prop_key: Ident = content.parse()?;
                content.parse::<Token![:]>()?;
                let prop_value: ClassPropValue = if content.peek(syn::token::Brace) {
                    let expr_content;
                    braced!(expr_content in content);
                    let expr: Expr = expr_content.parse()?;
                    ClassPropValue::Expr(expr.into_token_stream())
                } else {
                    let lit: LitStr = content.parse()?;
                    ClassPropValue::Literal(lit.value())
                };
                properties.push((prop_key, prop_value));
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

/// Converts a `ClassDef` into token stream generating a `euv_core::vdom::CssClass` function.
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
                    .map(|(key, value)| {
                        let css_key: String = key.to_string().replace('_', "-");
                        match value {
                            ClassPropValue::Literal(lit) => {
                                let css_val: String = lit.clone();
                                quote! { #css_key.to_string() + ": " + #css_val + "; " }
                            }
                            ClassPropValue::Expr(expr) => {
                                quote! { #css_key.to_string() + ": " + &(#expr) + "; " }
                            }
                        }
                    })
                    .collect();
                tokens.extend(quote! {
                    #[allow(non_snake_case)]
                    #vis fn #name(#(#param_defs),*) -> euv_core::vdom::CssClass {
                        let __css_string: String = [#(#css_string_parts),*].concat();
                        let __unique_name: String = format!("{}-{}", #class_name_str, [#(format!("{:?}", #param_names)),*].join("-"));
                        euv_core::vdom::CssClass::new(__unique_name, __css_string)
                    }
                });
            }
            None => {
                let css_string: String = self
                    .properties
                    .iter()
                    .map(|(key, value)| {
                        let css_key: String = key.to_string().replace('_', "-");
                        match value {
                            ClassPropValue::Literal(lit) => {
                                format!("{}: {};", css_key, lit)
                            }
                            ClassPropValue::Expr(_) => {
                                unreachable!(
                                    "non-parameterized class should not have expression values"
                                )
                            }
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                let const_name: Ident = Ident::new(&class_name_str.to_uppercase(), name.span());
                let fn_name: Ident = name.clone();
                tokens.extend(quote! {
                    #[allow(non_snake_case)]
                    #vis fn #fn_name() -> &'static euv_core::vdom::CssClass {
                        static #const_name: std::sync::OnceLock<euv_core::vdom::CssClass> = std::sync::OnceLock::new();
                        #const_name.get_or_init(|| euv_core::vdom::CssClass::new(#class_name_str.to_string(), #css_string.to_string()))
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
