use crate::*;

/// Parses the `css_vars!` macro input and generates `CssClass` function definitions.
///
/// # Arguments
///
/// - `TokenStream` - The raw token stream representing CSS variable block definitions.
///
/// # Returns
///
/// - `TokenStream` - The generated token stream constructing `CssClass` functions.
pub(crate) fn parse_css_vars(input: TokenStream) -> TokenStream {
    let css_var_input: CssVarInput = parse_macro_input!(input as CssVarInput);
    let tokens: TokenStream2 = css_var_input.into_token_stream();
    TokenStream::from(tokens)
}
