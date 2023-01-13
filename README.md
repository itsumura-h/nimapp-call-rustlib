Nimでプロジェクトを作る
```
cd /application/src
nimble init nimapp
```

Rustでプロジェクトを作る
```
cargo new rustlib --lib
```

実行
```
cargo test
```

Rustで動的ライブラリを作るには
- #[no_mangle]
- extern "C"
- crate-type   = ["cdylib"]

静的アーカイブを作るには、Cargo.tomlを以下にする
```
[lib]
crate-type   = ["staticlib"]
```
[Rust のライブラリ crate_type をまとめてみた  ](https://qiita.com/etoilevi/items/4bd4c5b726e41f5a6689)


```rust
#[no_mangle]
pub extern "C" fn fib(n:i64)->i64{
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-2) + fib(n-1),
    }
}
```

文字列の扱い
Rustの引数の型は `*const c_char`、返り値の型は `*mut c_char`にする

```rust
use std::ffi::c_char;
use std::ffi::CStr;
use std::ffi::CString;

// `*const c_char`から`String`への変換
fn cstirng_to_string(_arg:*const c_char)->String{
    let arg = unsafe {
        assert!(!_arg.is_null());
        let c_str = CStr::from_ptr(_arg);
        let str_slice = c_str.to_str().unwrap();
        str_slice.to_owned()
    };
    arg
}

// `String`から`*mut c_char`への変換
fn string_to_cstring(_arg:String)->*mut c_char{
    CString::new(_arg).unwrap().into_raw()
}
```

構造体をNimから呼び出すには

rust側pureコード
```rust
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
```
- 独自型のコンストラクタの返り値の型は `Box<Person>`

rust側グルー層
```rust
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
```
- 独自型のコンストラクタの返り値の型は`BOX::into_raw()`を介して`*mut Person`にする

Nim側グルー層

```nim
# 独自型を定義する
type PersonObj{.pure, final.}  = object
  id:int
  name:cstring
# 独自型の参照を定義する
type Person* = ref PersonObj

# C言語の型に合わせたグルー関数を定義する
proc newPerson(id:int, name:cstring):Person {.dynlib:libpath, importc:"new_person".}
# C言語の型とNimの型を相互変換する関数を定義する。Nimのコードからはこちらを呼ぶ
proc new*(_:type Person, id:int, name:string):Person = newPerson(id, name.cstring)

proc getPersonId(self:Person):int64 {.dynlib:libpath, importc:"get_person_id".}
proc id*(self:Person):int = self.getPersonId().int

proc getPersonName(self:Person):cstring {.dynlib:libpath, importc:"get_person_name".}
proc name*(self:Person):string = $self.getPersonName()
```

Nim側アプリケーション
```nim
let person = Person.new(1, "John")
echo person.repr
echo person.id()
echo person.name()
```
