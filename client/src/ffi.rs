//! The foreign function interface which exposes this library to non-Rust
//! languages.

use std::ffi::{CString, CStr};
use std::{ptr, slice, usize, time::Duration};
use anyhow::{Error, anyhow};
use libc::{c_char, c_int };
use std::cell::RefCell;
use std::convert::From;

thread_local! {
    static LAST_ERROR : RefCell<Option<Box<Error>>> = RefCell::new(None);
}


#[repr(C)]
//#[derive(Clone)]
pub struct Pair{
    key: *const c_char,
    value: *const c_char,
}

impl From<(String, String)> for Pair{
    fn from(tup: (String, String)) -> Self {
        Pair {
            key: match CString::new(tup.0) {
                Ok(v) => v.into_raw(),
                Err(_) => ptr::null(),
            },
            value: match CString::new(tup.1){
                Ok(v) => v.into_raw(),
                Err(_) => ptr::null_mut(),
            }
        }
    }
}

impl From<&Pair> for (String,String) {
    fn from(tup: &Pair) -> (String,String) {
        let key = match to_rust_str(tup.key, "key pasre error") {
            Some(v) => v.to_string(),
            None => String::from(""),
        };
        let value = match to_rust_str(tup.value, "value pasre error") {
            Some(v) => v.to_string(),
            None => String::from(""),
        };
        (key, value)
    }
}

/// Update the most recent error, clearing whatever may have been there before.
pub fn update_last_error(err: Error) {
    error!("Setting LAST_ERROR: {}",err);
    println!("update_last_error : {}", err);
     {
         // Print a pseudo-backtrace for this error, following back each error's
         // cause until we reach the root error.
         let mut cause = err.source();
         while let Some(parent_err) = cause {
             warn!("Caused by: {}", parent_err);
             cause = parent_err.source();
         }
     }

    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(Box::new(err));
    });
}

/// Retrieve the most recent error, clearing it in the process.
pub fn take_last_error() -> Option<Box<Error>> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

/// Calculate the number of bytes in the last error's error message **not**
/// including any trailing `null` characters.
#[no_mangle]
pub extern "C" fn last_error_length() -> c_int {
    LAST_ERROR.with(|prev| match *prev.borrow() {
        Some(ref err) => err.to_string().len() as c_int +1,
        None => 0,
})
}

/// Write the most recent error message into a caller-provided buffer as a UTF-8
/// string, returning the number of bytes written.
///
/// # Note
///
/// This writes a **UTF-8** string into the buffer. Windows users may need to
/// convert it to a UTF-16 "unicode" afterwards.
///
/// If there are no recent errors then this returns `0` (because we wrote 0
/// bytes). `-1` is returned if there are any errors, for example when passed a
/// null pointer or a buffer of insufficient size.
#[no_mangle]
pub unsafe extern "C" fn last_error_message(buffer: *mut c_char, length: c_int) -> c_int {
    if buffer.is_null() {
        warn!("Null pointer passed into last_error_message() as the buffer");
        return -1;
    }

    let last_error = match take_last_error() {
        Some(err) => err,
        None => return 0,
    };

    let error_message = last_error.to_string();

    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if error_message.len() >= buffer.len() {
        warn!("Buffer provided for writing the last error message is too small.");
        warn!("Expected at least {} bytes but got {}", error_message.len() + 1, buffer.len());
        return -1;
    }

    ptr::copy_nonoverlapping(
            error_message.as_ptr(),
            buffer.as_mut_ptr(),
            error_message.len(),
            );

    // Add a trailing null so people using the string as a `char *` don't
    // accidentally read into garbage.
    buffer[error_message.len()] = 0;

    error_message.len() as c_int
}

#[no_mangle]
pub unsafe extern "C" fn free_string(s: *const c_char){
    if s.is_null() {
        update_last_error(anyhow!("string is null"));
        return;
    }
    drop(CString::from_raw(s as *mut _));
}

#[no_mangle]
pub unsafe extern "C" fn free_vec_u8(s: *const u8, len: usize){
    if s.is_null() {
        update_last_error(anyhow!("u8 ptr is null"));
        return;
    }
    drop(Vec::from_raw_parts(s as *mut u8,len,len));
}

pub fn to_rust_str<'a>(ptr: *const c_char,err_tip :&'static str) -> Option<&'a str>{
    if ptr.is_null() {
        return None;
    }
    unsafe {
        match CStr::from_ptr(ptr).to_str() {
            Ok(v) => Some(v),
            Err(e) => {
                update_last_error(Error::new(e).context(err_tip));
                None
            }
        }
    }
}


pub fn u64_to_millos_duration(millisecond :*const u64) -> Option<Duration> {
    if millisecond.is_null() {
        return None;
    }
    unsafe{
        Some(Duration::from_millis(*millisecond))
    }
}
