(module
  (type (;0;) (func (param i32) (result i32)))
  (func $take_and_return (type 0) (param i32) (result i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "take_and_return" (func $take_and_return))
  (@interface type (;0;) (func (param u8) (result u16)))
  (@interface func (;0;) (type 0)
    arg.get 0
    u8-to-i32
    call-core $take_and_return
    i32-to-u16)
  (@interface export "take_and_return" (func 0)))
