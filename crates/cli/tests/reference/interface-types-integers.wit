(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (result f32)))
  (type (;2;) (func (result f64)))
  (type (;3;) (func (param i32 i32 i32 i32 i32 i32 f32 f64)))
  (func $integers (type 3) (param i32 i32 i32 i32 i32 i32 f32 f64))
  (func $ret_i8 (type 0) (result i32))
  (func $ret_u8 (type 0) (result i32))
  (func $ret_i16 (type 0) (result i32))
  (func $ret_u16 (type 0) (result i32))
  (func $ret_i32 (type 0) (result i32))
  (func $ret_u32 (type 0) (result i32))
  (func $ret_f32 (type 1) (result f32))
  (func $ret_f64 (type 2) (result f64))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "integers" (func $integers))
  (export "ret_i8" (func $ret_i8))
  (export "ret_u8" (func $ret_u8))
  (export "ret_i16" (func $ret_i16))
  (export "ret_u16" (func $ret_u16))
  (export "ret_i32" (func $ret_i32))
  (export "ret_u32" (func $ret_u32))
  (export "ret_f32" (func $ret_f32))
  (export "ret_f64" (func $ret_f64))
  (@interface type (;0;) (func (param u8) (param s8) (param u16) (param s16) (param u32) (param s32) (param f32) (param f64)))
  (@interface type (;1;) (func (result s8)))
  (@interface type (;2;) (func (result u8)))
  (@interface type (;3;) (func (result s16)))
  (@interface type (;4;) (func (result u16)))
  (@interface type (;5;) (func (result s32)))
  (@interface type (;6;) (func (result u32)))
  (@interface type (;7;) (func (result f32)))
  (@interface type (;8;) (func (result f64)))
  (@interface func (;0;) (type 0)
    arg.get 0
    u8-to-i32
    arg.get 1
    s8-to-i32
    arg.get 2
    u16-to-i32
    arg.get 3
    s16-to-i32
    arg.get 4
    u32-to-i32
    arg.get 5
    s32-to-i32
    arg.get 6
    arg.get 7
    call-core $integers)
  (@interface func (;1;) (type 1)
    call-core $ret_i8
    i32-to-s8)
  (@interface func (;2;) (type 2)
    call-core $ret_u8
    i32-to-u8)
  (@interface func (;3;) (type 3)
    call-core $ret_i16
    i32-to-s16)
  (@interface func (;4;) (type 4)
    call-core $ret_u16
    i32-to-u16)
  (@interface func (;5;) (type 5)
    call-core $ret_i32
    i32-to-s32)
  (@interface func (;6;) (type 6)
    call-core $ret_u32
    i32-to-u32)
  (@interface func (;7;) (type 7)
    call-core $ret_f32)
  (@interface func (;8;) (type 8)
    call-core $ret_f64)
  (@interface export "integers" (func 0))
  (@interface export "ret_i8" (func 1))
  (@interface export "ret_u8" (func 2))
  (@interface export "ret_i16" (func 3))
  (@interface export "ret_u16" (func 4))
  (@interface export "ret_i32" (func 5))
  (@interface export "ret_u32" (func 6))
  (@interface export "ret_f32" (func 7))
  (@interface export "ret_f64" (func 8)))
