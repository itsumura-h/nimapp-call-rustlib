pub struct Person {
  id: i64,
  name: String,
}

impl Person {
  pub fn new(id: i64, name: String) -> Box<Person> {
      let person = Box::new(Person { id, name });
       person
  }

  pub fn id(&self)->i64{
    self.id
  }

  pub fn name(&self) -> String {
    self.name.to_string()
  }
}
