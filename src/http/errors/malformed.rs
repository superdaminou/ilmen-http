use core::fmt;
use std::{error::Error, str::Utf8Error, num::ParseIntError};

use crate::http::structs::{HTTPResponse, Response};


#[derive(Debug)]
pub struct MalformedError  {
    details: String
}

impl fmt::Display for MalformedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MalformedError {
    fn description(&self) -> &str {
        &self.details
    }
}


impl From<&str> for MalformedError {
    fn from(err: &str) -> Self {
        MalformedError::from(err.to_string())
    }
}

impl From<String> for MalformedError {
    fn from(msg: String) -> MalformedError {
        MalformedError{details: msg}
    }
}


impl From<Utf8Error> for MalformedError {
    fn from(err: Utf8Error) -> Self {
        MalformedError::from(err.to_string())
    }
}

impl From<ParseIntError> for MalformedError {
    fn from(_: ParseIntError) -> Self {
        MalformedError::from("Expected a valid integer".to_string())
    }
}

impl From<MalformedError> for HTTPResponse {
    fn from(_: MalformedError) -> Self {
        return HTTPResponse::from(Response::from((400, "Malformed Url")));
    }
}

impl PartialEq<MalformedError> for MalformedError {
    fn eq(&self, other: &MalformedError) -> bool {
          return self.details == other.details;
    }
}