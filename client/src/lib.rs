extern crate chrono;
extern crate cookie;
#[macro_use]
extern crate error_chain;
extern crate fern;
extern crate libc;
#[macro_use]
extern crate log;
extern crate reqwest;

pub mod ffi;
mod errors;
mod request;
mod response;


use reqwest::Client;
pub use request::Request;
pub use response::Response;
use errors::*;

#[no_mangle]
pub extern "C" fn hello_world() {
    println!("Hello World!");
}

static mut CLIENT : Option<Client> = None;


//Send a 'Request'
pub fn send_request(req: &Request)
//-> Result<Response> {
{
    if &CLIENT == None {
        &CLIENT = Some(Client::new());
    }
    info!("Sending a GET request to {}", req.destination);
    if log_enabled!(::log::Level::Debug) {
        debug!("Sending {} Headers", req.headers.len());
        for header in req.headers.iter() {
            //debug!("\t{}: {}",header.0.to_string(),header.1.as_str());
        }
        for cookie in req.cookies.iter() {
            debug!("\t{} = {}", cookie.name(), cookie.value());
        }

        trace!("{:#?}",req);
    }

    &CLIENT
    .Send(req)
    .chain_err(|| "The native TLS backend couldn't be initialized")
    ;

    //Ok(Response::from_reqwest(req.to_reqwest()))

//    client
//        .execute(req.to_reqwest())
//    .
//        .await
//        .is_err_and(|| "The request failed")
//        .then(|r| Response::from_reqwest(r))
//        .chain_err(|| "The request failed")
}