use reqwest::blocking::{Client, ClientBuilder, RequestBuilder};
use reqwest::header::HeaderMap;
use libc::c_char;
use std::{ptr, ffi::CStr};

use crate::ffi::update_last_error;
use crate::errors::*;

#[no_mangle]
pub unsafe extern "C" fn new_client_builder() -> *mut ClientBuilder{
    println!("new_client_builder");
    Box::into_raw(Box::new(ClientBuilder::new()))
}

#[no_mangle]
pub unsafe extern "C" fn client_builder_default_headers(
        client_builder: *mut ClientBuilder,
        headermap :*mut HeaderMap,
)->*mut ClientBuilder{
    println!("default_headers");
    if client_builder.is_null() {
        update_last_error(Error::from("client buildert is null when default_headers"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.default_headers(*Box::from_raw(headermap));
    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub unsafe extern "C" fn client_builder_user_agent(client_builder: *mut ClientBuilder,value: *const c_char)
->*mut ClientBuilder{
    println!("user agent");
    if value.is_null(){
        update_last_error(Error::from("user agent is null"));
        return ptr::null_mut();
    }
    let r_value = CStr::from_ptr(value).to_str().unwrap();
    let result: ClientBuilder = Box::from_raw(client_builder).user_agent(r_value);
    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub unsafe extern "C" fn client_builder_destory(client_builder: *mut ClientBuilder){
    if !client_builder.is_null(){
        drop(Box::from_raw(client_builder));
    }
}

#[no_mangle]
pub unsafe extern "C" fn client_builder_build_client(client_builder: *mut ClientBuilder) ->*mut Client {
    if client_builder.is_null() {
        update_last_error(Error::from("client builder is null"));
        return ptr::null_mut();
    }
    println!("client builder build client");
    let r_client_builder = Box::from_raw(client_builder);
    match r_client_builder.build() {
        Ok(c) => Box::into_raw(Box::new(c)),
        Err(e) => {
            let err = Error::with_chain(e, "Unable to build client");
            update_last_error(err);
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn client_destory(client: *mut Client) {
    if !client.is_null(){
        drop(Box::from_raw(client));
    }
}

#[no_mangle]
pub unsafe extern "C" fn client_get(client: *mut Client,url: *const c_char) -> *mut RequestBuilder{
    print!("get");
    if client.is_null() || url.is_null(){
        update_last_error(Error::from("client is null"));
        return ptr::null_mut();
    }
    let r_value = CStr::from_ptr(url).to_str().unwrap();
    let r_client : &Client = &*client;
    let rb : reqwest::blocking::RequestBuilder = r_client.get(r_value);
    Box::into_raw(Box::new(rb))
}




