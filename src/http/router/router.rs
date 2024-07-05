use std::panic::catch_unwind;

use crate::http::{structs::{HTTPRequest, HTTPResponse, Response, Verb}, errors::internal::InternalError};

use super::{Routes, ParamsHandler};

pub fn handle_request(request: HTTPRequest, handler : Routes) -> HTTPResponse {
    catch_unwind(||route(request, handler))
    .map_err(|_| InternalError::from("Internal Server Error"))
    .map(|response| HTTPResponse::from(response))
    .unwrap_or_else(|err| HTTPResponse::from(err))
}

fn route(request: HTTPRequest, handler : Routes) -> Response {
    match request.start_line.verb {
        Verb::OPTION => options(),
        _ => handler.iter()
                .filter(|route| route.verb == request.start_line.verb)
                .find(|&route| 
                    valid_against(request.start_line.ressource.clone(), route.route.clone()))
                .map_or(Response::from(404), |route| 
                    (route.method)(ParamsHandler::from((request, route.clone()))))
    }
}



fn options() -> Response {
    let headers = vec!["Access-Control-Allow-Methods: POST, GET, DELETE, PATCH, OPTIONS\r\n".to_string()];
    Response::from(headers)
}


fn valid_against(request: String,reference: String) -> bool {
    
    
    let splitted_reference = reference.split('/').collect::<Vec<&str>>();
    let request_iter = request.split('/').collect::<Vec<&str>>();

    let mut reference_iter = splitted_reference.iter();

    return (splitted_reference.len() == request_iter.len()) && 
    request_iter.iter()
        .fold(true, |acc, &request_part| acc && compare(request_part, reference_iter.next()));
}

fn compare(req_part: &str, ref_part: Option<&&str>)  -> bool {
    match ref_part {
        None => false,
        Some(&ref_part) => {
            return ref_part.starts_with('{') || req_part == ref_part;
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
        let result = compare(&incoming, Some(&reference));
        assert!(result);
    }

    #[test]
    fn compare_ressource_nok() {
        let reference: &str ="route";
        let incoming ="routes";
        let result = compare(&incoming, Some(&reference));
        assert!(!result);
    }

    #[test]
    fn compare_param_ok() {
        let reference ="{id}";
        let incoming ="2";
        let result = compare(&incoming, Some(&reference));
        assert!(result);
    }

    #[test]
    fn valid_against_ok() {
        assert!(valid_against("ressource/1/toto/2".to_string(), "ressource/{id}/toto/{dd}".to_string()));
    }

    #[test]
    fn valid_against_ko_param() {
        assert!(!valid_against("ressource/1/toto/2".to_string(), "ressource/id/toto/{dd}".to_string()));
    }

    #[test]
    fn valid_against_ko_ressource() {
        assert!(!valid_against("ressources/1/toto/2".to_string(), "ressource/{id}/toto/{dd}".to_string()));
    }

}