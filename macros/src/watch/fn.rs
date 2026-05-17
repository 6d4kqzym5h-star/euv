use crate::*;

/// Parses the `watch!` macro input and generates reactive subscription code.
///
/// # Arguments
///
/// - `TokenStream` - The raw token stream representing watch expressions and closure.
///
/// # Returns
///
/// - `TokenStream` - The generated token stream that subscribes to signal changes
///   and executes the closure when any watched signal updates.
pub fn parse_watch(input: TokenStream) -> TokenStream {
    let tokens: TokenStream2 = match syn::parse::<WatchInput>(input) {
        Ok(watch_input) => watch_input.into_token_stream(),
        Err(error) => return error.to_compile_error().into(),
    };
    TokenStream::from(tokens)
}
