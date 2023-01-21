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

  test "verifying key":
    let secret = createSecretKeyHex()
    echo "=== secret key"
    echo secret
    echo "=== verify key"
    echo createVerifyingKey(secret)

  test "sign message":
    let msg = "Hello World"
    let secretKey = createSecretKeyHex()
    let signature = signMessage(secretKey, msg)
    echo "=== signature"
    echo signature
    let verifyKey = createVerifyingKey(secretKey)
    echo "=== verify key"
    echo verifyKey
    let isValid = verifySign(verifyKey, msg, signature)
    echo "=== expect true"
    echo isValid
    check isValid

  test "wrong message":
    let msg = "Hello World"
    let secret = createSecretKeyHex()
    let signature = signMessage(secret, msg)
    echo "=== signature"
    echo signature
    let verifyKey = createVerifyingKey(secret)
    echo "=== verify key"
    echo verifyKey
    let res = verifySign(verifyKey, "wrong hello", signature)
    echo "=== expect false"
    echo res
    check res == false

  test "wrong signature":
    let msg = "Hello World"
    let secret = createSecretKeyHex()
    let signature = signMessage(secret, msg)
    echo "=== signature"
    echo signature
    var expectWrong = verifySign("0x012345abcdef", msg, signature)
    echo "=== expect false"
    echo expectWrong
    check expectWrong == false
