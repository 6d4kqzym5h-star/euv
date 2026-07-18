use crate::*;

/// Parses the `computed!` macro input and generates a computed signal.
///
/// # Arguments
///
/// - `TokenStream` - The raw token stream representing computed expressions, closure, and return type.
///
/// # Returns
///
/// - `TokenStream` - The generated token stream that creates a signal whose value is
///   automatically derived from the watched signals via the closure.
pub(crate) fn parse_computed(input: TokenStream) -> TokenStream {
    let tokens: proc_macro2::TokenStream = match syn::parse::<ComputedInput>(input) {
        Ok(computed_input) => computed_input.into_token_stream(),
        Err(error) => return error.to_compile_error().into(),
    };
    TokenStream::from(tokens)
}
