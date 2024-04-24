use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("{0}")]
    ArchNotImplemented(String),
    #[error("{0}")]
    FuncNotImplemented(String),
    #[error("No valid free memory was found of size: {0}")]
    NoValidFreeMemoryFound(i32),
    #[error("No map found at address ({0})!")]
    MapNotFound(i32),
    #[error("Segmentation fault at address ({0})! {1}")]
    SegmentationViolation(i32, String),
    #[error("Invalid state: {0}")]
    InvalidState(String),
    #[error("Invalid register name: {0}")]
    InvalidRegisterName(String),
    #[error("Invalid calling convention: {0}")]
    UnknownCallingConvention(String),
    #[error("Error: {0}")]
    Generic(String)
}