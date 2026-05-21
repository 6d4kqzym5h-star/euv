use crate::*;

/// Implementation of `Parse` for `WatchInput`, parsing the `watch!` macro input.
///
/// Syntax: `watch!(signal1, signal2, ..., |param1, param2, ...| { body })`
///
/// The expressions before the closure are signal expressions.
/// The closure parameters correspond to `.get()` values of the respective signals.
impl Parse for WatchInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

/// Implementation of `ToTokens` for `WatchInput`, converting watch input into reactive subscription code.
///
/// Generated code:
/// 1. Uses a `use_signal(|| false)` guard to ensure subscriptions and
///    initial body execution only happen once per DynamicNode lifecycle,
///    preventing duplicate subscriptions and infinite re-render loops.
/// 2. Clones each signal into a local binding.
/// 3. On first execution, the entire initialisation (subscribe registration
///    and body execution) is wrapped in `with_suppressed_updates` so that
///    any `.set()` calls inside the body do not trigger premature
///    `schedule_signal_update()` dispatches. The guard signal is updated
///    via `set_silent` to avoid an unnecessary DOM re-render cycle.
/// 4. Subsequent render_fn invocations skip the block entirely — the body
///    only fires via the `subscribe` callbacks when a watched signal
///    actually changes.
impl ToTokens for WatchInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let signal_clones: Vec<Ident> = (0..self.get_signals().len())
            .map(|i: usize| Ident::new(&format!("__euv_watch_signal_{}", i), Span::call_site()))
            .collect();
        let signal_exprs: &Vec<Expr> = self.get_signals();
        let param_names: &Vec<Ident> = self.get_param_names();
        let body: &Vec<syn::Stmt> = self.get_body();
        let get_calls: Vec<proc_macro2::TokenStream> = signal_clones
            .iter()
            .zip(param_names.iter())
            .map(|(signal_clone, _param)| {
                quote! { #signal_clone.get() }
            })
            .collect();
        let all_gets: Vec<proc_macro2::TokenStream> = signal_clones
            .iter()
            .zip(param_names.iter())
            .map(|(sc, param)| quote! { let #param = #sc.get(); })
            .collect();
        let subscribe_calls: Vec<proc_macro2::TokenStream> = signal_clones
            .iter()
            .map(|signal_clone| {
                quote! {
                    {
                        let #signal_clone: _ = #signal_clone;
                        let __euv_watch_fire_clone: ::std::rc::Rc<std::cell::RefCell<dyn ::std::ops::FnMut()>> = ::std::rc::Rc::clone(&__euv_watch_fire);
                        #signal_clone.subscribe(move || {
                            (__euv_watch_fire_clone.borrow_mut())();
                        });
                    }
                }
            })
            .collect();
        tokens.extend(quote! {{
            #(let #signal_clones = #signal_exprs;)*
            let __euv_watch_subscribed: ::euv_core::Signal<bool> = ::euv_core::use_signal(|| false);
            if !__euv_watch_subscribed.get() {
                let __euv_watch_fire: ::std::rc::Rc<std::cell::RefCell<dyn ::std::ops::FnMut()>> = ::std::rc::Rc::new(std::cell::RefCell::new(move || {
                    #(#all_gets)*
                    { #(#body)* }
                }));
                ::euv_core::with_suppressed_updates(|| {
                    #(#subscribe_calls)*
                    {
                        #(let #param_names = #get_calls;)*
                        { #(#body)* }
                    }
                    __euv_watch_subscribed.set_silent(true);
                });
            }
        }});
    }
}
