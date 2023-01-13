# const libpath = "/application/src/rustlib/target/release/librustlib.a"

# {.passL:libpath.}
# proc add*(a, b:int64):int64 {.cdecl, importc: "add".}
# proc fib*(n:int):int {.cdecl, importc: "fib".}

# type PersonObj{.pure, final.}  = object
#   id:int
#   name:cstring
# type Person* = ref PersonObj

# proc newPerson(id:int, name:cstring):Person {.cdecl, importc:"new_person".}
# proc new*(_:type Person, id:int, name:string):Person = newPerson(id, name.cstring)

# proc getPersonName(self:Person):cstring {.cdecl, importc:"get_person_name".}
# proc name*(self:Person):string = $self.getPersonName()


const libpath = "/application/src/rustlib/target/release/librustlib.so"

proc add*(a, b:int64):int64 {.dynlib:libpath, importc: "add".}
proc fib*(n:int):int {.dynlib:libpath, importc: "fib".}

type PersonObj{.pure, final.}  = object
  id:int
  name:cstring
type Person* = ref PersonObj

proc newPerson(id:int, name:cstring):Person {.dynlib:libpath, importc:"new_person".}
proc new*(_:type Person, id:int, name:string):Person = newPerson(id, name.cstring)

proc getPersonId(self:Person):int64 {.dynlib:libpath, importc:"get_person_id".}
proc id*(self:Person):int = self.getPersonId().int

proc getPersonName(self:Person):cstring {.dynlib:libpath, importc:"get_person_name".}
proc name*(self:Person):string = $self.getPersonName()
