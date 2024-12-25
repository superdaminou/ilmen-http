use crate::http::security::service::SecurityProtocol;


#[derive(Clone)]
pub struct Config {
    port: Port,
    ip: Ip,
    security: SecurityProtocol,
    request_size: usize
}


type Port = i32;
type Ip = String;

impl Default for Config {
    fn default() -> Self {
        Self { 
            port: 7878, 
            ip: "0.0.0.0".to_string(),
            security: SecurityProtocol::None,
            request_size: 10485760
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

    pub fn request_size(&self) -> usize {
        self.request_size
    }

    pub fn initialize() -> Config {
        Config::default()
    }

    pub fn with_request_size(&mut self, size: &usize) -> &mut Self {
        self.request_size = *size;
        self
    }

    pub fn with_security(&mut self, security: &SecurityProtocol) -> &mut Self {
        self.security = security.clone();
        self
    }

    pub fn with_adresse(&mut self, ip: &str, port: &Port) ->  &mut Self {
        self.ip = ip.to_string();
        self.port = *port;
        self
    }
}
