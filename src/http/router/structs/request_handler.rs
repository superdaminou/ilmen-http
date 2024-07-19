use std::collections::HashMap;

use crate::{http::HTTPRequest, Route};

pub struct RequestHandler {
    request: HTTPRequest,
    path_params: PathParams
}

type PathParams = HashMap<String, String>;

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
    template.split("/").enumerate().into_iter()
        .filter(|(_, v)| v.starts_with("{"))
        .map(|(i, v)| (v.to_string().drain(1..v.len()-1).collect(), split_request.get(i).unwrap().to_string()))
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