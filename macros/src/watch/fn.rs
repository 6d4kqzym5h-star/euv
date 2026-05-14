use crate::*;

/// Parses the `watch!` macro input and generates reactive subscription code.
///
/// # Arguments
///
/// - `TokenStream`: The raw token stream representing watch expressions and closure.
///
/// # Returns
///
/// - `TokenStream`: The generated token stream that subscribes to signal changes
///   and executes the closure when any watched signal updates.
pub fn parse_watch(input: TokenStream) -> TokenStream {
    let watch_input: WatchInput = parse_macro_input!(input as WatchInput);
    let tokens: TokenStream2 = watch_input.into_token_stream();
    TokenStream::from(tokens)
}
