use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("{0}")]
    ArchNotImplemented(String),
    #[error("{0}")]
    FuncNotImplemented(String),
}