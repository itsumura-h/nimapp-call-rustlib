#[no_mangle]
pub extern "C" fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

#[no_mangle]
pub extern "C" fn fib(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 2) + fib(n - 1),
    }
}


use std::ffi::c_char;
mod submods {
    pub mod c_ffi;
    pub mod person;
}
use crate::submods::c_ffi;
use crate::submods::person::Person;

#[no_mangle]
pub extern "C" fn new_person(id: i64, _name: *const c_char) -> *mut Person {
    let name = c_ffi::cstirng_to_string(_name);
    let person = Person::new(id, name);
    Box::into_raw(person)
}

#[no_mangle]
pub extern "C" fn get_person_id(person: &Person) -> i64 {
    person.id()
}

#[no_mangle]
pub extern "C" fn get_person_name(person: &Person) -> *mut c_char {
    c_ffi::string_to_cstring(person.name())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn fib_test() {
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(8), 21);
        assert_eq!(fib(9), 34);
        assert_eq!(fib(10), 55);
    }
}
