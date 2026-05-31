use std::{io, path::PathBuf};

/// The error type for the euv CLI application.
///
/// Covers all failure modes including I/O errors, server errors,
/// HTML generation errors, and general application errors.
#[derive(Debug)]
pub enum EuvError {
    /// An I/O error with additional context about the operation.
    Io {
        /// A human-readable description of the failed I/O operation.
        message: String,
        /// The underlying I/O error.
        error: io::Error,
    },
    /// An I/O error with path context.
    IoPath {
        /// A human-readable description of the failed I/O operation.
        message: String,
        /// The file or directory path involved in the I/O operation.
        path: PathBuf,
        /// The underlying I/O error.
        error: io::Error,
    },
    /// A UTF-8 decoding error.
    Utf8 {
        /// A human-readable description of the UTF-8 decoding failure.
        message: String,
        /// The underlying UTF-8 error.
        error: std::string::FromUtf8Error,
    },
    /// A server error from the hyperlane framework.
    Server(String),
    /// A general application error with a descriptive message.
    Message(String),
    /// A file watcher (notify) error.
    Watch(notify::Error),
}
