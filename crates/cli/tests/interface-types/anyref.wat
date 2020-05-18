(module
  (func $foo (param externref) (result externref)
    local.get 0)

  (func $store (param externref)
    i32.const 0
    local.get 0
    table.set 0)

  (func $load (result externref)
    i32.const 0
    table.get 0)

  (table 1 externref)

  (@interface func (export "foo") (param externref) (result externref)
    arg.get 0
    call-core $foo)

  (@interface func (export "store") (param externref)
    arg.get 0
    call-core $store)

  (@interface func (export "load") (result externref)
    call-core $load)
)
