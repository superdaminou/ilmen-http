use anyhow::Context;
use base64::{engine::general_purpose::URL_SAFE, Engine};
use std::str;
use crate::{http::HttpError, HTTPRequest, Route};

pub fn apply_security(request: &HTTPRequest, route: Route, security: SecurityProtocol) -> Result<Route, HttpError> {
    match security {
        SecurityProtocol::None => Ok(()),
        SecurityProtocol::Basic(validate_methode) => base_auth(request, validate_methode),
    }.map(|_| route)
}

fn base_auth(request: &HTTPRequest, validate_methode: AuthMethod) -> Result<(), HttpError>{
    let header_auth_value : Vec<String> = request.get_header("Authorization")
        .ok_or(HttpError::UnauthorizedError("Missing header".to_string()))?
        .1
        .split(' ')
        .map(str::to_owned)
        .collect();

    let is_basic_protocol = header_auth_value.first()
        .ok_or(HttpError::UnauthorizedError("No protocol specified".to_string()))
        .map(|protocol| protocol.eq(&"Basic"))?;

    if !is_basic_protocol {
        return Err(HttpError::UnauthorizedError("Wrong Protocol".to_string()));
    }

    header_auth_value
            .get(1)
            .context("No user password provided")
            .and_then(decode_base64_auth)
            .map_err(|e| HttpError::UnauthorizedError(e.to_string()))
            .map(validate_methode)
            .and_then(|is_valid_creds| {
                match is_valid_creds {
                    true => Ok(()),
                    false => Err(HttpError::UnauthorizedError("Unauthorized".to_string())),
                }
            })
}

type Username = String;
type Password = String;
type AuthMethod = fn((Username, Password)) -> bool;

fn decode_base64_auth(b64_value: &String) -> anyhow::Result<(String, String)>{
    URL_SAFE.decode(b64_value)
            .context("Authentication parameter is not base64 encrypted".to_string())
            .and_then(|vect| String::from_utf8(vect).context("Could not parse from utf8"))
            .map(|decoded | {
                let splitter = decoded.split_once(':').unwrap_or(("", ""));
                (splitter.0.to_string(), splitter.1.to_string())
            })
}

#[derive(Clone)]
pub enum SecurityProtocol {
    None,
    Basic(AuthMethod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_basic_security_with_good_creds() {
        let buffer = "GET rappel/1 HTTP/1.1\r\nAuthorization: Basic dG90bzp0YXRh\r\n\r\ntoto";
        
        let  request = HTTPRequest::try_from(buffer).unwrap();
        let validate : AuthMethod= |_| true;

        let result = apply_security(&request, Route::default(), SecurityProtocol::Basic(validate)).unwrap();
        assert_eq!(result, Route::default())
    }

    #[test]
    fn apply_basic_security_with_bad_creds() {
        let buffer = "GET rappel/1 HTTP/1.1\r\nAuthorization: Basic dG90bzp0YXR1YWE=\r\n\r\ntoto";
        
        let  request = HTTPRequest::try_from(buffer).unwrap();

        let validate : AuthMethod = |_| false;

        let result = apply_security(&request, Route::default(), SecurityProtocol::Basic(validate)).unwrap_err();
        assert_eq!(result, HttpError::UnauthorizedError("Unauthorized".to_string()))
    }

    #[test]
    fn apply_basic_security_with_bad_protocol() {
        let buffer = "GET rappel/1 HTTP/1.1\r\nAuthorization: Basics dG90bzp0YXR1YWE=\r\n\r\ntoto";
        
        let  request = HTTPRequest::try_from(buffer).unwrap();

        let validate : AuthMethod = |_| false;

        let result = apply_security(&request, Route::default(), SecurityProtocol::Basic(validate)).unwrap_err();
        assert_eq!(result, HttpError::UnauthorizedError("Wrong Protocol".to_string()))
    }
}