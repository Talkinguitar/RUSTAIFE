//IDEA: cosa sa fare un provider

use async_trait::async_trait;
use std::fmt;

#[derive(Debug)]
pub enum ErrorHandling {
    RequestError(reqwest::Error),
    AnswerError(String),
}

impl fmt::Display for ErrorHandling {
    //formattazione
    fn fmt(&self, error: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorHandling::RequestError(err) => write!(error, "HTTP error: {}", err),
            ErrorHandling::AnswerError(err_string) => write!(error, "API error: {}", err_string),
        }
    }
}

impl std::error::Error for ErrorHandling {}

impl From<reqwest::Error> for ErrorHandling {
    //trasformo gli errori dal tipo "reqwest::Error" che mi passa la crate reqwest al tipo ErrorHandling
    fn from(error: reqwest::Error) -> Self {
        ErrorHandling::RequestError(error)
    }
}

#[async_trait]
pub trait Provider {
    async fn identify_and_answer(&self, prompt: &str) -> Result<String, ErrorHandling>;
}
