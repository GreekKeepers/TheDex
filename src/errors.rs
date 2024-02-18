use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error making request: {0}")]
    RequestError(ReqwestError),

    #[error("Error deserializing response: {0}")]
    SerdeError(SerdeError),
}
