use std::ffi::c_char;
use crate::c_ffi;


pub struct Person {
    id: i64,
    name: String,
}

impl Person {
    pub fn new(id: i64, name: String) -> Box<Person> {
        let person = Box::new(Person { id, name });
        person
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }
}

// ==================== FFI ====================
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
mod person_tests {
    use super::*;

    #[test]
    fn person_test() {
        let person = Person::new(1, "John".to_string());
        assert_eq!(person.id(), 1);
        assert_eq!(person.name(), "John");
    }
}
