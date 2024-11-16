use crate::{HTTPResponse, ResponseBuilder, Verb};

use super::request_handler::RequestHandler;

#[derive(PartialEq, Eq,Debug)]
pub struct Route{
    pub verb  : Verb,
    pub route:  String,
    pub method : fn(&RequestHandler) -> HTTPResponse,
    pub need_security: bool
}

impl Route {
    pub fn new(verb: &Verb, route: &str, method: fn(&RequestHandler) -> HTTPResponse, need_security: bool) -> Route {
        Route {
            verb: *verb,
            route: route.to_string(), 
            method,
            need_security
        }
    }
}

impl Clone for Route {
    fn clone(&self) -> Self {
        Route { route: self.route.clone(), ..*self }
    }
}

impl Default for Route {
    fn default() -> Self {
        Self { verb: Verb::GET, route: "/".to_string(), method: default_method, need_security: false }
    }
}

fn default_method(_: &RequestHandler) -> HTTPResponse {
    ResponseBuilder::new(200, Some("default".to_string())).build()
}

pub type Routes = Vec<Route>;