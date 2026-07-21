use super::*;

/// Custom logger implementation for the euv CLI.
///
/// Implements the `log::Log` trait to provide colored console output
/// matching the hyperlane-quick-start log format.
#[derive(Data, New)]
pub struct Logger;
