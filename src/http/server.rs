
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
}