use thiserror::Error;

/// Represents all possible errors returned by the Robin library.
///
/// This includes errors from netlink communication, I/O operations, parsing,
/// and cases where requested items are not found.
#[derive(Error, Debug)]
pub enum RobinError {
    /// Represents errors originating from netlink operations.
    ///
    /// Contains a `String` describing the underlying netlink error.
    #[error("Netlink error: {0}")]
    Netlink(String),

    /// Represents I/O related errors.
    ///
    /// Contains a `String` describing the underlying I/O failure.
    #[error("IO error: {0}")]
    Io(String),

    /// Represents errors encountered during parsing of netlink messages or other data.
    ///
    /// Contains a `String` describing the parsing issue.
    #[error("Parse error: {0}")]
    Parse(String),

    /// Indicates that a requested item was not found.
    ///
    /// Contains a `String` describing what could not be found.
    #[error("Not found: {0}")]
    NotFound(String),
}
