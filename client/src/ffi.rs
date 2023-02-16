//! The foreign function interface which exposes this library to non-Rust
//! languages.

use std::ffi::{CStr, CString};
use std::{ptr, slice};
use std::error::Error as StdError;
use libc::{c_char, c_int, size_t };
use reqwest::{Url, Method};
use std::cell::RefCell;
//use errors::*;

//
//thread_local! {
//    static LAST_ERROR : RefCell<Option<Box<Error>>> = RefCell::new(None);
//}

/// Update the most recent error, clearing whatever may have been there before.
//pub fn update_last_error<E: StdError + 'static>(err: E) {
//    error!("Setting LAST_ERROR: {}",err);
//
//     {
//         // Print a pseudo-backtrace for this error, following back each error's
//         // cause until we reach the root error.
//         let mut cause = err.cause();
//         while let Some(parent_err) = cause {
//             warn!("Caused by: {}", parent_err);
//             cause = parent_err.cause();
//         }
//     }
//
//    LAST_ERROR.with(|prev| {
//        *prev.borrow_mut() = Some(Box::new(err));
//    });
//}

/// Retrieve the most recent error, clearing it in the process.
//pub fn take_last_error() -> Option<Box<StdError>> {
//    LAST_ERROR.with(|prev| prev.borrow_mut().take())
//}

/// Calculate the number of bytes in the last error's error message **not**
/// including any trailing `null` characters.
//#[no_mangle]
//pub extern "C" fn last_error_length() -> c_int {
//    LAST_ERROR.with(|prev| match *prev.borrow() {
//        Some(ref err) => err.to_string().len() as c_int +1,
//        None => 0,
//})
//}

#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char){
    if s.is_null() {
        return;
    }
    drop(CString::from_raw(s));
}
