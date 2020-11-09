;; @xform import "" "a" () externref_owned

(module
  (import "" "a" (func $a (result i32)))
  (func (export "foo") (result i32)
    call $a)
  (func $alloc (export "__externref_table_alloc") (result i32)
    i32.const 0)
  (func $dealloc (export "__externref_table_dealloc") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (result externref)))
  (import "" "a" (func $a (type 1)))
  (func $a_externref_shim (@name "a externref shim") (type 0) (result i32)
    (local i32 externref)
    call $a
    local.set 1
    call $alloc
    local.tee 0
    local.get 1
    table.set 0
    local.get 0)
  (func (;2;) (type 0) (result i32)
    call $a_externref_shim)
  (func $alloc (type 0) (result i32)
    i32.const 0)
  (table (;0;) 32 externref)
  (export "foo" (func 2)))
;)
