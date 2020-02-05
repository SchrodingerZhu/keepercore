#![feature(linkage)]
use std::ffi::*;
mod client;
mod utils;
pub use utils::*;
use crate::client::{query, Insertion};
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug)]
struct Result {
    length: usize,
    capacity: usize,
    vector: *mut *mut c_char
}

#[no_mangle]
unsafe extern "C" fn fetch_list(password: *const c_char) -> Result {
    let rng = botan::RandomNumberGenerator::new();
    if rng.is_err() {
        return transform_vec(vec![String::from("Unable to initialize list")]);
    }
    let rng = rng.unwrap();
    let mut bits = Vec::new();
    bits.resize(16, 0);
    rng.fill(bits.as_mut()).unwrap();
    let password = CStr::from_ptr(password);
    let list = client::query(String::from("list"),
                  String::from_utf8_unchecked(bits), password);
    transform_vec(list)
}

#[no_mangle]
unsafe extern "C" fn fetch_password(name: *const c_char, password: *const c_char) -> Result {
    let password = CStr::from_ptr(password);
    let name = CStr::from_ptr(name);
    let name = name.to_str().map(|x|x.to_string()).unwrap();
    let res = query("fetch".to_string(), name, password);
    transform_vec(res)
}

#[no_mangle]
unsafe extern "C" fn add_password(name: *const c_char, content: *const c_char, password: *const c_char) -> Result {
    let password = CStr::from_ptr(password);
    let name = CStr::from_ptr(name);
    let content = CStr::from_ptr(content);
    let name = name.to_str().map(|x|x.to_string()).unwrap();
    let content = content.to_str().map(|x|x.to_string()).unwrap();
    let json = serde_json::to_string(&Insertion {
        name, content
    }).unwrap();
    let res = query("add".to_string(), json, password);
    transform_vec(res)
}

#[no_mangle]
unsafe extern "C" fn delete_password(name: *const c_char, password: *const c_char) -> Result {
    let password = CStr::from_ptr(password);
    let name = CStr::from_ptr(name);
    let name = name.to_str().map(|x|x.to_string()).unwrap();
    let res = query("delete".to_string(), name, password);
    transform_vec(res)
}

#[no_mangle]
unsafe extern "C" fn generate_password(name: *const c_char, password: *const c_char) -> Result {
    let password = CStr::from_ptr(password);
    let name = CStr::from_ptr(name);
    let name = name.to_str().map(|x|x.to_string()).unwrap();
    let res = query("generate".to_string(), name, password);
    transform_vec(res)
}

#[no_mangle]
unsafe extern "C" fn clean_result(list: Result) {
    if list.length == 0 {return; }
    let vec = Vec::from_raw_parts(list.vector, list.length, list.capacity);
    for i in &vec {
        let t = CString::from_raw(*i);
        drop(t);
    }
    drop(vec)
}

#[no_mangle]
unsafe extern "C" fn botan_version() -> Result {
    let string = vec![botan::Version::current().unwrap().string];
    transform_vec(string)
}

#[no_mangle]
unsafe extern "C" fn get_server() -> Result {
    let string = vec![SERVER.to_string()];
    transform_vec(string)
}

