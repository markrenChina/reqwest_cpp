//use cookie::CookieJar;
use reqwest::blocking::{Response, RequestBuilder};
use libc::c_char;
use std::{ptr, ffi::CStr};


#[no_mangle]
pub unsafe extern "C" fn header(
        request_builder : *mut RequestBuilder,
        key: *const c_char,
        value: *const c_char,
)-> *mut RequestBuilder{
    if request_builder.is_null() || key.is_null() || value.is_null() {
        return ptr::null_mut();
    }
    let r_key = CStr::from_ptr(key).to_str().unwrap();
    let r_value = CStr::from_ptr(value).to_str().unwrap();
    let r_request_builder = Box::from_raw(request_builder);
    let res = r_request_builder.header(r_key, r_value);
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub unsafe extern "C" fn send_request(
        request_builder : *mut RequestBuilder,
        )-> *mut Response{
    if request_builder.is_null(){
        return ptr::null_mut();
    }

    let r_request_builder = Box::from_raw(request_builder);
    let result = r_request_builder.send();
    println!("after send");
    match result {
        Ok(r) => Box::into_raw(Box::new(r)),
        Err(_) => ptr::null_mut()
    }
}