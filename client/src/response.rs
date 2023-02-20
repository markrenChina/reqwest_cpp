use reqwest::blocking::Response;
use libc::c_char;
use std::ptr;
use std::ffi::CString;


#[no_mangle]
pub unsafe extern "C" fn response_text(response: *mut Response) -> *mut c_char{
    let result = Box::from_raw(response).text();
    match result {
        Ok(v) => CString::new(v).unwrap().into_raw(),
        Err(_) => ptr::null_mut(),
    }
}