use crate::http::method;
// use crate::http::request;
use crate::http::Method;
use crate::http::QueryString;
use std::convert::TryFrom;
use crate::http::Error;
use crate::http::Result;
use std::str;


#[derive(std::fmt::Debug)]
pub struct Request {
    method: Method,
    path: String,
    query_string: Option<QueryString>, // Option is return None and Some, in this case I "query Strinf" is can exist and not exist. 
}

impl Request {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

}

impl TryFrom<&[u8]> for Request{
    type Error = Error;

    fn try_from(buf: &[u8]) -> Result<Self>{

        let request = str::from_utf8(&buf)?;
        println!("{}",request);

        // GET /hello?name=bond&age=18 HTTP/1.1
        // GET /hello HTTP/1.1

        let mut request = request.split_whitespace();
        let method = request.next().ok_or(Error::InvalidRequest)?;
        let mut path = request.next().ok_or(Error::InvalidRequest)?;
        let protocol = request.next().ok_or(Error::InvalidRequest)?;

        if protocol != "HTTP/1.1"{
            return  Err(Error::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let query_string = match path.find('?'){
            Some(i) => {
                let qs = &path[i+1..];
                path = &path[..i];
                Some(QueryString::from(qs))
            }
            None => None

        };

        Ok(Self{
            method: Method::GET,
            path: path.to_string(),
            query_string: query_string,
        })
    }
}
// query string is syntax pattern like this ?abc=xyz&def=zxc 