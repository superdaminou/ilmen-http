use std::usize;
use std::str;
use log::info;

use crate::http::errors::http_errors::HttpError;

use super::startline::StartLine;
#[derive(PartialEq, Debug, Clone)]

#[derive(Default)]
pub struct HTTPRequest {
    pub start_line: StartLine,
    pub headers: Option<Headers>,
    pub body: Option<Body>
}

type Headers = Vec<Header>;
type Header = (HeaderKey, HeaderValue);
type HeaderKey = String;
type HeaderValue = String;

type Body = String;
pub type Ressource = String;


impl From<&HTTPRequest> for Ressource {
    fn from(value: &HTTPRequest) -> Self {
        value.start_line.to_string()
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

impl TryFrom<&str> for HTTPRequest {
    type Error = HttpError;
    fn try_from(buffer: &str) -> Result<Self, HttpError> {
        let parsed_request = parse(buffer);
        
        let start_line = StartLine::try_from(parsed_request.clone())?;
        info!("Ressource: {}", start_line.to_string());

        let headers = extract_headers(parsed_request.clone());
        info!("Headers: {:?}", headers);
        
        let body = match get_header(&headers, "Content-Length") {
            Some(content_length_header) => {        
                let body_size = content_length_header.1.parse::<usize>().map_err(|e|HttpError::BadRequest(e.to_string()))?; 
                Some(extract_body(parsed_request, body_size))
                
            },
            None => None,
        };

        Ok (HTTPRequest { start_line, headers: Some(headers), body})
    }
}


fn extract_body(request : Vec<String>, content_length: usize) -> Body {
    return request.iter()
            .skip_while(|&str| str.trim() != "")
            .skip(1)
            .fold("".to_owned(), |acc, e| acc + e)
            .to_string()
            .drain(0..content_length)
            .collect();
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
            .find(|(header, _)| header.starts_with(key)).cloned()
    }
}

fn get_header(headers: &Headers, key: &str) -> Option<(String, String)> {
    headers
        .iter()
        .find(|(header, _)| header.starts_with(key)).cloned()
}

fn extract_headers(request : Vec<String>) -> Headers {
    request.iter()
        .skip(1)
        .take_while(|&str| str.trim() != "")
        .map(|header| header.split_once(':'))
        .map(|spliterator| spliterator.unwrap_or_default())
        .map(|(a, b)| (a.trim().to_owned(), b.trim().to_owned()))
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
        let buffer = "POST rappel/1 HTTP/1.1\r\nContent-Length: 4\r\n\r\ntoto";
        
        let  request = HTTPRequest::try_from(buffer);
        
        let http_request = request.unwrap();
        assert_eq!(http_request.start_line, StartLine::new(Verb::POST, "rappel/1".to_string()));
        assert_eq!(http_request.body, Some("toto".to_string()));
        assert_eq!(http_request.headers, Some(vec![("Content-Length".to_string(), "4".to_string())]))
    }

    #[test]
    fn request_try_from_ko() {
        let buffer = "POST rappel/1 HTTP/1.1\r\nContent-Length: 4\r\n\r\ntoto";
        
        let  request = HTTPRequest::try_from(buffer);
        
        assert!(request.is_ok());
    }

    #[test]
    fn extract_body_ok() {
        let request = vec!["POST rappel/1 HTTP/1.1".to_string()," ".to_string(),"toto".to_string(), "toto".to_string()];
        
        let  request = extract_body(request, 8);
        
        assert_eq!(request, "totototo");
    }

    #[test]
    fn extract_headers_ok() {
        let request = vec!["ressource".to_string(),"Content-Length: 1".to_string(),"Content-type: x and y".to_string(), "".to_string()];
        
        let  request = extract_headers(request);
        
        assert_eq!(request, vec![("Content-Length".to_string(), "1".to_string()),("Content-type".to_string(), "x and y".to_string())]);
    }

}