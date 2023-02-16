use reqwest::blocking::{Client, ClientBuilder, RequestBuilder};
use libc::c_char;
use std::{ptr, ffi::CStr};

#[no_mangle]
pub unsafe extern "C" fn newClientBuilder() -> *mut ClientBuilder{
    Box::into_raw(Box::new(ClientBuilder::new()))
}

#[no_mangle]
pub unsafe extern "C" fn buildClient(client_builder: *mut ClientBuilder) ->*mut Client {
    if client_builder.is_null() {
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    println!("client_builder = {:#?}", r_client_builder);
    match r_client_builder.build() {
        Ok(c) => Box::into_raw(Box::new(c)),
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn client_destory(client: *mut Client) {
    if !client.is_null(){
        drop(Box::from_raw(client));
    }
}

#[no_mangle]
pub unsafe extern "C" fn user_agent(client_builder: *mut ClientBuilder,value: *const c_char)
->*mut ClientBuilder{
    if value.is_null(){
        return ptr::null_mut();
    }
    let r_value = CStr::from_ptr(value).to_str().unwrap();
    let result: ClientBuilder = (*Box::from_raw(client_builder)).user_agent(r_value);
    Box::into_raw(Box::new(result))
}

//#[no_mangle]
//pub unsafe extern "C" fn default_headers(clientBuilder: *mut ClientBuilder,value: *const c_char)
//->* mut ClientBuilder{
//
//}

#[no_mangle]
pub unsafe extern "C" fn get(client: *mut Client,url: *const c_char) -> *mut RequestBuilder{
    if client.is_null() || url.is_null(){
        return ptr::null_mut();
    }
    let r_value = CStr::from_ptr(url).to_str().unwrap();
    let r_client : &Client = &*client;
    println!("client = {:#?}", r_client);
    let rb : reqwest::blocking::RequestBuilder = r_client.get(r_value);
    Box::into_raw(Box::new(rb))
}

// send直接用，测试过了
//#[no_mangle]
//pub unsafe extern "C" fn get_test(client: *mut Client,url: *const c_char) -> *mut Response{
//    if client.is_null() || url.is_null(){
//        return ptr::null_mut();
//    }
//    let r_value = CStr::from_ptr(url).to_str().unwrap();
//    let r_client : &Client = &*client;
//    let rep = r_client.get(r_value).send();
//    match rep {
//        Ok(r) => Box::into_raw(Box::new(r)),
//        Err(_) => ptr::null_mut()
//    }
//}

