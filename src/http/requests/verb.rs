use std::str::FromStr;

use strum_macros::Display;

use crate::http::errors::http_errors::HttpError;


#[derive(PartialEq, Eq, Clone, Copy, Debug, Display)]
pub enum Verb {
    POST,
    GET,
    PUT,
    DELETE,
    PATCH,
    OPTION
}

impl FromStr for Verb {
    type Err = HttpError;

    fn from_str(input: &str) -> Result<Verb, HttpError> {
        match input {
            "POST" => Ok(Self::POST),
            "GET" => Ok(Self::GET),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "PATCH" => Ok(Self::PATCH),
            "OPTIONS" => Ok(Self::OPTION),
            _ =>  Err(HttpError::BadRequest(format!("Unknown verb: {}", input)))
        }
    }
}