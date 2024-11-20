



use std::net::TcpListener;
use std::io::Read;
// use std::str;

use crate::http::Result;

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
        }


        Ok(())
    }
}