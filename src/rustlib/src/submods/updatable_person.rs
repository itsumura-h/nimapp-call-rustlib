use std::ffi::c_char;
use crate::submods::c_ffi;

pub struct UpdatablePerson {
    id: i64,
    name: String,
}

impl UpdatablePerson {
    pub fn new(id: i64, name: String) -> Box<UpdatablePerson> {
        let person = UpdatablePerson { id, name };
        Box::new(person)
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = id
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }
}


#[no_mangle]
pub extern "C" fn new_updatable_person(id: i64, _name: *const c_char) -> *mut UpdatablePerson {
    let name = c_ffi::cstirng_to_string(_name);
    let person = UpdatablePerson::new(id, name);
    Box::into_raw(person)
}

#[no_mangle]
pub extern "C" fn get_updatable_person_id(person: &UpdatablePerson) -> i64 {
    person.id()
}

#[no_mangle]
pub extern "C" fn set_updatable_person_id(person: &mut UpdatablePerson, id: i64) {
    person.set_id(id)
}

#[no_mangle]
pub extern "C" fn get_updatable_person_name(person: &UpdatablePerson) -> *mut c_char {
    c_ffi::string_to_cstring(person.name())
}

#[no_mangle]
pub extern "C" fn set_updatable_person_name(person: &mut UpdatablePerson, _name: *const c_char) {
    let name = c_ffi::cstirng_to_string(_name);
    person.set_name(name)
}


#[cfg(test)]
mod updatable_person_test {
    use super::*;

    #[test]
    fn test1() {
        let mut person = UpdatablePerson::new(1, "John".to_string());
        assert_eq!(person.id(), 1);
        assert_eq!(person.name(), "John");
        person.set_id(2);
        person.set_name("Paul".to_string());
        assert_eq!(person.id(), 2);
        assert_eq!(person.name(), "Paul");
    }
}
