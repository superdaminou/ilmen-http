pub struct Config {
    port: Port,
    ip: Ip
}

type Port = i32;
type Ip = String;


impl Default for Config {
    fn default() -> Self {
        Self { port: 7878, ip: "127.0.0.1".to_string() }
    }
}

impl Config {
    pub fn adresse(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}