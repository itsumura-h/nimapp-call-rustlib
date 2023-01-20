use std::ffi::c_char;
use std::ffi::CStr;
use std::ffi::CString;

pub fn cstirng_to_string(_arg: *const c_char) -> String {
    let arg = unsafe {
        assert!(!_arg.is_null());
        let c_str = CStr::from_ptr(_arg);
        let str_slice = c_str.to_str().unwrap();
        drop(c_str);
        str_slice.to_owned()
    };
    arg
}

pub fn string_to_cstring(_arg: String) -> *mut c_char {
    CString::new(_arg).unwrap().into_raw()
}
