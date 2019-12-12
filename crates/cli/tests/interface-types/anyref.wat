(module
  (func $foo (param anyref) (result anyref)
    local.get 0)

  (func $store (param anyref)
    i32.const 0
    local.get 0
    table.set 0)

  (func $load (result anyref)
    i32.const 0
    table.get 0)

  (table 1 anyref)

  (@interface func (export "foo") (param anyref) (result anyref)
    arg.get 0
    call-core $foo)

  (@interface func (export "store") (param anyref)
    arg.get 0
    call-core $store)

  (@interface func (export "load") (result anyref)
    call-core $load)
)
