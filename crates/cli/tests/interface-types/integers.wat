(module
  (func $add_i32 (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add)

  (@interface func (export "add_i8") (param s8) (param s8) (result s8)
    arg.get 0
    s8-to-i32
    arg.get 1
    s8-to-i32
    call-core $add_i32
    i32-to-s8)

  (@interface func (export "add_i16") (param s16) (param s16) (result s16)
    arg.get 0
    s16-to-i32
    arg.get 1
    s16-to-i32
    call-core $add_i32
    i32-to-s16)

  (@interface func (export "add_i32") (param s32) (param s32) (result s32)
    arg.get 0
    s32-to-i32
    arg.get 1
    s32-to-i32
    call-core $add_i32
    i32-to-s32)

  (@interface func (export "add_u8") (param s8) (param s8) (result s8)
    arg.get 0
    s8-to-i32
    arg.get 1
    s8-to-i32
    call-core $add_i32
    i32-to-s8)

  (@interface func (export "add_u16") (param u16) (param u16) (result u16)
    arg.get 0
    u16-to-i32
    arg.get 1
    u16-to-i32
    call-core $add_i32
    i32-to-u16)

  (@interface func (export "add_u32") (param u32) (param u32) (result u32)
    arg.get 0
    u32-to-i32
    arg.get 1
    u32-to-i32
    call-core $add_i32
    i32-to-u32)
)
