use crate::http::Method;
use crate::http::QueryString;
use std::convert::TryFrom;
use crate::http::Error;
use crate::http::Result;

pub struct Request {
    method: Method,
    path: String,
    query_string: Option<QueryString>, // Option is return None and Some, in this case I "query Strinf" is can exist and not exist. 
}

impl TryFrom<&[u8]> for Request{
    type Error = Error;

    fn try_from(buf: &[u8]) -> Result<Self>{
        Ok(Self{
            method: Method::GET,
            path: "/".to_string(),
            query_string: None,
        })
    }
}
// query string is syntax pattern like this ?abc=xyz&def=zxc 