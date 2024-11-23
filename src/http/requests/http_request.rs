use std::collections::HashMap;
use std::str::FromStr;
use std::usize;
use std::str;
use log::info;

use crate::http::errors::http_errors::HttpError;
use crate::http::header::Headers;
use crate::Verb;

#[derive(Clone)]
pub struct HTTPRequest {
    pub protocol: Protocol,
    pub verb: Verb,
    pub resource: Resource, 
    pub query_params: Option<QueryParams>,
    pub headers: Option<Headers>,
    pub body: Option<Body>
}

pub type Resource = String; 
pub type Protocol = String; 
pub type QueryParams = HashMap<String, String>;
type Body = String;

impl Default for HTTPRequest {
    fn default() -> Self {
        Self { protocol: Default::default(), verb: Verb::GET, resource: "/".to_string(), query_params: Default::default(), headers: Default::default(), body: Default::default() }
    }
}

impl TryFrom<[u8; 1024]> for HTTPRequest {
    type Error = HttpError;
    fn try_from(value: [u8; 1024]) -> Result<Self, HttpError> {
        str::from_utf8(&value)
        .map_err(|error| 
            HttpError::BadRequest(error.to_string()))
        .map(HTTPRequest::try_from)?
    }
}

impl TryFrom<Vec<String>> for Verb {
    type Error = HttpError;

    fn try_from(value: Vec<String>) -> Result<Self, HttpError> {
        value.first()
            .ok_or(HttpError::BadRequest("Missing verb".to_string()))
            .and_then(|verb| Verb::from_str(verb))
    }
}


impl TryFrom<&str> for HTTPRequest {
    type Error = HttpError;
    fn try_from(buffer: &str) -> Result<Self, HttpError> {
        let parsed_request = parse(buffer);
        
        let decomposed_start_line = parsed_request.first()
            .ok_or(HttpError::BadRequest("Mising ressources".to_string()))?
            .split(' ')
            .take(3)
            .map(String::from)
            .collect::<Vec<String>>();

        let verb = Verb::try_from(decomposed_start_line.clone())?;

        let protocol = decomposed_start_line
            .get(2)
            .ok_or(HttpError::BadRequest("Missing protocol".to_string()))?
            .to_string();


        let requested_resource = decomposed_start_line
            .get(1)
            .ok_or(HttpError::BadRequest("No ressource".to_string()))
            .map(|str| str.split("?").collect::<Vec<&str>>())?; 

        let resource = requested_resource.first().ok_or(HttpError::BadRequest("Missing path".to_string()))?;

        let query_params = requested_resource.get(1)
            .map(|params| params.split("&").collect::<Vec<&str>>())
            .map(|vec_params| 
                vec_params.iter()
                    .map(|couple| couple
                        .split_once("=")
                        .unwrap_or((couple, "")))
                    .map(|(a,b)|(a.to_string(), b.to_string()))
                    .collect::<QueryParams>());


        let headers = extract_headers(parsed_request.clone());
        info!("Headers: {:?}", headers);
        
        let body = get_header(&headers, "Content-Length")
            .map(|(_, length)| length.parse::<usize>())
            .transpose()
            .map_err(|_| HttpError::BadRequest("Content Length not a number".to_string()))?
            .map(|length| extract_body(buffer, length));

        Ok (HTTPRequest {protocol, 
            verb, 
            query_params, 
            headers: Some(headers), 
            body,
            resource: resource.to_string()})
    }
}


fn extract_body(request : &str, content_length: usize) -> Body {
    return request.split_once("\r\n\r\n")
        .map(|(_, b)| b[0..content_length].to_string())
        .unwrap_or_default();
}


fn parse(buffer : &str) -> Vec<String> {
    return buffer
        .trim_matches(char::from(0))
        .split("\r\n")
        .map(|str| str.to_string())
        .collect::<Vec<String>>();
}

impl HTTPRequest {
    pub fn get_header(&self, key: &str) -> Option<(String, String)> {
        self.headers.clone().unwrap_or_default()
            .iter()
            .find(|(header, _)| header.to_lowercase().starts_with(&key.to_lowercase())).cloned()
    }
}

fn get_header(headers: &Headers, key: &str) -> Option<(String, String)> {
    headers
        .iter()
        .find(|(header, _)| header.to_lowercase().starts_with(&key.to_lowercase())).cloned()
}

fn extract_headers(request : Vec<String>) -> Headers {
    request.iter()
        .skip(1)
        .take_while(|&str| str.trim() != "")
        .map(|header| header.split_once(':'))
        .map(|spliterator| spliterator.unwrap_or_default())
        .map(|(cle, value)| (cle.trim().to_lowercase().to_owned(), value.trim().to_owned()))
        .collect::<Headers>()
}



// UNIT TEST
#[cfg(test)]
mod tests {
    use std::vec;
    use crate::Verb;

    use super::*;

    #[test]
    fn request_try_from_ok() {
        let buffer = "POST rappel/1?moi=toi&toi=moi HTTP/1.1\r\nContent-Length: 10\r\n\r\ntoto\r\ntata";

        let mut expected_query_params = HashMap::new();
        expected_query_params.insert("moi".to_string(), "toi".to_string());
        expected_query_params.insert("toi".to_string(),"moi".to_string());
        
        let  request = HTTPRequest::try_from(buffer);
        
        let http_request = request.unwrap();
        assert_eq!(http_request.verb, Verb::POST);
        assert_eq!(http_request.resource, "rappel/1");
        assert_eq!(http_request.query_params, Some(expected_query_params));
        assert_eq!(http_request.body, Some("toto\r\ntata".to_string()));
        assert_eq!(http_request.headers, Some(vec![("Content-Length".to_string(), "10".to_string())]))
    }

    #[test]
    fn request_try_from_ko() {
        let buffer = "POST rappel/1 HTTP/1.1\r\nContent-Length: 4\r\n\r\ntoto";
        
        let  request = HTTPRequest::try_from(buffer);
        
        assert!(request.is_ok());
    }


    #[test]
    fn extract_headers_ok() {
        let request = vec!["ressource".to_string(),"Content-Length: 1".to_string(),"Content-type: x and y".to_string(), "".to_string()];
        
        let  request = extract_headers(request);
        
        assert_eq!(request, vec![("Content-Length".to_string(), "1".to_string()),("Content-type".to_string(), "x and y".to_string())]);
    }
}

