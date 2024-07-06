use std::fmt;

use crate::http::errors::http_errors::HttpError;

const PROTOCOL : &str= "HTTP/1.1";

pub struct HTTPResponse {
    code: i32,
    acces_control: String,
    content_type : String,
    body: String,
    headers: Vec<String>
}

impl HTTPResponse {
    fn new(code: i32,headers: Vec<String>,  body: Option<&str>) -> HTTPResponse {
        HTTPResponse {
            code, 
            headers,  
            body: String::from(body.unwrap_or("")), 
            acces_control: "*".to_string(), 
            content_type: "application/json".to_string()
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
            acces_control: "*".to_string(),
            content_type:  "application/json".to_string(), 
            body: Default::default(), 
            headers: Default::default() }
    }
}

impl From<Response> for HTTPResponse {
    fn from(response: Response) -> Self {
        HTTPResponse::new(response.code,Vec::new() ,response.body.as_deref())
    }
}


impl From<Vec<String>> for HTTPResponse {
    fn from(headers: Vec<String>) -> Self {
        HTTPResponse::new(200, headers, None)
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
        write!(f,
            "{}\r\n{}Access-Control-Allow-Origin: {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            construct_status_line(self.code),
            self.headers.join("\r\n"),
            self.acces_control,
            self.content_type,
            self.body.len(),
            self.body
        )
    }
}



pub struct Response {
    pub code: i32,
    pub headers: Vec<String>,
    pub body: Option<String>
}


impl Response {
    fn new(code: i32,headers: Vec<String>,  body: Option<String>) -> Response {
        Response {code, headers, body}
    }    
}


impl From<i32> for Response {
    fn from(code: i32) -> Self {
        Response::new(code, Vec::new(), None)
    }
}


impl From<(i32, &str)> for Response {
    fn from(code: (i32, &str)) -> Self {
        Response::new(code.0, Vec::new(),Some(code.1.to_string()))
    }
}

impl From<Vec<String>> for Response {
    fn from(headers: Vec<String>) -> Self {
        Response::new(200, headers, None)
    }
}

impl From<(i32, String)> for Response {
    fn from(code: (i32, String)) -> Self {
        Response::new(code.0, Vec::new(),Some(code.1))
    }
}

impl From<&HTTPResponse> for Code {
    fn from(value: &HTTPResponse) -> Code {
        value.code
    }
}

impl From<HttpError> for Response {
    fn from(value: HttpError) -> Self {
        match value {
            HttpError::DefaultError => Response::from(0),
            HttpError::NotFoundError(_) => Response::from(404),
            HttpError::UnauthorizedError(_) => Response::from(401),
            HttpError::BadRequest(_) => Response::from(400),
        }
    }
}

impl From<HttpError> for HTTPResponse {
    fn from(value: HttpError) -> Self {
        match value {
            HttpError::DefaultError => HTTPResponse::from(Response::from(0)),
            HttpError::NotFoundError(_) => HTTPResponse::from(Response::from(404)),
            HttpError::UnauthorizedError(_) => HTTPResponse::from(Response::from(401)),
            HttpError::BadRequest(_) => HTTPResponse::from(Response::from(400)),
        }
    }
}


pub type Code = i32;