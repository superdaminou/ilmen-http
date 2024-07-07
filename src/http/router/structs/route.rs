use crate::{HTTPResponse, ResponseBuilder, Verb};

use super::ParamsHandler;

#[derive(PartialEq, Eq,Debug)]
pub struct Route{
    pub verb  : Verb,
    pub route:  String,
    pub method : fn(ParamsHandler) -> HTTPResponse,
    pub need_security: bool
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

fn default_method(_: ParamsHandler) -> HTTPResponse {
    ResponseBuilder::new(200, Some("default".to_string())).build()
}

pub type Routes = Vec<Route>;