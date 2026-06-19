use crate::*;

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
