use thiserror::Error;

#[derive(Error, Debug)]
pub enum RobinError {
    #[error("Netlink error: {0}")]
    Netlink(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Not found: {0}")]
    NotFound(String),
}
