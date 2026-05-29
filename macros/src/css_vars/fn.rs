use crate::*;

/// Generates the `OnceLock`-based static function body for a no-param CSS variable block.
///
/// # Arguments
///
/// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
/// - `&Visibility` - The visibility modifier.
/// - `&proc_macro2::TokenStream` - The function name token stream.
/// - `&proc_macro2::TokenStream` - The `OnceLock` constant name token stream.
/// - `&str` - The class name string literal.
/// - `&proc_macro2::TokenStream` - Token stream that evaluates to the CSS style string.
pub(crate) fn emit_css_var_once_lock_fn(
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
                let css: ::euv::Css = ::euv::Css::new(#class_name_str.to_string(), #style_expr, vec![], vec![]);
                css.inject_style();
                css
            })
        }
    });
}

/// Parses the `css_vars!` macro input and generates `Css` function definitions.
///
/// # Arguments
///
/// - `TokenStream` - The raw token stream representing CSS variable block definitions.
///
/// # Returns
///
/// - `TokenStream` - The generated token stream constructing `Css` functions.
pub(crate) fn parse_css_vars(input: TokenStream) -> TokenStream {
    let tokens: proc_macro2::TokenStream = match syn::parse::<CssVarInput>(input) {
        Ok(css_var_input) => css_var_input.into_token_stream(),
        Err(error) => return error.to_compile_error().into(),
    };
    TokenStream::from(tokens)
}
