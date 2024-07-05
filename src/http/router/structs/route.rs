use crate::http::structs::{Response, Verb};

use super::ParamsHandler;

#[derive(PartialEq, Eq)]
pub struct Route{
    pub verb  : Verb,
    pub route:  String,
    pub  method : fn(ParamsHandler) -> Response
}

impl Clone for Route {
    fn clone(&self) -> Self {
        Route { route: self.route.clone(), ..*self }
    }
}


pub type Routes = Vec<Route>;