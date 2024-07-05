use core::fmt;
use std::{error::Error, str::Utf8Error};

use crate::http::structs::{HTTPResponse, Response};

#[derive(Debug)]
pub struct InternalError  {
    pub details: String
}

impl InternalError {
    fn new(msg: String) -> InternalError {
        InternalError{details: msg}
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for InternalError {
    fn description(&self) -> &str {
        &self.details
    }
}


impl From<&str> for InternalError {
    fn from(err: &str) -> Self {
        InternalError::new(err.to_string())
    }
}


impl From<Utf8Error> for InternalError {
    fn from(err: Utf8Error) -> Self {
        InternalError::new(err.to_string())
    }
}

impl From<InternalError> for HTTPResponse {
    fn from(_: InternalError) -> Self {
        return HTTPResponse::from(Response::from((500,"Internal Server Error")));
    }
}
