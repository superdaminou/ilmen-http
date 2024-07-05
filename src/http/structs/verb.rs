use std::str::FromStr;

use crate::http::errors::malformed::MalformedError;


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Verb {
    POST,
    GET,
    PUT,
    DELETE,
    PATCH,
    OPTION
}

impl FromStr for Verb {
    type Err = MalformedError;

    fn from_str(input: &str) -> Result<Verb, MalformedError> {
        return match input {
            "POST" => Ok(Self::POST),
            "GET" => Ok(Self::GET),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "PATCH" => Ok(Self::PATCH),
            "OPTIONS" => Ok(Self::OPTION),
            _ =>  Err(MalformedError::from(format!("Unknown verb: {}", input)))
        }
    }
}