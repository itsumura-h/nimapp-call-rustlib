mod submods {
    pub mod fib;
    pub mod c_ffi;
    pub mod crypto;
    pub mod person;
    pub mod updatable_person;
}

use crate::submods::c_ffi;

#[no_mangle]
pub extern "C" fn add(a: i64, b: i64) -> i64 {
    return a + b;
}
