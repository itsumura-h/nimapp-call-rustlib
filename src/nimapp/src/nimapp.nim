import ./rustlib


when isMainModule:
  echo add(1, 2)
  echo fib(10)
 
  var res = newSeq[int]()
  for i in 1..30:
    res.add(fib(i))
  echo res

  let person = Person.new(1, "John")
  echo person.repr
  echo person.id()
  echo person.name()
