use std::str::FromStr;

use crate::{http::errors::http_errors::HttpError, Verb};


#[derive(PartialEq, Debug, Clone)]
pub struct StartLine{
    protocol:Protocol,
    verb: Verb,
    resource: Resource
}

pub type Resource = String; 
pub type Protocol = String; 


impl Default for StartLine  {
    fn default() -> Self {
        StartLine {
            protocol: "HTTP/1.1".to_string(), 
            verb: Verb::GET, 
            resource: "/".to_string()
        }
    }
}

impl StartLine {
    pub fn new(verb: Verb, resource: Resource) -> StartLine {
        StartLine {
            verb,
            resource,
            ..Default::default()
        }
    } 

    pub fn resource(&self) -> String {
        self.resource.clone()
    }
    pub fn verb(&self) -> Verb {
        self.verb
    }
}

impl TryFrom<Vec<String>> for StartLine {
    type Error = HttpError;
    fn try_from(value : Vec<String>) -> Result<StartLine, HttpError> {
        value.first()
            .map(|start_line| StartLine::try_from((*start_line).clone()))
            .unwrap_or(Err(HttpError::BadRequest("Need start line".to_string())))
    }
    
}

impl ToString for StartLine {
    fn to_string(&self) -> String {
        format!("{} {}", self.verb, self.resource)
    }
}

impl TryFrom<String> for StartLine {
    type Error = HttpError;
        fn try_from(value: String) -> Result<Self, HttpError> {
            let binding = value
                .split(' ')
                .take(3).collect::<Vec<&str>>();
            let mut decomposed_start_line =  binding
                .iter();
            
            let verb = decomposed_start_line
                .next()
                .ok_or(HttpError::BadRequest("Missing verb".to_string()))
                .map(|&verb| Verb::from_str(verb))??;


            let resource = decomposed_start_line
                .next()
                .ok_or(HttpError::BadRequest("No ressource".to_string()))
                .map(|&str| str.to_string())?;

            Ok(StartLine{verb, resource, ..Default::default()})
    }
}


#[cfg(test)]
mod tests {
    use crate::Verb;
    use super::*;

    #[test]
    fn start_line_try_from_ok() {
        let result = StartLine::try_from("GET /test HTTP/1.1".to_string()).unwrap();
        assert_eq!(result.resource, "/test");
        assert_eq!(result.verb, Verb::GET)
    }

    #[test]
    fn start_line_try_from_malformed_verb() {
        let result = StartLine::try_from("TOTO / HTTP/1.1".to_string());
        assert_eq!(result.err().unwrap(), HttpError::BadRequest("Unknown verb: TOTO".to_string()));
    }

    #[test]
    fn start_line_try_from_default() {
        let result = StartLine::try_from("GET / HTTP/1.1".to_string()).unwrap();
        assert_eq!(result.resource, "/");
    }
}