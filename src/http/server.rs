use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use log::info;
use log::trace;

use crate::http::router::handle_request;
use crate::http::requests::HTTPRequest;
use crate::http::responses::HTTPResponse;
use crate::http::configuration::ThreadPool;

use super::router::Routes;
use super::configuration::Config;


#[derive(Default)]
pub struct HttpServer {
    configuration: Config, 
    handler : Routes
}

impl HttpServer {

    pub fn new(configuration: Config, handler : Routes) -> Self {
        HttpServer {
            configuration,
            handler
        }
    } 

   pub fn start(&self) {
        info!("Opening connection and listening");
        let config = self.configuration.clone();
        info!("Start listening on {}", config.adresse());
        let listener = TcpListener::bind(config.adresse()).unwrap();
        info!("Initializing thread pool : {}", 5);
        let pool = ThreadPool::new(5);
        let term = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).unwrap();
        while !term.load(Ordering::Relaxed) {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let routes = self.handler.clone();
                let config = config.clone();
    
                pool.execute(move || {
                    handle_connection(stream, routes, config);
                });
            }
        }
   } 
}


// PRIVATE
fn handle_connection(mut stream: TcpStream, handler : Routes, config: Config) {
    let mut buffer: Vec<u8> = vec![0; config.request_size()];
    stream.read(&mut buffer).unwrap();

    let response = 
    HTTPRequest::try_from(buffer)
        .map(|request |  handle_request(&request, handler, config))
        .unwrap_or_else(HTTPResponse::from);
        
            
    trace!("Response: {}", response.to_string());
    write(stream, response.to_string());
}

fn write(mut stream : TcpStream, response: String) {
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
