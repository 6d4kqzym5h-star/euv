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
pub(crate) fn emit_vars_once_lock_fn(
    tokens: &mut proc_macro2::TokenStream,
    visibility: &Visibility,
    fn_name_token: &proc_macro2::TokenStream,
    const_name_token: &proc_macro2::TokenStream,
    class_name_str: &str,
    style_expr: &proc_macro2::TokenStream,
) {
    tokens.extend(quote! {
        #visibility fn #fn_name_token() -> &'static ::euv::Css {
            static #const_name_token: ::std::sync::OnceLock<::euv::Css> = ::std::sync::OnceLock::new();
            #const_name_token.get_or_init(|| {
                let css: ::euv::Css = ::euv::Css::new(#class_name_str.to_string(), #style_expr, vec![], vec![]);
                css.inject_style();
                css
            })
        }
    });
}

/// Parses the `vars!` macro input and generates `Css` function definitions.
///
/// # Arguments
///
/// - `TokenStream` - The raw token stream representing CSS variable block definitions.
///
/// # Returns
///
/// - `TokenStream` - The generated token stream constructing `Css` functions.
pub(crate) fn parse_vars(input: TokenStream) -> TokenStream {
    let tokens: proc_macro2::TokenStream = match syn::parse::<VarsInput>(input) {
        Ok(vars_input) => vars_input.into_token_stream(),
        Err(error) => return error.to_compile_error().into(),
    };
    TokenStream::from(tokens)
}

/// Parses the `var!` macro input and generates a CSS `var()` function string.
///
/// The variable name can be written in two forms:
/// - String literal: `var!("bg-primary")` → `"var(--bg-primary)"`
/// - Unquoted kebab-case: `var!(bg-primary)` → `"var(--bg-primary)"`
///
/// # Arguments
///
/// - `TokenStream` - The raw token stream containing the variable name.
///
/// # Returns
///
/// - `TokenStream` - The generated token stream producing the CSS `var()` string.
pub(crate) fn parse_var(input: TokenStream) -> TokenStream {
    let tokens: proc_macro2::TokenStream = input.into();
    let var_name: String = reconstruct_ident_from_tokens(&tokens);
    let css_name: String = format!("{CSS_VAR_PREFIX}{var_name}{CSS_VAR_SUFFIX}");
    TokenStream::from(quote! { #css_name })
}
