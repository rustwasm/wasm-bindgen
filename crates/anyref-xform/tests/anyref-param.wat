;; @xform export "foo" (anyref_borrowed)

(module
  (func $foo (export "foo") (param i32))
  (func $alloc (export "__anyref_table_alloc") (result i32)
    i32.const 0)
  (func $dealloc (export "__anyref_table_dealloc") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param anyref)))
  (func $foo anyref shim (type 1) (param anyref)
    (local i32)
    global.get 0
    i32.const 1
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    table.set 0
    local.get 1
    call $foo
    local.get 1
    ref.null
    table.set 0
    local.get 1
    i32.const 1
    i32.add
    global.set 0)
  (func $foo (type 0) (param i32))
  (table (;0;) 32 anyref)
  (global (;0;) (mut i32) (i32.const 32))
  (export "foo" (func $foo anyref shim)))
;)
