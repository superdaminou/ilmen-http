# Ilmen-http

DO NOT USE IN PRODUCTION

A small, home made, weird little http server.


With Default configuration and no-security
```
fn main() {
    let config=Config::new(8082, SecurityProtocol::Basic(auth_method));
    let server= HttpServer::new(config, routes());

    server.start();
}

fn auth_validate(couple :(String, String)) -> bool {
    return true;
}

routes = vec![
        Route {verb: Verb::GET, route: "/book/{id}".to_string(),method: get_book, need_security: true},
]

pub fn get_book(handler: ParamsHandler) -> Response {   
    return handler.params
            .get("id")
            .ok_or(TechnicalError::from("Missing Id"))
            .and_then(|id| id.parse::<i32>().unwrap())
            .and_then(|id| database::get_one_book(id))
            .and_then(|book| serde_json::to_string(&rappel).unwrap())
            .map(|rows| Response::from((200, rows.to_string())))
            .unwrap_or_else(Response::from);
}

```




