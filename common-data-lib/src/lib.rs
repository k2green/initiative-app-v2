use serde::{Deserialize, Serialize};

pub mod creatures;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackendError {
    InternalError(String),
    LogicError(String),
    ArgumentError{ argument_name: String, message: String},
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalError(e) => write!(f, "Internal Error: {}", e),
            Self::LogicError(e) => write!(f, "Logic Error: {}", e),
            Self::ArgumentError { argument_name, message } => write!(f, "Error with argument '{}': {}", argument_name, message),
        }
    }
}

impl<T: std::error::Error> From<T> for BackendError {
    fn from(value: T) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl BackendError {
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalError(message.into())
    }
    
    pub fn logic_error(message: impl Into<String>) -> Self {
        Self::LogicError(message.into())
    }

    pub fn argument_error(arg_name: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ArgumentError {
            argument_name: arg_name.into(),
            message: message.into()
        }
    }
}

pub trait ToBackendResult<T> {
    fn to_backend_result(self) -> Result<T, BackendError>;
}

impl<T, E: std::error::Error> ToBackendResult<T> for Result<T, E> {
    fn to_backend_result(self) -> Result<T, BackendError> {
        self.map_err(|e| BackendError::from(e))
    }
}