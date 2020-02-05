use botan::Privkey;
use std::ffi::*;
use std::os::raw::c_char;

static MY_PRIVATE : &'static str = include_str!("/code/keys/keeper_pri.pem");
pub static SERVER_PUBLIC: &'static str = include_str!("/code/server/server.pem.pub");
pub static SERVER : &'static str = "http://192.168.0.104:1024";

#[inline(always)]
pub(crate) fn get_private(password: &CStr) -> Option<Privkey> {
    password.to_str().ok().and_then(|x|
        Privkey::load_encrypted_pem(MY_PRIVATE, x).ok()
    )
}

#[inline(always)]
pub(crate) fn transform_vec(vec: Vec<String>) -> crate::Result {
    let mut inner = vec.into_iter().map(|x| {
        let string = CString::new(x.into_bytes()).unwrap();
        let ptr = string.as_ptr() as *mut c_char;
        std::mem::forget(string);
        ptr
    }).collect::<Vec<_>>();
    let res = crate::Result {
        length: inner.len(),
        capacity: inner.capacity(),
        vector: inner.as_mut_ptr()
    };
    std::mem::forget(inner);
    res
}
