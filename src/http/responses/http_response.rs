use std::fmt;

use crate::http::{errors::http_errors::HttpError, header::{HeaderKey, HeaderValue, Headers}};

const PROTOCOL : &str= "HTTP/1.1";

pub struct HTTPResponse {
    code: i32,
    body: Option<String>,
    headers: Headers
}

impl HTTPResponse {
    fn new(code: i32, body: Option<&str>) -> HTTPResponse {
        let mut headers = vec![
            ("Access-Control-Allow-Origin".to_string(), "*".to_string()), 
            ("Content-Type".to_string(), "application/json".to_string())
        ];

        match body {
            Some(b) => {
                headers.push(("Content-Type".to_string(), "application/json".to_string()));
                headers.push(("Content-Length".to_string(), b.len().to_string()));
            },
            None => (),
        }

        HTTPResponse {
            code, 
            headers,  
            body: body.map(String::from)
        }
    }

    pub fn code(&self) -> i32 {
        self.code
    }    
}

impl Default for HTTPResponse {
    fn default() -> Self {
        Self { 
            code: 404,  
            body: Default::default(), 
            headers: Default::default() }
    }
}



fn construct_status_line(code : i32) -> String {
    format!("{} {} {}", PROTOCOL, code, message_from_code(code))
}

fn message_from_code(code : i32) -> String {
    match code {
        200 =>  String::from("OK"),
        500 => "INTERNAL".to_string(),
        404 => "NOT FOUND".to_string(),
        400 => "BAD REQUEST".to_string(),
        _  => "".to_string()
    }
}

impl fmt::Display for HTTPResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let headers= self.headers.iter().map(|(key, value)| key.to_string() + ": " + value).collect::<Vec<String>>().join("\r\n");
        let body = match &self.body {
            Some(body) => "\r\n".to_string() + body,
            None => "".to_string(),
        };
        write!(f,
            "{}\r\n{}\r\n\r\n{}",
            construct_status_line(self.code),
            headers,
            body
        )
    }
}



impl From<HttpError> for HTTPResponse {
    fn from(value: HttpError) -> Self {
        match value {
            HttpError::DefaultError => ResponseBuilder::new(0, None).build(),
            HttpError::NotFoundError(_) => ResponseBuilder::new(404, Some("Not found".to_string())).build(),
            HttpError::UnauthorizedError(_) => ResponseBuilder::new(401, None).build(),
            HttpError::BadRequest(_) => ResponseBuilder::new(400, None).build(),
        }
    }
}


pub type Code = i32;


#[derive(Default)]
pub struct ResponseBuilder {
    code: i32,
    body: Option<String>,
    headers: Headers
}


impl ResponseBuilder {
    pub fn new(code: i32, body: Option<String>) -> Self {
        ResponseBuilder {
            code,
            body,
            ..Default::default()
        }
    }

    pub fn content_type(mut self, content_type: String) -> Self {
        self.headers.push(("Content-Type".to_string(), content_type)); 
        self
    }

    pub fn build(&self) -> HTTPResponse{
        let headers = vec![];
        HTTPResponse {
            body: self.body.clone(),
            code: self.code,
            headers
        }   
    }

    pub fn put_header(mut self, key: HeaderKey, value: HeaderValue) -> ResponseBuilder {
        self.headers.push((key, value));
        self
    }
}
