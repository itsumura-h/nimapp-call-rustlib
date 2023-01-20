mod submods {
    pub mod c_ffi;
    pub mod crypto;
    pub mod fib;
    pub mod person;
    pub mod updatable_person;
}

use submods::c_ffi::cstirng_to_string;
use std::ffi::c_char;

use crate::submods::c_ffi::{self, string_to_cstring};


#[no_mangle]
pub extern "C" fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

#[no_mangle]
pub extern "C" fn recieve_str_then_return(str: &mut c_char) -> *mut c_char {
    let s = cstirng_to_string(str);
    print!("{}", s);
    string_to_cstring(s)
}
