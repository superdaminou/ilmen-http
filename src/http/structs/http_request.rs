use std::{usize, str::FromStr};
use std::str;



use log::info;

use crate::{http::errors::malformed::MalformedError, Verb};
#[derive(PartialEq, Debug, Clone)]

pub struct HTTPRequest {
    pub start_line: StartLine,
    pub header: Option<Headers>,
    pub body: Option<Body>
}

#[derive(PartialEq, Debug, Clone)]
pub struct StartLine{
    pub verb: Verb,
    pub ressource: String
}

type Headers = Vec<(String, String)>;
type Body = String;



impl TryFrom<[u8; 1024]> for HTTPRequest {
    type Error = MalformedError;
    fn try_from(value: [u8; 1024]) -> Result<Self, MalformedError> {
        return str::from_utf8(&value)
        .map_err(|error| 
            MalformedError::from(error))
        .map(|request| 
            HTTPRequest::try_from(request))?
    }
}

impl TryFrom<&str> for HTTPRequest {
    type Error = MalformedError;
    fn try_from(buffer: &str) -> Result<Self, MalformedError> {

        let parsed_request = parse(buffer);
        
        let start_line = get_start_line(parsed_request.clone())?;
        info!("Ressource: {}", start_line.ressource);
        
        let headers = extract_headers(parsed_request.clone());
        info!("Headers: {:?}", headers);

        let content_length = get_content_length(headers.clone())?;
        info!("Content-Length: {}", content_length.to_string());

        let body = extract_body(parsed_request, content_length);
        info!("Body: {}", body);

        return Ok (HTTPRequest { start_line: start_line, header: Some(headers), body: Some(body)});
    }
}


fn extract_body(request : Vec<String>, content_length: usize) -> Body {
    return request.iter()
            .skip_while(|&str| str.trim() != "")
            .fold("".to_owned(), |acc, e| acc + e + "\r\n")
            .to_string()
            .drain(0..content_length)
            .collect();
}

fn get_start_line(request : Vec<String>) -> Result<StartLine, MalformedError> {
    return request.iter()
        .next()
        .map(|start_line| StartLine::try_from((*start_line).clone()))
        .unwrap_or(Err(MalformedError::from("Need start line")))
}

fn parse(buffer : &str) -> Vec<String> {
    return buffer
        .trim_matches(char::from(0))
        .split("\r\n")
        .map(|str| str.to_string())
        .collect::<Vec<String>>();
}

impl TryFrom<Vec<String>> for StartLine {
    type Error = MalformedError;
    fn try_from(request: Vec<String>) -> Result<Self, Self::Error> {
        return request.iter()
            .next()
            .map(|start_line| StartLine::try_from((*start_line).clone()))
            .unwrap_or(Err(MalformedError::from("Need start line")));
    }
}

fn get_content_length(headers: Headers) -> Result<usize, MalformedError> {
    return headers
        .iter()
        .find(|(header, _)| header.starts_with("Content-Length:"))
        .map(|(_, value)| 
            (*value).parse::<usize>()
            .map_err(MalformedError::from))
        .transpose()
        .map(|length| length.unwrap_or(0));

}

fn extract_headers(request : Vec<String>) -> Headers {
    return request.iter()
        .skip(1)
        .take_while(|&str| str.trim() != "")
        .map(|header| {
            let mut line = header.split_ascii_whitespace();
            return match line.next() {
                None => None,
                Some(str)=> Some((str.to_string(), line.next().unwrap_or("").to_string())) 
            }
        })
        .filter(|line| line.is_some())
        .flatten()
        .collect::<Headers>();
}




impl TryFrom<String> for StartLine {
    type Error = MalformedError;
        fn try_from(value: String) -> Result<Self, MalformedError> {
            let binding = value
                .split(' ')
                .take(3).collect::<Vec<&str>>();
            let mut decomposed_start_line =  binding
                .iter();
            
            let verb = decomposed_start_line
                .next()
                .ok_or(MalformedError::from("Missing verb"))
                .map(|&verb| Verb::from_str(verb))??;


            let ressource = decomposed_start_line
                .next()
                .ok_or(MalformedError::from("No ressource"))
                .map(|&str| str.to_string())?;

            return Ok(StartLine { verb: verb, ressource: ressource });
}
    
}



// UNIT TEST
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;



    #[test]
    fn request_try_from_ok() {
        let buffer = "POST rappel/1 HTTP/1.1\r\nContent-Length: 4\r\n\r\ntoto";
        
        let  request = HTTPRequest::try_from(buffer);
        
        let http_request = request.unwrap();
        assert_eq!(http_request.start_line, StartLine{verb: Verb::POST, ressource: "rappel/1".to_string()});
        assert_eq!(http_request.body, Some("toto".to_string()));
        assert_eq!(http_request.header, Some(vec![("Content-Length:".to_string(), "4".to_string())]))
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
        let request = vec!["ressource".to_string(),"Content-Length: 1".to_string(),"Content-type: x".to_string(), "".to_string()];
        
        let  request = extract_headers(request);
        
        assert_eq!(request, vec![("Content-Length:".to_string(), "1".to_string()),("Content-type:".to_string(), "x".to_string())]);
    }

    #[test]
    fn get_content_length_ok() {
        let headers = vec![("Content-Length:".to_string(), "152".to_string()),("Content-type:".to_string(), "x".to_string())];

        let content_length = get_content_length(headers);
        
        assert_eq!(content_length, Ok(152));
    }

    #[test]
    fn get_content_length_ko_malformed() {
        let headers = vec![("Content-Length:".to_string(), "152a".to_string()),("Content-type:".to_string(), "x".to_string())];

        let content_length = get_content_length(headers);
        
        assert_eq!(content_length, Err(MalformedError::from("Expected a valid integer")));
    }

    #[test]
    fn get_content_length_ok_0() {
        let headers = vec![("Content-type:".to_string(), "x".to_string())];

        let content_length = get_content_length(headers);
        
        assert_eq!(content_length, Ok(0));
    }

    


}