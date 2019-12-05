(module
  (global $ctr (mut i32) (i32.const 0))

  (func $increment
    global.get $ctr
    i32.const 1
    i32.add
    global.set $ctr)

  (func $get (result i32)
    global.get $ctr)

  (@interface func (export "foo") (result s32)
    defer-call-core $increment
    call-core $get
    i32-to-s32)

  (@interface func (export "get") (result s32)
    call-core $get
    i32-to-s32)
)
