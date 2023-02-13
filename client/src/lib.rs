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

//Send a 'Request'
pub fn send_request(req: &Request) -> Result<Response> {
    info!("Sending a GET request to {}", req.destination);
    if log_enabled!(::log::LogLevel::Debug) {
        debug!("Sending {} Headers", req.headers.len());
        for header in req.headers.iter() {
            debug!("\t{}: {}", header.name(),header.value_string());
        }
        for cookie in req.cookies.iter() {
            debug!("\t{} = {}", cookie.name(), cookie.value());
        }

        trace!("{:#?}",req);
    }

    let client = Client::builder()
        .build()
        .chain_err(|| "The native TLS backend couldn't be initialized")?;

    client
        .execute(req.to_reqwest())
        .chain_err(|| "The request failed")
        .and_then(|r| Response::from_reqwest(r))
}