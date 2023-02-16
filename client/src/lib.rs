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
mod client;

#[no_mangle]
pub extern "C" fn hello_world() {
    println!("Hello World!");
}
