use crate::*;

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
