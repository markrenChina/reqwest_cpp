//use cookie::CookieJar;
use reqwest::blocking::{Response, RequestBuilder};
use libc::c_char;
use anyhow::Error;
use std::{ptr, ffi::CStr};

use crate::ffi::update_last_error;

#[no_mangle]
pub unsafe extern "C" fn request_builder_header(
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
pub unsafe extern "C" fn request_builder_send(
        request_builder : *mut RequestBuilder,
        )-> *mut Response{
    if request_builder.is_null(){
        return ptr::null_mut();
    }

    let r_request_builder = Box::from_raw(request_builder);
    let result = r_request_builder.send();
    match result {
        Ok(r) => Box::into_raw(Box::new(r)),
        Err(e) => {
            let err = Error::new(e).context("send request error");
            update_last_error(err);
            ptr::null_mut()
        }
    }
}