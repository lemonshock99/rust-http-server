



use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
// use std::str;
use crate::http::Result;
use crate::http::Method;
use crate::http::Response;
use crate::http::HttpStatus;

#[derive(Debug)]
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        // println!("{}",addr);
        Self { addr }
    }

    pub fn get_addr(&self) -> &str {
        &self.addr
    }

    // pub fn run(&self) -> Result<(), std::io::Error> {
    // pub fn run(&self) -> std::io::Result<()> {
    pub fn run(&self) -> Result<()> {
        println!("Listening on {}",self.addr);

        let listener = TcpListener::bind(&self.addr)?;

        for stream in listener.incoming() {
            let mut stream  = stream?;
            let mut buf: [u8; 1024] = [0; 1024];

            stream.read(&mut buf)?;

            // if let Ok(request) = str::from_utf8(&buf) {
            //     println!("{}", request);
            // }
            // let request = str::from_utf8(&buf)?;
            // println!("{}",request);

            let request = Request::try_from(&buf[..])?;
            println!("{:#?}", request);


            let response = match request.method() {
                Method::GET => match request.path().as_str(){
                    "/" => Response::new(HttpStatus::Ok,Some("home".to_string())),
                    "/hello" => Response::new(HttpStatus::Ok,Some("hello".to_string())),
                    _ => Response::new(HttpStatus::NotFound,None),
                },
                _ => Response::new(HttpStatus::NotFound,None),
            };
            response.send(&mut stream)?;
        }


        Ok(())
    }
}