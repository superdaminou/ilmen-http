use std::{error::Error, fmt};


#[derive(Debug)]
pub struct SecurityError  {
    pub details: String
}

impl fmt::Display for SecurityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for SecurityError {
    fn description(&self) -> &str {
        &self.details
    }
}