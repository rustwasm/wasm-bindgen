(module
  (memory 1)

  (global $glen (mut i32) (i32.const 0))
  (global $gptr (mut i32) (i32.const 0))

  (func $malloc (param i32) (result i32) i32.const 23)

  (func $set (param $ptr i32) (param $len i32)
    local.get $ptr
    global.set $gptr
    local.get $len
    global.set $glen)

  (func $get (result i32 i32)
    global.get $gptr
    global.get $glen)

  (@interface func (export "set") (param string)
    arg.get 0
    string-to-memory $malloc
    call-core $set)

  (@interface func (export "get") (result string)
    call-core $get
    memory-to-string)
)
