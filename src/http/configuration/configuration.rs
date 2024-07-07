use crate::http::security::service::SecurityProtocol;


#[derive(Clone)]
pub struct Config {
    port: Port,
    ip: Ip,
    security: SecurityProtocol
}


type Port = i32;
type Ip = String;

impl Default for Config {
    fn default() -> Self {
        Self { 
            port: 7878, 
            ip: "127.0.0.1".to_string(),
            security: SecurityProtocol::None
        }
    }
}

impl Config {
    pub fn adresse(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn security(&self) -> SecurityProtocol {
        self.security.clone()
    }

    pub fn new(port: Port, security: SecurityProtocol) -> Config {
        Config {
            port,
            security,
            ..Default::default()
        }
    }
}
