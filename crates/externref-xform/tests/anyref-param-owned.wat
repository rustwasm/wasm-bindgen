;; @xform export "foo" (externref_owned)

(module
  (func $foo (export "foo") (param i32))
  (func $alloc (export "__externref_table_alloc") (result i32)
    i32.const 0)
  (func $dealloc (export "__externref_table_dealloc") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param externref)))
  (func $foo_externref_shim (@name "foo externref shim") (type 2) (param externref)
    (local i32)
    call $alloc
    local.tee 1
    local.get 0
    table.set 0
    local.get 1
    call $foo)
  (func $alloc (type 0) (result i32)
    i32.const 0)
  (func $foo (type 1) (param i32))
  (table (;0;) 32 externref)
  (export "foo" (func $foo_externref_shim)))
;)
