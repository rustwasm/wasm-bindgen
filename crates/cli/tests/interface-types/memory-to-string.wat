(module
  (memory 1)

  (func $foo (result i32 i32)
    i32.const 0
    i32.const 3)
  (func $hexa (result i32 i32)
    i32.const 10
    i32.const 4)

  (data (i32.const 0) "foo")
  (data (i32.const 10) "hexa")

  (@interface func (export "foo") (result string)
    call-core $foo
    memory-to-string)

  (@interface func (export "hexa") (result string)
    call-core $hexa
    memory-to-string)
)
