use std::panic::catch_unwind;

use log::info;

use crate::{http::{ errors::{http_errors::HttpError, InternalError},  security::service::{apply_security, SecurityProtocol}}, Config, HTTPRequest, HTTPResponse, ResponseBuilder, Route, Verb};

use super::{ structs::RequestHandler, Routes};

pub fn handle_request(request: &HTTPRequest, handler : Routes, config: Config) -> HTTPResponse {
    let response = catch_unwind(||route(request, handler, config.security()))
        .map_err(|_| InternalError::from("Internal Server Error"))
        .map(HTTPResponse::from)
        .unwrap_or_else(HTTPResponse::from);

    access_log(request, &response);
    
    response
}

fn access_log(request: &HTTPRequest, response: &HTTPResponse) {
    info!("{} {} {}", request.verb, request.resource, response.code())
}

fn route(request: &HTTPRequest, handler : Routes, security: SecurityProtocol) -> HTTPResponse {
    match request.verb {
        Verb::OPTION => options(),
        _ => {
            find_route(request, &handler)
                .and_then(|route| apply_security(request, route, security))
                .map(|route| execute(request, route))
                .unwrap_or_else(HTTPResponse::from)
            }
    }
}

fn execute(request: &HTTPRequest, route: Route ) -> HTTPResponse {
    let handler = RequestHandler::from((request, &route));
    (route.method)(&handler)   
}


fn find_route(request: &HTTPRequest, handler : &Routes) -> Result<Route, HttpError> {
     handler.iter()
                .filter(|route| route.verb == request.verb)
                .find(|&route| 
                    valid_against(request.resource.clone(), route.route.clone()))
                .map(|r| r.to_owned())
                .ok_or(HttpError::NotFoundError("Coult not find ressource".to_string()))
}

fn options() -> HTTPResponse {
        ResponseBuilder::new(200, None)
            .put_header("Access-Control-Allow-Methods".to_string(), "POST, GET, DELETE, PATCH, OPTIONS".to_string())
            .build()
}

fn valid_against(request: String,reference: String) -> bool {    
    let splitted_reference = reference.split('/').collect::<Vec<&str>>();
    let request_iter = request.split('/').collect::<Vec<&str>>();

    let mut reference_iter = splitted_reference.iter();

    return (splitted_reference.len() == request_iter.len()) && 
    request_iter.iter()
        .all(|request_part| compare(request_part, reference_iter.next()));
}

fn compare(req_part: &str, ref_part: Option<&&str>)  -> bool {
    match ref_part {
        None => false,
        Some(&ref_part) => {
            ref_part.starts_with('{') || req_part == ref_part
        }
    }
}


// UNIT TEST
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_ressource_ok() {
        let reference ="route";
        let incoming ="route";
        let result = compare(incoming, Some(&reference));
        assert!(result);
    }

    #[test]
    fn compare_ressource_nok() {
        let reference: &str ="route";
        let incoming ="routes";
        let result = compare(incoming, Some(&reference));
        assert!(!result);
    }

    #[test]
    fn compare_param_ok() {
        let reference ="{id}";
        let incoming ="2";
        let result = compare(incoming, Some(&reference));
        assert!(result);
    }

    #[test]
    fn test_valid_against_combinations() {
        let combinations :Vec<(Combination, ExpectedResult)> = vec![
            (Combination::new("ressource/1/toto/2", "ressource/id/toto/{dd}"), false),
            (Combination::new("ressource/1/toto/2", "ressource/{id}/toto/{dd}"), true),
            (Combination::new("ressource/1/toto/2?test=12", "ressource/{id}/toto/{dd}"), true)
            ];
            
        combinations.iter().for_each(|combinaisons| assert_eq!(valid_against(combinaisons.0.input.clone(), combinaisons.0.template.clone()), combinaisons.1));

    }

    struct Combination{input: String,template:  String}
    type ExpectedResult = bool;
    impl Combination {
        pub fn new(input: &str, template: &str) -> Combination {
            Combination{input: input.to_string(),template: template.to_string()}
        }
    }


    #[test]
    fn handle_request_without_security_should_return_200() {

        let request = HTTPRequest::default();
        let routes = vec![Route::default()];
        let config = Config::default();

        let responses_code = handle_request(&request, routes, config).to_string().split("\r\n").map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(responses_code.first().unwrap(), &"HTTP/1.1 200 OK");
    }
}