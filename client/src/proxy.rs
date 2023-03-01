use libc::c_char;
use reqwest::Proxy;
use anyhow::{anyhow, Error};
use std::{ptr, ffi::CStr};

use crate::ffi::*;

/// Proxy all HTTP traffic to the passed URL.
#[no_mangle]
pub unsafe extern "C" fn proxy_reqwest_http(proxy_scheme: *const c_char) -> *mut Proxy{
    if proxy_scheme.is_null() {
        update_last_error(anyhow!("proxy_scheme is null"));
        return ptr::null_mut()
    }
    let result = reqwest::Proxy::http(
            CStr::from_ptr(proxy_scheme).to_str().unwrap()
    );
    match result {
        Ok(p) => Box::into_raw(Box::new(p)),
        Err(e) => {
            update_last_error(Error::new(e).context("prox fail"));
            return ptr::null_mut();
        }
    }
}

/// Proxy all HTTPS traffic to the passed URL.
#[no_mangle]
pub unsafe extern "C" fn proxy_reqwest_https(proxy_scheme: *const c_char) -> *mut Proxy{
    if proxy_scheme.is_null() {
        update_last_error(anyhow!("proxy_scheme is null"));
        return ptr::null_mut()
    }
    let result = reqwest::Proxy::https(
            CStr::from_ptr(proxy_scheme).to_str().unwrap()
    );
    match result {
        Ok(p) => Box::into_raw(Box::new(p)),
        Err(e) => {
            update_last_error(Error::new(e).context("prox fail"));
            return ptr::null_mut();
        }
    }
}


/// Proxy **all** traffic to the passed URL.
#[no_mangle]
pub unsafe extern "C" fn proxy_reqwest_all(proxy_scheme: *const c_char) -> *mut Proxy{
    if proxy_scheme.is_null() {
        update_last_error(anyhow!("proxy_scheme is null"));
        return ptr::null_mut()
    }
    let result = reqwest::Proxy::all(
            CStr::from_ptr(proxy_scheme).to_str().unwrap()
            );
    match result {
        Ok(p) => Box::into_raw(Box::new(p)),
        Err(e) => {
            update_last_error(Error::new(e).context("prox fail"));
            return ptr::null_mut();
        }
    }
}

//TODO custom

#[no_mangle]
pub unsafe extern "C" fn proxy_reqwest_destory(p :*mut Proxy){
    if p.is_null() {
        return ;
    }
    drop(Box::from_raw(p))
}