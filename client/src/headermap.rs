use reqwest::header::{HeaderMap, HeaderValue};
use libc::c_char;
use std::ffi::{CString};
use std::ptr;
use anyhow::{Error, anyhow};

use crate::ffi::*;

#[no_mangle]
pub unsafe extern "C" fn new_header_map() -> *mut HeaderMap {
    Box::into_raw(Box::new(HeaderMap::new()))
}
/// Inserts a key-value pair into the map.
///
/// If the map did not previously have this key present, then `None` is
/// returned.
///
/// If the map did have this key present, the new value is associated with
/// the key and all previous values are removed. **Note** that only a single
/// one of the previous values is returned. If there are multiple values
/// that have been previously associated with the key, then the first one is
/// returned. See `insert_mult` on `OccupiedEntry` for an API that returns
/// all values.
///
/// The key is not updated, though; this matters for types that can be `==`
/// without being identical.
#[no_mangle]
pub unsafe extern "C" fn header_map_insert(
        header_map: *mut HeaderMap,
        key: *const c_char,
        value: *const c_char,
        )-> bool{
    if header_map.is_null(){
        update_last_error(anyhow!("header_map is null"));
        return false;
    }
    let r_key = match to_rust_str(key, "parse key error"){
        Some(v) => v,
        None => {return false;}
    };
    let r_value = match to_rust_str(value, "parse value error") {
        Some(v) => v,
        None => {return false;}
    };
    match (&mut *header_map).insert(r_key, r_value.parse().unwrap()) {
        Some(_) => true,
        None => false
    }
}

/// Inserts a key-value pair into the map.
///
/// If the map did not previously have this key present, then `false` is
/// returned.
///
/// If the map did have this key present, the new value is pushed to the end
/// of the list of values currently associated with the key. The key is not
/// updated, though; this matters for types that can be `==` without being
/// identical.
#[no_mangle]
pub unsafe extern "C" fn header_map_append(
        header_map: *mut HeaderMap,
        key: *const c_char,
        value: *const c_char,
        )-> bool{
    if header_map.is_null(){
        update_last_error(anyhow!("header_map is null"));
        return false;
    }
    let r_key = match to_rust_str(key, "parse key error"){
        Some(v) => v,
        None => {return false;}
    };
    let r_value = match to_rust_str(value, "parse value error") {
        Some(v) => v,
        None => {return false;}
    };
    (&mut *header_map).append(r_key, r_value.parse().unwrap())
}

/// Removes a key from the map, returning the value associated with the key.
///
/// Returns `None` if the map does not contain the key. If there are
/// multiple values associated with the key, then the first one is returned.
/// See `remove_entry_mult` on `OccupiedEntry` for an API that yields all
/// values.
#[no_mangle]
pub unsafe extern "C" fn header_map_remove(
        header_map: *mut HeaderMap,
        key: *const c_char,
        )-> bool{
    if header_map.is_null(){
        update_last_error(anyhow!("header_map is null"));
        return false;
    }
    let r_key = match to_rust_str(key, "parse key error"){
        Some(v) => v,
        None => {return false;}
    };
    (&mut *header_map).remove(r_key);
    true
}

///Don't forget free
#[no_mangle]
pub unsafe extern "C" fn header_map_get(
        header_map: *const HeaderMap,
        key: *const c_char,
        )-> *const c_char {
    if header_map.is_null(){
        update_last_error(anyhow!("header_map is null"));
        return ptr::null();
    }
    let r_key = match to_rust_str(key, "parse key error"){
        Some(v) => v,
        None => {return ptr::null();}
    };
    let r_value: Option<&HeaderValue> = (*header_map).get(r_key);

    match r_value{
        Some(v) => {
            let str = match v.to_str(){
                Ok(v) => v,
                Err(e) => {
                    update_last_error(Error::new(e));
                    return ptr::null();
                }
            };
            CString::new(str).unwrap().into_raw()
        },
        None => { return ptr::null(); }
    }
}

/// Returns the number of headers stored in the map.
///
/// This number represents the total number of **values** stored in the map.
/// This number can be greater than or equal to the number of **keys**
/// stored given that a single key may have more than one associated value.
#[no_mangle]
pub unsafe extern "C" fn header_map_len(
        header_map: *const HeaderMap,
        )-> usize{
    if header_map.is_null(){
        update_last_error(anyhow!("header_map is null"));
        return usize::MAX;
    }
    (*header_map).len()
}

/// Returns the number of keys stored in the map.
///
/// This number will be less than or equal to `len()` as each key may have
/// more than one associated value.
///
#[no_mangle]
pub unsafe extern "C" fn header_map_keys_len(
        header_map: *const HeaderMap,
        )-> usize{
    if header_map.is_null(){
        update_last_error(anyhow!("header_map is null"));
        return usize::MAX;
    }
    (*header_map).keys_len()
}

/// Clears the map, removing all key-value pairs. Keeps the allocated memory
/// for reuse.
#[no_mangle]
pub unsafe extern "C" fn header_map_clear(
        header_map: *mut HeaderMap,
        ){
    if header_map.is_null(){
        update_last_error(anyhow!("header_map is null"));
    }else {
        (*header_map).clear()
    }
}
/// Returns the number of headers the map can hold without reallocating.
///
/// This number is an approximation as certain usage patterns could cause
/// additional allocations before the returned capacity is filled.
#[no_mangle]
pub unsafe extern "C" fn header_map_capacity(
        header_map: *const HeaderMap,
        )-> usize{
    if header_map.is_null(){
        update_last_error(anyhow!("header_map is null"));
        return usize::MAX;
    }
    (*header_map).capacity()
}

/// Reserves capacity for at least `additional` more headers to be inserted
/// into the `HeaderMap`.
///
/// The header map may reserve more space to avoid frequent reallocations.
/// Like with `with_capacity`, this will be a "best effort" to avoid
/// allocations until `additional` more headers are inserted. Certain usage
/// patterns could cause additional allocations before the number is
/// reached.
#[no_mangle]
pub unsafe extern "C" fn header_map_reserve(
        header_map: *mut HeaderMap,
        additional: usize
        ){
    if header_map.is_null(){
        update_last_error(anyhow!("header_map is null"));
    }else {
        (*header_map).reserve(additional)
    }
}

/// Returns a view of all values associated with a key.
///
/// The returned view does not incur any allocations and allows iterating
/// the values associated with the key.  See [`GetAll`] for more details.
/// Returns `None` if there are no values associated with the key.
#[no_mangle]
pub unsafe extern "C" fn header_map_get_all(
        header_map: *const HeaderMap,
        key: *const c_char,
        )-> *const c_char {
    if header_map.is_null(){
        update_last_error(anyhow!("header_map is null"));
        return ptr::null();
    }
    let r_key = match to_rust_str(key, "parse key error"){
        Some(v) => v,
        None => {return ptr::null();}
    };
    let r_value: Vec<&HeaderValue> = (*header_map).get_all(r_key).iter().collect();
    CString::new(format!("{:?}",r_value)).unwrap().into_raw()
}

/// Returns true if the map contains a value for the specified key.
/// Return -1 if function failed.
/// why not use bool? because bk is bool.
/// If only return false,can't show function failed or isn't contains.
#[no_mangle]
pub unsafe extern "C" fn header_map_contains_key(
        header_map: *const HeaderMap,
        key: *const c_char,
        bk: *mut bool
        )-> i32{
    if header_map.is_null() || bk.is_null(){
        update_last_error(anyhow!("header_map is null"));
        return -1;
    }
    let r_key = match to_rust_str(key, "parse key error"){
        Some(v) => v,
        None => {return -1;}
    };
    *bk = (*header_map).contains_key(r_key);
    0
}

//TODO get keys array-string
#[no_mangle]
pub unsafe extern "C" fn header_map_keys(
        header_map: *const HeaderMap,
        )-> *const c_char {
    if header_map.is_null(){
        update_last_error(anyhow!("header_map is null"));
        return ptr::null();
    }
    let r_value: Vec<&str> = (*header_map)
        .keys()
        .into_iter()
        .map(|v| v.as_str() )
        .collect();
    CString::new(format!("{:?}",r_value)).unwrap().into_raw()
}

//TODO get values array-string
#[no_mangle]
pub unsafe extern "C" fn header_map_values(
        header_map: *const HeaderMap,
        )-> *const c_char {
    if header_map.is_null(){
        update_last_error(anyhow!("header_map is null"));
        return ptr::null();
    }
    let r_value: Vec<&str> = (*header_map)
        .values()
        .into_iter()
        .map(|v| match v.to_str() {
            Ok(s) => s,
            Err(_) => "opaque"
        })
        .collect();

    CString::new(format!("{:?}",r_value)).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn header_map_destory(header_map: *const HeaderMap){
    if header_map.is_null(){
        return ;
    }
    drop(Box::from_raw(header_map as *mut HeaderMap))
}