use std::io::Read;
use reqwest::{ self, StatusCode };
use reqwest::header::HeaderMap;

use errors::*;

#[derive(Debug, Clone)]
pub struct Response {
    pub headers: HeaderMap,
    //pub body: Bytes,
    pub status: StatusCode,
}

impl Response {

//    #[tokio::main]
    pub(crate) fn from_reqwest(original: reqwest::Response) -> Result<Response> {
        let mut original = original.error_for_status()?;
        let headers = original.headers().clone();
        let status = original.status();
        //let body = original.bytes().await;

        Ok(Response {
//            status,body,headers
            status,headers
        })
    }
}