use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::{TryFrom, TryInto};

pub struct Server {
    addr: String,    
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self,mut handler:impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", self.addr);

        loop {

            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Accepted connection from : {} ", addr.to_string());
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => { 
                            println!("Received a request: {}", String::from_utf8_lossy(&mut buffer));
                            let response = match Request::try_from(&buffer[..]){
                                Ok(request) => handler.handle_request(&request),
                                Err(e) =>  handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e)
                            }
                        },
                        Err(e) => println!("Error: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }    
        }
    }
}