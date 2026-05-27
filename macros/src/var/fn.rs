use crate::*;

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
    let var_name: String = reconstruct_kebab_from_tokens(&tokens);
    let css_name: String = format!("{CSS_VAR_PREFIX}{var_name}{CSS_VAR_SUFFIX}");
    TokenStream::from(quote! { #css_name })
}
