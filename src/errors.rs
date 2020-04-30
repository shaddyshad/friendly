use serde::Serialize;

/// Intent parsing errors
#[derive(Debug, Serialize)]
pub enum Errors {
    NetworkError(String),
    ParsingError,
    InvalidInput(String),
    InternalError(String)   
}

