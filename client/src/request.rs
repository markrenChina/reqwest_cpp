use cookie::CookieJar;
use reqwest::blocking::{RequestBuilder, Response};
use libc::c_char;
use std::{ptr, ffi::CStr};

#[no_mangle]
pub unsafe extern "C" fn header(
        requestBuilder : *mut RequestBuilder,
        key: *const c_char,
        value: *const c_char,
)-> *mut RequestBuilder{
    if requestBuilder.is_null() || key.is_null() || value.is_null() {
        return ptr::null_mut();
    }
    let r_key = CStr::from_ptr(key).to_str().unwrap();
    let r_value = CStr::from_ptr(value).to_str().unwrap();
    Box::into_raw(Box::new(Box::from_raw(requestBuilder).header(r_key, r_value)))
}

#[no_mangle]
pub unsafe extern "C" fn send(
        requestBuilder : *mut RequestBuilder,
        )-> *mut Response{
    if requestBuilder.is_null(){
        return ptr::null_mut();
    }
    let result = Box::from_raw(requestBuilder).send();
    match result {
        Ok(rep) => Box::into_raw(Box::new(rep)),
        Err(_) => ptr::null_mut()
    }
}