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

type VectorPtr = ptr object

proc fibArrayLib(n:int):VectorPtr {.dynlib:libpath, importc: "fib_array".}
proc len(self:VectorPtr):int {.dynlib:libpath, importc: "get_vector_len".}
proc `[]`(self:VectorPtr, offset:int):int {.dynlib:libpath, importc: "get_vector_item".}
proc fibArray*(n:int):seq[int] =
  let v = fibArrayLib(n)
  defer: v.dealloc()
  var s = newSeq[int](n)
  for i in 0..<v.len:
    s[i] = v[i]
  return s


# ==================== Person ====================
type
  PersonObj {.pure, final.} = object
    id:int
    name:cstring

  PersonPtr = ptr PersonObj

  Person* = ref object
    rawPtr: PersonPtr


proc newPerson(id:int, name:cstring):PersonPtr {.dynlib:libpath, importc:"new_person".}
proc new*(_:type Person, id:int, name:string):Person = Person(rawPtr:newPerson(id, name.cstring))

proc getPersonId(self:PersonPtr):int64 {.dynlib:libpath, importc:"get_person_id".}
proc id*(self:Person):int = self.rawPtr.getPersonId().int

proc getPersonName(self:PersonPtr):cstring {.dynlib:libpath, importc:"get_person_name".}
proc name*(self:Person):string = $self.rawPtr.getPersonName()


# ==================== UpdatablePerson ====================
type
  UpdatablePersonObj {.pure, final.} = object
    id:int
    name:cstring

  UpdatablePersonPtr = ptr UpdatablePersonObj

  UpdatablePerson* = ref object
    rawPtr: UpdatablePersonPtr


proc newUpdatablePerson(id:int, name:cstring):UpdatablePersonPtr {.dynlib:libpath, importc:"new_updatable_person".}
proc new*(_:type UpdatablePerson, id:int, name:string):UpdatablePerson = UpdatablePerson(rawPtr:newUpdatablePerson(id, name.cstring))

proc getUpdatablePersonId(self:UpdatablePersonPtr):int64 {.dynlib:libpath, importc:"get_updatable_person_id".}
proc id*(self:UpdatablePerson):int = self.rawPtr.getUpdatablePersonId().int

proc setUpdatablePersonId(self:UpdatablePersonPtr, id:int) {.dynlib:libpath, importc:"set_updatable_person_id".}
proc setId*(self:UpdatablePerson, id:int) = self.rawPtr.setUpdatablePersonId(id)

proc getUpdatablePersonName(self:UpdatablePersonPtr):cstring {.dynlib:libpath, importc:"get_updatable_person_name".}
proc name*(self:UpdatablePerson):string = $self.rawPtr.getUpdatablePersonName()

proc setUpdatablePersonName(self:UpdatablePersonPtr, name:cstring) {.dynlib:libpath, importc:"set_updatable_person_name".}
proc setName*(self:UpdatablePerson, name:string) = self.rawPtr.setUpdatablePersonName(name.cstring)


# ==================== Crypto ====================
