use reqwest::Error as ReqwestError;

// Define a custom error enum
#[derive(Debug)]
pub enum RequestError {
    ReqwestError(ReqwestError),
    NoBodyError(String),
}

impl From<ReqwestError> for RequestError {
    fn from(error: ReqwestError) -> Self {
        RequestError::ReqwestError(error)
    }
}
