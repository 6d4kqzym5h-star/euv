use crate::*;

/// Parses the `watch!` macro input.
///
/// Syntax: `watch!(signal1, signal2, ..., |param1, param2, ...| { body })`
///
/// The expressions before the closure are signal expressions.
/// The closure parameters correspond to `.get()` values of the respective signals.
impl Parse for WatchInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut signals: Vec<Expr> = Vec::new();
        while !input.peek(Token![|]) {
            let expr: Expr = input.parse()?;
            signals.push(expr);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        input.parse::<Token![|]>()?;
        let mut param_names: Vec<Ident> = Vec::new();
        while !input.peek(Token![|]) {
            let name: Ident = input.parse()?;
            param_names.push(name);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        input.parse::<Token![|]>()?;
        let body_content;
        braced!(body_content in input);
        let mut body: Vec<syn::Stmt> = Vec::new();
        while !body_content.is_empty() {
            let stmt: syn::Stmt = body_content.parse()?;
            body.push(stmt);
        }
        if signals.len() != param_names.len() {
            return Err(input.error(
                "the number of signal expressions must match the number of closure parameters",
            ));
        }
        Ok(WatchInput {
            signals,
            param_names,
            body,
        })
    }
}

/// Converts a `WatchInput` into reactive subscription code.
///
/// Generated code:
/// 1. Clones each signal into a local binding.
/// 2. Subscribes each signal with a closure that reads all current signal values
///    and invokes the user-provided body.
/// 3. Executes the body once immediately with the current signal values.
impl ToTokens for WatchInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let signal_clones: Vec<Ident> = (0..self.signals.len())
            .map(|i: usize| {
                Ident::new(
                    &format!("__euv_watch_signal_{}", i),
                    proc_macro2::Span::call_site(),
                )
            })
            .collect();
        let signal_exprs: &Vec<Expr> = &self.signals;
        let param_names: &Vec<Ident> = &self.param_names;
        let body: &Vec<syn::Stmt> = &self.body;
        let get_calls: Vec<TokenStream2> = signal_clones
            .iter()
            .zip(param_names.iter())
            .map(|(signal_clone, _param)| {
                quote! { #signal_clone.get() }
            })
            .collect();
        let subscribe_calls: Vec<TokenStream2> = signal_clones
            .iter()
            .map(|signal_clone| {
                let all_gets: Vec<TokenStream2> = signal_clones
                    .iter()
                    .zip(param_names.iter())
                    .map(|(sc, _p)| quote! { let #_p = #sc.get(); })
                    .collect();
                quote! {
                    {
                        let #signal_clone: _ = #signal_clone;
                        #signal_clone.subscribe(move || {
                            #(#all_gets)*
                            { #(#body)* }
                        });
                    }
                }
            })
            .collect();
        tokens.extend(quote! {{
            #(let #signal_clones = #signal_exprs;)*
            #(#subscribe_calls)*
            {
                #(let #param_names = #get_calls;)*
                { #(#body)* }
            }
        }});
    }
}
