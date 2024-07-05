use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use log::info;

use crate::http::router::handle_request;
use crate::http::structs::HTTPRequest;
use crate::http::structs::HTTPResponse;
use crate::http::structs::ThreadPool;

use super::router::Routes;
use super::structs::Config;

pub fn open_connection(configuration: Option<Config>, handler : Routes){
    info!("Opening connection and listening");
    let config = configuration.unwrap_or_default();
    info!("Start listening on {}", config.adresse());
    let listener = TcpListener::bind(config.adresse()).unwrap();
    info!("Initializing thread pool : {}", 5);
    let pool = ThreadPool::new(5);


    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let clone = handler.clone();

        pool.execute(move || {
            handle_connection(stream, clone);
        });
    }
}

// PRIVATE
fn handle_connection(mut stream: TcpStream, handler : Routes) {
    info!("Handling Connection");
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = 
    HTTPRequest::try_from(buffer)
        .map(|request |  handle_request(request, handler))
        .unwrap_or_else(|err| HTTPResponse::from(err));
        
            
    info!("{}", response.to_string());
    write(stream, response.to_string());
}

fn write(mut stream : TcpStream, response: String) {
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
