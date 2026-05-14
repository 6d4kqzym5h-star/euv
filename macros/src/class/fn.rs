use crate::*;

/// Parses the `class!` macro input and generates `CssClass` function definitions.
///
/// # Arguments
///
/// - `TokenStream`: The raw token stream representing class definitions.
///
/// # Returns
///
/// - `TokenStream`: The generated token stream constructing `CssClass` functions.
pub fn parse_class(input: TokenStream) -> TokenStream {
    let class_input: ClassInput = parse_macro_input!(input as ClassInput);
    let tokens: TokenStream2 = class_input.into_token_stream();
    TokenStream::from(tokens)
}
