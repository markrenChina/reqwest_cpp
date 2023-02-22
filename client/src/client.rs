use reqwest::blocking::{Client, ClientBuilder, RequestBuilder};
use reqwest::header::HeaderMap;
use reqwest::redirect;
use libc::c_char;
use std::{ptr, ffi::CStr};
use anyhow::{Error, anyhow};
use std::net::IpAddr;

use crate::ffi::*;

/// Constructs a new `ClientBuilder`.
#[no_mangle]
pub unsafe extern "C" fn new_client_builder() -> *mut ClientBuilder{
    Box::into_raw(Box::new(ClientBuilder::new()))
}

/// Sets the default headers for every request.
#[no_mangle]
pub unsafe extern "C" fn client_builder_default_headers(
        client_builder: *mut ClientBuilder,
        headermap :*mut HeaderMap,
)->*mut ClientBuilder{
    if client_builder.is_null() {
        update_last_error(anyhow!("client_builder is null when use headers"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.default_headers(*Box::from_raw(headermap));
    Box::into_raw(Box::new(result))
}

/// Sets the `User-Agent` header to be used by this client.
#[no_mangle]
pub unsafe extern "C" fn client_builder_user_agent(
        client_builder: *mut ClientBuilder,
        value: *const c_char
)->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("user agent is null when use user_agent"));
        return ptr::null_mut();
    }
    let r_value = to_rust_str(value, "").unwrap();
    let result: ClientBuilder = Box::from_raw(client_builder).user_agent(r_value);
    Box::into_raw(Box::new(result))
}

/// Set a `redirect::Policy` for this client.
///
/// Default will follow redirects up to a maximum of 10.
#[no_mangle]
pub unsafe extern "C" fn client_builder_redirect(
        client_builder: *mut ClientBuilder,
        policy: usize
)->*mut ClientBuilder{
    if client_builder.is_null() {
        update_last_error(anyhow!("client_builder is null when use redirect"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let r_policy : redirect::Policy = redirect::Policy::limited(policy);
    let result = r_client_builder.redirect(r_policy);
    Box::into_raw(Box::new(result))
}

/// Enable or disable automatic setting of the `Referer` header.
///
/// Default is `true`.
#[no_mangle]
pub unsafe extern "C" fn client_builder_referer(
        client_builder: *mut ClientBuilder,
        enable: bool
)->*mut ClientBuilder{
    if client_builder.is_null() {
        update_last_error(anyhow!("client_builder is null when use referer"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.referer(enable);
    Box::into_raw(Box::new(result))
}

// Proxy options

/// Add a `Proxy` to the list of proxies the `Client` will use.
///
/// # Note
///
/// Adding a proxy will disable the automatic usage of the "system" proxy.
#[no_mangle]
pub unsafe extern "C" fn client_builder_proxy(
        client_builder: *mut ClientBuilder,
        proxy: *mut reqwest::Proxy
)->*mut ClientBuilder{
    if client_builder.is_null() || proxy.is_null(){
        update_last_error(anyhow!("client_builder or proxy is null"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.proxy(*Box::from_raw(proxy));
    Box::into_raw(Box::new(result))
}

// Timeout options

/// Set a timeout for connect, read and write operations of a `Client`.
///
/// Default is 30 seconds.
///
/// Pass `None` to disable timeout.
#[no_mangle]
pub unsafe extern "C" fn client_builder_timeout(
        client_builder: *mut ClientBuilder,
        millisecond: *const u64
)->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use timeout"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.timeout(u64_to_millos_duration(millisecond));
    Box::into_raw(Box::new(result))
}

// HTTP options

/// Set an optional timeout for idle sockets being kept-alive.
///
/// Pass `None` to disable timeout.
///
/// Default is 90 seconds.
#[no_mangle]
pub unsafe extern "C" fn client_builder_pool_idle_timeout(
        client_builder: *mut ClientBuilder,
        millisecond: *const u64
)->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use pool_idle_timeout"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.pool_idle_timeout(u64_to_millos_duration(millisecond));

    Box::into_raw(Box::new(result))
}

/// Sets the maximum idle connection per host allowed in the pool.
#[no_mangle]
pub unsafe extern "C" fn client_builder_pool_max_idle_per_host(
        client_builder: *mut ClientBuilder,
        max: usize,
)->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use pool_max_idle_per_host"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.pool_max_idle_per_host(max);
    Box::into_raw(Box::new(result))
}

/// Sets the maximum idle connection per host allowed in the pool.
#[no_mangle]
pub unsafe extern "C" fn client_builder_http1_title_case_headers(
        client_builder: *mut ClientBuilder
)->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use timeout"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.http1_title_case_headers();
    Box::into_raw(Box::new(result))
}

/// Set whether HTTP/1 connections will accept obsolete line folding for
/// header values.
///
/// Newline codepoints (`\r` and `\n`) will be transformed to spaces when
/// parsing.
#[no_mangle]
pub unsafe extern "C" fn client_builder_http1_allow_obsolete_multiline_headers_in_responses(
        client_builder: *mut ClientBuilder,
        val: bool
)->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use http1_allow_obsolete_multiline_headers_in_responses"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.http1_allow_obsolete_multiline_headers_in_responses(val);
    Box::into_raw(Box::new(result))
}

/// Only use HTTP/1.
#[no_mangle]
pub unsafe extern "C" fn client_builder_http1_only(
        client_builder: *mut ClientBuilder
)->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use http1_only"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.http1_only();
    Box::into_raw(Box::new(result))
}

/// Allow HTTP/0.9 responses
#[no_mangle]
pub unsafe extern "C" fn client_builder_http09_responses(
        client_builder: *mut ClientBuilder
)->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use http09_responses"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.http09_responses();
    Box::into_raw(Box::new(result))
}

/// Only use HTTP/2.
#[no_mangle]
pub unsafe extern "C" fn client_builder_http2_prior_knowledge(
        client_builder: *mut ClientBuilder
)->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use http2_prior_knowledge"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.http2_prior_knowledge();
    Box::into_raw(Box::new(result))
}

/// Sets the `SETTINGS_INITIAL_WINDOW_SIZE` option for HTTP2 stream-level flow control.
///
/// Default is currently 65,535 but may change internally to optimize for common uses.
#[no_mangle]
pub unsafe extern "C" fn client_builder_http2_initial_stream_window_size(
        client_builder: *mut ClientBuilder,
        size : *mut u32,
)->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use http2_initial_stream_window_size"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.http2_initial_stream_window_size(*size);
    Box::into_raw(Box::new(result))
}

/// Sets the max connection-level flow control for HTTP2
///
/// Default is currently 65,535 but may change internally to optimize for common uses.
#[no_mangle]
pub unsafe extern "C" fn client_builder_http2_initial_connection_window_size(
        client_builder: *mut ClientBuilder,
        size : *mut u32,
        )->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use http2_initial_connection_window_size"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.http2_initial_connection_window_size(*size);
    Box::into_raw(Box::new(result))
}

/// Sets whether to use an adaptive flow control.
///
/// Enabling this will override the limits set in `http2_initial_stream_window_size` and
/// `http2_initial_connection_window_size`.
#[no_mangle]
pub unsafe extern "C" fn client_builder_http2_adaptive_window(
        client_builder: *mut ClientBuilder,
        enable: bool
        )->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use http2_adaptive_window"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.http2_adaptive_window(enable);
    Box::into_raw(Box::new(result))
}

/// Sets the maximum frame size to use for HTTP2.
///
/// Default is currently 16,384 but may change internally to optimize for common uses.
#[no_mangle]
pub unsafe extern "C" fn client_builder_http2_max_frame_size(
        client_builder: *mut ClientBuilder,
        size : *mut u32,
)->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use http2_adaptive_window"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.http2_max_frame_size(*size);
    Box::into_raw(Box::new(result))
}

// TCP options

/// Set whether sockets have `TCP_NODELAY` enabled.
///
/// Default is `true`.
#[no_mangle]
pub unsafe extern "C" fn client_builder_tcp_nodelay(
        client_builder: *mut ClientBuilder,
        enable: bool,
        )->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use http2_adaptive_window"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.tcp_nodelay(enable);
    Box::into_raw(Box::new(result))
}

/// Bind to a local IP Address.
#[no_mangle]
pub unsafe extern "C" fn client_builder_local_address(
        client_builder: *mut ClientBuilder,
        local_address: *mut c_char,
        )->*mut ClientBuilder{
    if client_builder.is_null() {
        update_last_error(anyhow!("client_builder is null when use local_address"));
        return ptr::null_mut();
    }
    let r_local_address_str :&str;
    match to_rust_str(local_address, "arg is local_address") {
        Some(s) => r_local_address_str = s,
        None => {return ptr::null_mut();}
    }
    let r_local_address : IpAddr;
    match r_local_address_str.parse() {
        Ok(v) => r_local_address = v,
        Err(e) => {
            update_last_error(Error::new(e).context("ip illegality"));
            return ptr::null_mut();
        }
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.local_address(r_local_address);
    Box::into_raw(Box::new(result))
}

/// Set that all sockets have `SO_KEEPALIVE` set with the supplied duration.
///
/// If `None`, the option will not be set.
#[no_mangle]
pub unsafe extern "C" fn client_builder_tcp_keepalive(
        client_builder: *mut ClientBuilder,
        millisecond: *const u64
)->*mut ClientBuilder{
    if client_builder.is_null(){
        update_last_error(anyhow!("client_builder is null when use tcp_keepalive"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.tcp_keepalive(u64_to_millos_duration(millisecond));

    Box::into_raw(Box::new(result))
}

// TLS options

/// Add a custom root certificate.
///
/// This allows connecting to a server that has a self-signed
/// certificate for example. This **does not** replace the existing
/// trusted store.
#[no_mangle]
pub unsafe extern "C" fn client_builder_add_root_certificate(
        client_builder: *mut ClientBuilder,
        cert_path: *mut c_char,
        )->*mut ClientBuilder{
    if client_builder.is_null() {
        update_last_error(anyhow!("client_builder is null when use local_address"));
        return ptr::null_mut();
    }
    let r_cert_path :&str;
    match to_rust_str(cert_path, "cert_path is wrong") {
        Some(s) => r_cert_path = s,
        None => {return ptr::null_mut();}
    }

    let der = match std::fs::read(r_cert_path) {
        Ok(v) => v,
        Err(e) => {
            update_last_error(Error::new(e));
            return ptr::null_mut();
        }
    };
    let cert = match reqwest::Certificate::from_der(&der){
        Ok(v) => v,
        Err(e) => {
            update_last_error(Error::new(e));
            return ptr::null_mut();
        }
    };
    let r_client_builder = Box::from_raw(client_builder);
    Box::into_raw(Box::new(r_client_builder.add_root_certificate(cert)))
}


/// Controls the use of built-in system certificates during certificate validation.
///
/// Defaults to `true` -- built-in system certs will be used.
///
/// # Optional
///
/// This requires the optional `default-tls`, `native-tls`, or `rustls-tls(-...)`
/// feature to be enabled.
#[no_mangle]
pub unsafe extern "C" fn client_builder_tls_built_in_root_certs(
        client_builder: *mut ClientBuilder,
        tls_built_in_root_certs: bool
        )->*mut ClientBuilder{
    if client_builder.is_null() {
        update_last_error(anyhow!("client_builder is null when use tls_built_in_root_certs"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.tls_built_in_root_certs(tls_built_in_root_certs);
    Box::into_raw(Box::new(result))
}

/// Controls the use of certificate validation.
///
/// Defaults to `false`.
///
/// # Warning
///
/// You should think very carefully before using this method. If
/// invalid certificates are trusted, *any* certificate for *any* site
/// will be trusted for use. This includes expired certificates. This
/// introduces significant vulnerabilities, and should only be used
/// as a last resort.
#[no_mangle]
pub unsafe extern "C" fn client_builder_danger_accept_invalid_certs(
        client_builder: *mut ClientBuilder,
        accept_invalid_certs: bool
)->*mut ClientBuilder{
    if client_builder.is_null() {
        update_last_error(anyhow!("client_builder is null when use danger_accept_invalid_certs"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.danger_accept_invalid_certs(accept_invalid_certs);
    Box::into_raw(Box::new(result))
}

/// Controls the use of TLS server name indication.
///
/// Defaults to `true`.
#[no_mangle]
pub unsafe extern "C" fn client_builder_tls_sni(
        client_builder: *mut ClientBuilder,
        tls_sni: bool
)->*mut ClientBuilder{
    if client_builder.is_null() {
        update_last_error(anyhow!("client_builder is null when use danger_accept_invalid_certs"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    let result = r_client_builder.tls_sni(tls_sni);
    Box::into_raw(Box::new(result))
}


#[no_mangle]
pub unsafe extern "C" fn client_builder_min_tls_version(
        client_builder: *mut ClientBuilder,
        version: *const c_char
)->*mut ClientBuilder{
    if client_builder.is_null() || version.is_null(){
        update_last_error(anyhow!("client builder or version is null when use min_tls_version"));
        return ptr::null_mut();
    }
    let r_version = match to_rust_str(version, "version to str failed") {
        Some("1.0") => reqwest::tls::Version::TLS_1_0,
        Some("1.1") => reqwest::tls::Version::TLS_1_1,
        Some("1.2") => reqwest::tls::Version::TLS_1_2,
        Some("1.3") => reqwest::tls::Version::TLS_1_3,
        _ => {return ptr::null_mut();}
    };
    let result: ClientBuilder = Box::from_raw(client_builder).min_tls_version(r_version);
    Box::into_raw(Box::new(result))
}


#[no_mangle]
pub unsafe extern "C" fn client_builder_destory(client_builder: *mut ClientBuilder){
    if !client_builder.is_null(){
        drop(Box::from_raw(client_builder));
    }
}

/// Returns a `Client` that uses this `ClientBuilder` configuration.
///
/// # Errors
///
/// This method fails if TLS backend cannot be initialized, or the resolver
/// cannot load the system configuration.
#[no_mangle]
pub unsafe extern "C" fn client_builder_build_client(client_builder: *mut ClientBuilder)
->*mut Client {
    if client_builder.is_null() {
        update_last_error(anyhow!("client builder is null when use client_builder_build_client"));
        return ptr::null_mut();
    }
    let r_client_builder = Box::from_raw(client_builder);
    match r_client_builder.build() {
        Ok(c) => Box::into_raw(Box::new(c)),
        Err(e) => {
            let err = Error::new(e).context("Unable to build client");
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
    if client.is_null() || url.is_null(){
        update_last_error(anyhow!("client is null"));
        return ptr::null_mut();
    }
    let r_value = CStr::from_ptr(url).to_str().unwrap();
    let r_client : &Client = &*client;
    let rb : reqwest::blocking::RequestBuilder = r_client.get(r_value);
    Box::into_raw(Box::new(rb))
}




