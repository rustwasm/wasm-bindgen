;; @xform import "" "a" (externref_borrowed)

(module
  (import "" "a" (func $a (param i32)))
  (func (export "foo")
    i32.const 0
    call $a)
  (func $alloc (export "__externref_table_alloc") (result i32)
    i32.const 0)
  (func $dealloc (export "__externref_table_dealloc") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param externref)))
  (import "" "a" (func $a (type 2)))
  (func $a_externref_shim (@name "a externref shim") (type 1) (param i32)
    local.get 0
    table.get 0
    call $a)
  (func (;2;) (type 0)
    i32.const 0
    call $a_externref_shim)
  (table (;0;) 32 externref)
  (export "foo" (func 2)))
;)
