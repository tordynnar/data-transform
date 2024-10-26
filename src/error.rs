use wasmtime_wasi::{SocketError, StreamError};

#[derive(thiserror::Error, Debug)]
pub enum RunError {
    #[error("sockets not implemented")]
    SocketsNotImplemented,
    
    #[error("sleep not implemented")]
    SleepNotImplemented,

    #[error("exit called")]
    ExitCalled,

    #[error("unexpected error")]
    Unexpected,

    #[error("unexpected failure to unwrap mutex")]
    UnexpectedMutexError,
}

impl From<RunError> for SocketError {
    fn from(error: RunError) -> Self {
        Self::trap(error)
    }
}

impl From<RunError> for StreamError {
    fn from(error: RunError) -> Self {
        Self::LastOperationFailed(error.into())
    }
}
