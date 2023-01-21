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
proc recieveStrThenReturn*(str:cstring):cstring {.dynlib:libpath, importc:"recieve_str_then_return".}

# ==================== Fibo ====================
type FibPtr = ptr object

proc fibArrayLib(n:uint64):FibPtr {.dynlib:libpath, importc: "fib_array".}
proc len(self:FibPtr):int {.dynlib:libpath, importc: "get_fib_len".}
proc `[]`(self:FibPtr, offset:int):int {.dynlib:libpath, importc: "get_fib_item".}
proc fibArray*(n:int):seq[int] =
  let v = fibArrayLib(n.uint64)
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
type
  SecretKey = ptr object


proc createSecretKeyLib():SecretKey {.dynlib:libpath, importc:"create_secret_key".}
proc len(self:SecretKey):int {.dynlib:libpath, importc:"get_secret_key_len".}
proc `[]`(self:SecretKey, offset:int):uint8 {.dynlib:libpath, importc:"get_secret_key_item".}
proc createSecretKey*():seq[uint8] =
  let secretKey = createSecretKeyLib()
  defer: secretKey.dealloc()
  var s = newSeq[uint8](secretKey.len())
  for i in 0..<secretKey.len().int:
    s[i] = secretKey[i]
  return s

proc createSecretKeyHexLib():cstring {.dynlib:libpath, importc:"create_secret_key_hex".}
proc createSecretKeyHex*():string = "0x" & $createSecretKeyHexLib()

proc createVerifyingKeyLib(secret:cstring):cstring {.dynlib:libpath, importc:"create_verifying_key".}
proc createVerifyingKey*(secret:string):string =
  let secret = secret[2..^1]
  "0x" & $createVerifyingKeyLib(secret.cstring)

proc signMessageLib(key, msg:cstring):cstring {.dynlib:libpath, importc:"sign_message".}
proc signMessage*(key, msg:string):string =
  let key = key[2..^1] # 先頭の0xを削除
  return "0x" & $signMessageLib(key.cstring, msg.cstring)

proc verifySignLib(verifyKey, msg, signature:cstring):bool {.dynlib: libpath, importc:"verify_sign".}
proc verifySign*(verifyKey, msg, signature:string):bool =
  let verifyKey = verifyKey[2..^1 ]# 先頭の0xを削除
  let signature = signature[2..^1] # 先頭の0xを削除
  return verifySignLib(verifyKey.cstring, msg.cstring, signature.cstring)
