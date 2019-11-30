;; @xform import "" "a" (anyref_owned)

(module
  (import "" "a" (func $a (param i32)))
  (func (export "foo")
    i32.const 0
    call $a)
  (func $alloc (export "__wbindgen_anyref_table_alloc") (result i32)
    i32.const 0)
  (func $dealloc (export "__wbindgen_anyref_table_dealloc") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param anyref)))
  (import "" "a" (func $a (type 3)))
  (func $a anyref shim (type 2) (param i32)
    local.get 0
    table.get 0
    local.get 0
    call $dealloc
    call $a)
  (func (;2;) (type 0)
    i32.const 0
    call $a anyref shim)
  (func $alloc (type 1) (result i32)
    i32.const 0)
  (func $dealloc (type 2) (param i32))
  (table (;0;) 32 anyref)
  (export "foo" (func 2))
  (export "__wbindgen_anyref_table_alloc" (func $alloc))
  (export "__wbindgen_anyref_table_dealloc" (func $dealloc)))
;)
