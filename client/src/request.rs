//use cookie::CookieJar;
use reqwest::blocking::{Request, Response, RequestBuilder};
use reqwest::header::HeaderMap;
use libc::c_char;
use anyhow::{Error, anyhow};
use std::{ptr, slice, time::Duration};

use crate::ffi::*;

/// Add a `Header` to this Request.
#[no_mangle]
pub unsafe extern "C" fn request_builder_header(
        request_builder : *mut RequestBuilder,
        key: *const c_char,
        value: *const c_char,
)-> *mut RequestBuilder{
    if request_builder.is_null(){
        update_last_error(anyhow!("request_builder is null when use header"));
        return ptr::null_mut();
    }
    let r_key = match to_rust_str(key, "parse key error"){
        Some(v) => v,
        None => {return ptr::null_mut(); }
    };
    let r_value = match to_rust_str(value, "parse value error"){
        Some(v) => v,
        None => {return ptr::null_mut(); }
    };
    let r_request_builder = Box::from_raw(request_builder);
    let res = r_request_builder.header(r_key, r_value);
    Box::into_raw(Box::new(res))
}

/// Add a `Header` to this Request.
#[no_mangle]
pub unsafe extern "C" fn request_builder_headers(
        request_builder : *mut RequestBuilder,
        headers: *mut HeaderMap,
        )-> *mut RequestBuilder{
    if request_builder.is_null() || headers.is_null(){
        update_last_error(anyhow!("request_builder or heanders is null when use headers"));
        return ptr::null_mut();
    }
    let r_request_builder = Box::from_raw(request_builder);
    let res = r_request_builder.headers(*Box::from_raw(headers));
    Box::into_raw(Box::new(res))
}

/// Enable HTTP basic authentication.
#[no_mangle]
pub unsafe extern "C" fn request_builder_basic_auth(
        request_builder : *mut RequestBuilder,
        username: *const c_char,
        password: *const c_char,
        )-> *mut RequestBuilder{
    if request_builder.is_null() {
        update_last_error(anyhow!("request_builder is null when use basic_auth"));
        return ptr::null_mut();
    }
    let r_usename = match to_rust_str(username, "pasre username error"){
        Some(v) => v,
        None => { return ptr::null_mut(); }
    };
    let r_password = to_rust_str(password, "pasre username error");
    let r_request_builder = Box::from_raw(request_builder);
    let res = r_request_builder.basic_auth(r_usename,r_password);
    Box::into_raw(Box::new(res))
}

/// Enable HTTP bearer authentication.
#[no_mangle]
pub unsafe extern "C" fn request_builder_bearer_auth(
        request_builder : *mut RequestBuilder,
        token: *const c_char,
        )-> *mut RequestBuilder{
    if request_builder.is_null() {
        update_last_error(anyhow!("request_builder is null when use bearer_auth"));
        return ptr::null_mut();
    }
    let r_token = match to_rust_str(token, "pasre token error"){
        Some(v) => v,
        None => { return ptr::null_mut(); }
    };
    let r_request_builder = Box::from_raw(request_builder);
    let res = r_request_builder.bearer_auth(r_token);
    Box::into_raw(Box::new(res))
}

/// Set the request body from u8 array.
#[no_mangle]
pub unsafe extern "C" fn request_builder_body_bytes(
        request_builder : *mut RequestBuilder,
        bytes: *const u8,
        size: usize,
        )-> *mut RequestBuilder{
    if request_builder.is_null() {
        update_last_error(anyhow!("request_builder is null when use body"));
        return ptr::null_mut();
    }
    let r_request_builder = Box::from_raw(request_builder);
    let r_bytes = slice::from_raw_parts(bytes, size);
    let res = r_request_builder.body(r_bytes);
    Box::into_raw(Box::new(res))
}

/// Set the request body from UTF-8 text.
#[no_mangle]
pub unsafe extern "C" fn request_builder_body_string(
        request_builder : *mut RequestBuilder,
        str: *const c_char,
        )-> *mut RequestBuilder{
    if request_builder.is_null() {
        update_last_error(anyhow!("request_builder is null when use body"));
        return ptr::null_mut();
    }
    let r_request_builder = Box::from_raw(request_builder);
    let r_str = match to_rust_str(str, "parse body string error") {
        Some(v) => v,
        None => { return ptr::null_mut(); }
    };
    let res = r_request_builder.body(r_str);
    Box::into_raw(Box::new(res))
}

/// Set the request body from file.
#[no_mangle]
pub unsafe extern "C" fn request_builder_body_file(
        request_builder : *mut RequestBuilder,
        file_path: *const c_char,
        )-> *mut RequestBuilder{
    if request_builder.is_null() {
        update_last_error(anyhow!("request_builder is null when use body"));
        return ptr::null_mut();
    }
    let r_request_builder = Box::from_raw(request_builder);
    let r_file_path = match to_rust_str(file_path, "parse body string error") {
        Some(v) => v,
        None => { return ptr::null_mut(); }
    };
    let file = match std::fs::File::open(r_file_path){
        Ok(f) => f,
        Err(e) => {
            update_last_error(Error::new(e));
            return ptr::null_mut();
        }
    };
    let res = r_request_builder.body(file);
    Box::into_raw(Box::new(res))
}

/// Enables a request timeout.
///
/// The timeout is applied from when the request starts connecting until the
/// response body has finished. It affects only this request and overrides
/// the timeout configured using `ClientBuilder::timeout()`.
#[no_mangle]
pub unsafe extern "C" fn request_builder_timeout(
        request_builder : *mut RequestBuilder,
        millisecond: u64,
        )-> *mut RequestBuilder{
    if request_builder.is_null() {
        update_last_error(anyhow!("request_builder is null when use timeout"));
        return ptr::null_mut();
    }
    let r_request_builder = Box::from_raw(request_builder);
    let res = r_request_builder.timeout(Duration::from_millis(millisecond));
    Box::into_raw(Box::new(res))
}

/// Modify the query string of the URL.
///
/// Modifies the URL of this request, adding the parameters provided.
/// This method appends and does not overwrite. This means that it can
/// be called multiple times and that existing query parameters are not
/// overwritten if the same key is used. The key will simply show up
/// twice in the query string.
/// Calling `.query(&[("foo", "a"), ("foo", "b")])` gives `"foo=a&foo=b"`.
#[no_mangle]
pub unsafe extern "C" fn request_builder_query(
        request_builder : *mut RequestBuilder,
        querys: *const Pair,
        len: usize,
        )-> *mut RequestBuilder{
    if request_builder.is_null() {
        update_last_error(anyhow!("request_builder is null when use query"));
        return ptr::null_mut();
    }
    let r_request_builder = Box::from_raw(request_builder);
    let c_querys : &[Pair]= slice::from_raw_parts(querys, len);
    let r_querys : Vec<(String,String)> = c_querys
        .iter()
        .map(|f| f.into())
        .collect();
    let res = r_request_builder.query(&r_querys);
    Box::into_raw(Box::new(res))
}

/// Set HTTP version
#[no_mangle]
pub unsafe extern "C" fn request_builder_version(
        request_builder : *mut RequestBuilder,
        version: *const c_char,
        )-> *mut RequestBuilder{
    //TODO 第一个判断应该都是this指针是否为空，接着马上获取所有权，保证c++ throw错误后没有释放对象
    if request_builder.is_null(){
        update_last_error(anyhow!("client builder or version is null when use min_tls_version"));
        return ptr::null_mut();
    }
    let r_request_builder = Box::from_raw(request_builder);
    let r_version = match to_rust_str(version, "version parse failed") {
        Some("0.9") => reqwest::Version::HTTP_09,
        Some("1.0") => reqwest::Version::HTTP_10,
        Some("1.1") => reqwest::Version::HTTP_11,
        Some("2") => reqwest::Version::HTTP_2,
        Some("3") => reqwest::Version::HTTP_3,
        _ => {return ptr::null_mut();}
    };
    let res = r_request_builder.version(r_version);
    Box::into_raw(Box::new(res))
}

/// Send a form body.
///
/// Sets the body to the url encoded serialization of the passed value,
/// and also sets the `Content-Type: application/x-www-form-urlencoded`
/// header.
#[no_mangle]
pub unsafe extern "C" fn request_builder_form(
        request_builder : *mut RequestBuilder,
        pairs: *const Pair,
        len: usize,
        )-> *mut RequestBuilder{
    if request_builder.is_null() {
        update_last_error(anyhow!("request_builder is null when use query"));
        return ptr::null_mut();
    }
    let r_request_builder = Box::from_raw(request_builder);
    let c_querys : &[Pair]= slice::from_raw_parts(pairs, len);
    let r_querys : Vec<(String,String)> = c_querys
        .iter()
        .map(|f| f.into())
        .collect();
    let res = r_request_builder.form(&r_querys);
    Box::into_raw(Box::new(res))
}


/// Send a JSON body.
///
/// Sets the body to the JSON serialization of the passed value, and
/// also sets the `Content-Type: application/json` header.
#[no_mangle]
pub unsafe extern "C" fn request_builder_json(
        request_builder : *mut RequestBuilder,
        pairs: *const Pair,
        len: usize,
        )-> *mut RequestBuilder{
    if request_builder.is_null() {
        update_last_error(anyhow!("request_builder is null when use json"));
        return ptr::null_mut();
    }
    let r_request_builder = Box::from_raw(request_builder);
    let c_querys : &[Pair]= slice::from_raw_parts(pairs, len);
    let r_querys : Vec<(String,String)> = c_querys
        .iter()
        .map(|f| f.into())
        .collect();
    let res = r_request_builder.json(&r_querys);
    Box::into_raw(Box::new(res))
}

//TODO add multipart

/// Build a `Request`, which can be inspected, modified and executed with
/// `Client::execute()`.
#[no_mangle]
pub unsafe extern "C" fn request_builder_build(request_builder : *mut RequestBuilder)
-> *mut Request{
    if request_builder.is_null() {
        update_last_error(anyhow!("request_builder is null when use build"));
        return ptr::null_mut();
    }
    let r_request_builder = Box::from_raw(request_builder);
    let res = match r_request_builder.build(){
        Ok(v) => v,
        Err(e) => {
            update_last_error(Error::new(e).context("build request failed"));
            return ptr::null_mut();
        }
    };
    Box::into_raw(Box::new(res))
}

/// Constructs the Request and sends it the target URL, returning a Response.
///
/// # Errors
///
/// This method fails if there was an error while sending request,
/// redirect loop was detected or redirect limit was exhausted.
#[no_mangle]
pub unsafe extern "C" fn request_builder_send(
        request_builder : *mut RequestBuilder,
        )-> *mut Response{
    if request_builder.is_null(){
        update_last_error(anyhow!("request_builder is null when use send"));
        return ptr::null_mut();
    }
    let r_request_builder = Box::from_raw(request_builder);
    let result = r_request_builder.send();
    match result {
        Ok(r) => Box::into_raw(Box::new(r)),
        Err(e) => {
            let err = Error::new(e);
            update_last_error(err);
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn request_builder_try_clone(request_builder : *mut RequestBuilder)
-> *mut RequestBuilder{
    if request_builder.is_null(){
        update_last_error(anyhow!("request_builder is null when use send"));
        return ptr::null_mut();
    }
    match (*request_builder).try_clone() {
        Some(v) => Box::into_raw(Box::new(v)),
        None => ptr::null_mut()
    }
}