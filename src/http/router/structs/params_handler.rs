use std::collections::HashMap;

use log::info;

use crate::{HTTPRequest, Route};


pub struct ParamsHandler {
    pub params: Params,
    pub body: Option<String>
}


pub type Params = HashMap<String, String>;

impl From<(&HTTPRequest, Route)> for ParamsHandler {
    fn from((request , ressource): (&HTTPRequest, Route)) -> Self {
        let positions =  ressource.route.split('/')
            .enumerate()
            .filter(|(_, val)| val.starts_with('{'))
            .collect::<HashMap<usize, &str>>();

        info!("Params Position: {:?}", positions);
        let params = request.start_line.resource().split('/')
            .enumerate()
            .filter(|(index, _)| positions.keys().collect::<Vec<&usize>>().contains(&index))
            .map(|(index, param)| (path_param_name_from_position(positions.clone(), index), param.to_string()))
            .collect::<HashMap<String, String>>();

        info!("Params: {:?}", params);
        ParamsHandler { params, body: request.body.clone() }
    }
}

fn path_param_name_from_position(params: HashMap<usize, &str>, index: usize) -> String {
    return params.get(&index)
        .and_then(|&param |param.strip_prefix('{'))
        .and_then(|param |param.strip_suffix('}'))
        .unwrap().to_string();     
}


#[cfg(test)]
mod tests {

    use crate::{HTTPResponse, ResponseBuilder, Verb};

    use super::*;

    fn route_mock(_: ParamsHandler) -> HTTPResponse {
        ResponseBuilder::new(200, None).build()
    } 



    #[test]
    fn request_try_from_ok() {
        let buffer = "POST rappel/1/att/57 HTTP/1.1\r\nContent-Length: 6\r\n\r\ntoto\r\nNN";
        let request = HTTPRequest::try_from(buffer).unwrap();
        let route = Route {
            verb: Verb::GET,
            route: "rappel/{id}/att/{att}".to_string(),
            method: route_mock,
            ..Default::default()
        };
        let result = ParamsHandler::try_from((&request, route));

        assert!(result.is_ok());
        let params = result.unwrap();

        assert_eq!(params.params.get(&"id".to_string()), Some(&"1".to_string()));
        assert_eq!(params.params.get(&"att".to_string()), Some(&"57".to_string())); 
        assert_eq!(params.body.unwrap(), "totoNN");
    }

}