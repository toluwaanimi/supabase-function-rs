use std::fmt;

#[derive(Debug)]
pub enum FunctionsError {
    FetchError(String),
    HttpError(String),
    RelayError(String),
}

impl fmt::Display for FunctionsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionsError::FetchError(msg) => write!(f, "FetchError: {}", msg),
            FunctionsError::HttpError(msg) => write!(f, "HttpError: {}", msg),
            FunctionsError::RelayError(msg) => write!(f, "RelayError: {}", msg),
        }
    }
}

impl std::error::Error for FunctionsError {}

pub struct FunctionsFetchError;

impl FunctionsFetchError {
    pub fn new(context: String) -> FunctionsError {
        FunctionsError::FetchError(context)
    }
}

pub struct FunctionsRelayError;

impl FunctionsRelayError {
    pub fn new(context: String) -> FunctionsError {
        FunctionsError::RelayError(context)
    }
}

pub struct FunctionsHttpError;

impl FunctionsHttpError {
    pub fn new(context: String) -> FunctionsError {
        FunctionsError::HttpError(context)
    }
}
