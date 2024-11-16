use std::collections::HashMap;

use crate::{http::HTTPRequest, Route};

pub struct RequestHandler {
    request: HTTPRequest,
    path_params: PathParams
}

type PathParams = HashMap<String, String>;
type QueryParams = HashMap<String, String>;

impl RequestHandler {
    pub fn query_params(&self) -> Option<QueryParams> {
        self.request.query_params.clone()
    } 

    pub fn path_params(&self) -> PathParams {
        self.path_params.clone()
    } 

    pub fn body(&self) -> Option<String> {
        self.request.body.clone()
    } 
}

impl From<(&HTTPRequest, &Route)> for RequestHandler {
    fn from(value: (&HTTPRequest, &Route)) -> Self {
        RequestHandler {
            request: value.0.clone(),
            path_params: extract_path_params(&value.0.resource, &value.1.route)
        }
    }
}

fn extract_path_params(request: &String, template: &String) -> PathParams {
    let split_request = request.split("/").collect::<Vec<&str>>();
    template.split("/").enumerate()
        .filter(|(_, key)| key.starts_with("{") && key.ends_with("}"))
        .map(|(index, key)| (key.to_string().drain(1..key.len()-1).collect(), split_request.get(index).unwrap().to_string()))
        .collect::<PathParams>()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_path_params_multiple_params() {
        let request = "base/3/another/tata";
        let template = "base/{id}/another/{name}";
        let mut expected = HashMap::new();
        expected.insert("id".to_string(), "3".to_string());
        expected.insert("name".to_string(), "tata".to_string());

        let result = extract_path_params(&request.to_string(),&template.to_string());
        assert_eq!(result,expected)
    }

}