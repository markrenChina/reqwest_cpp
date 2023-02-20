use reqwest::header::HeaderMap;
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn new_header_map() -> *mut HeaderMap {
    Box::into_raw(Box::new(HeaderMap::new()))
}

#[no_mangle]
pub unsafe extern "C" fn header_map_insert(
        header_map: *mut HeaderMap,
        key: *const c_char,
        value: *const c_char,
        )-> i32{
    if header_map.is_null() || key.is_null() || value.is_null() {
        return -1;
    }
    let r_key = CStr::from_ptr(key).to_str().unwrap();
    let r_value = CStr::from_ptr(value).to_str().unwrap();
    let bk = (&mut *header_map).insert(r_key, r_value.parse().unwrap());
    0
}
