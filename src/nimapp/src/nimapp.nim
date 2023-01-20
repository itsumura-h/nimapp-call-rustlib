import std/strutils
import std/unittest
import ./rustlib


suite "test":
  test "add":
    echo add(1, 2)
    check add(1, 2) == 3

  test "recieveStrThenReturn":
    let str = "Hello World"
    let resp = recieveStrThenReturn(str)
    echo resp

  test "fib array":
    let res = fibArray(10)
    echo res
    check res == @[0, 1, 1, 2, 3, 5, 8, 13, 21, 34]


suite "object":
  test "person":
    let person = Person.new(1, "John")
    echo person.repr
    echo person.id()
    echo person.name()
    check:
      person.id() == 1
      person.name() == "John"

  test "updatable person":
    let person = UpdatablePerson.new(1, "John")
    echo person.repr
    echo person.id()
    echo person.name()
    check:
      person.id() == 1
      person.name() == "John"
    
    person.setId(2)
    person.setName("Paul")
    echo person.repr
    echo person.id()
    echo person.name()
    check:
      person.id() == 2
      person.name() == "Paul"

suite "crypto":
  test "secret key":
    let secretKey = createSecretKey()
    echo secretKey

  test "hex key":
    let key = createSecretKeyHex()
    echo key

  test "hex key to seq":
    let key = createSecretKeyHex()
    echo key
    let s = hexKeyToSeq($key)
    echo s

  test "sign message":
    let msg = "Hello World"
    let key = createSecretKeyHex()
    echo "=== key"
    echo key
    let signature = signMessage(msg, key)
    echo "=== signature"
    echo signature
    let isValid = verifySign(key, msg, signature)
    echo "=== expect true"
    echo isValid
    let expectWrong = verifySign(key, "wrong", signature)
    echo "=== expect false"
    echo expectWrong
