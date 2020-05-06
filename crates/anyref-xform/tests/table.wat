;; @xform table 0 (anyref_owned)

(module
  (func $foo (param i32))
  (table (export "func") 0 funcref)
  (elem (i32.const 0) 0)
  (func $alloc (export "__anyref_table_alloc") (result i32)
    i32.const 0)
  (func $dealloc (export "__anyref_table_dealloc") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param anyref)))
  (func $closure0 anyref shim (type 2) (param anyref)
    (local i32)
    call $alloc
    local.tee 1
    local.get 0
    table.set 1
    local.get 1
    call $foo)
  (func $alloc (type 0) (result i32)
    i32.const 0)
  (func $foo (type 1) (param i32))
  (table (;0;) 2 funcref)
  (table (;1;) 32 anyref)
  (export "func" (table 0))
  (elem (;0;) (i32.const 0) func $foo)
  (elem (;1;) (i32.const 1) func $closure0 anyref shim))
;)
