;; @xform import "" "a" () anyref_owned

(module
  (import "" "a" (func $a (result i32)))
  (func (export "foo") (result i32)
    call $a)
  (func $alloc (export "__anyref_table_alloc") (result i32)
    i32.const 0)
  (func $dealloc (export "__anyref_table_dealloc") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (result anyref)))
  (import "" "a" (func $a (type 1)))
  (func $a anyref shim (type 0) (result i32)
    (local i32 anyref)
    call $a
    local.set 1
    call $alloc
    local.tee 0
    local.get 1
    table.set 0
    local.get 0)
  (func (;2;) (type 0) (result i32)
    call $a anyref shim)
  (func $alloc (type 0) (result i32)
    i32.const 0)
  (table (;0;) 32 anyref)
  (export "foo" (func 2)))
;)
