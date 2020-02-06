(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func $add_u32 (type 0) (param i32 i32) (result i32))
  (func $add_i32 (type 0) (param i32 i32) (result i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "add_u32" (func $add_u32))
  (export "add_i32" (func $add_i32))
  (@interface type (;0;) (func (param u32) (param u32) (result u32)))
  (@interface type (;1;) (func (param s32) (param s32) (result s32)))
  (@interface func (;0;) (type 0)
    arg.get 0
    u32-to-i32
    arg.get 1
    u32-to-i32
    call-core $add_u32
    i32-to-u32)
  (@interface func (;1;) (type 1)
    arg.get 0
    s32-to-i32
    arg.get 1
    s32-to-i32
    call-core $add_i32
    i32-to-s32)
  (@interface export "add_u32" (func 0))
  (@interface export "add_i32" (func 1)))
