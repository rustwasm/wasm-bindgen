;; @xform export "foo" (externref_owned) externref_owned

(module
  (import "__wbindgen_placeholder__" "__wbindgen_object_clone_ref"
    (func $clone (param i32) (result i32)))
  (func $foo (export "foo") (param i32) (result i32)
    local.get 0
    call $clone)
  (func $alloc (export "__externref_table_alloc") (result i32)
    i32.const 0)
  (func $dealloc (export "__externref_table_dealloc") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param externref) (result externref)))
  (func $foo_externref_shim (@name "foo externref shim") (type 3) (param externref) (result externref)
    (local i32)
    call $alloc
    local.tee 1
    local.get 0
    table.set 0
    local.get 1
    call $foo
    local.tee 1
    table.get 0
    local.get 1
    call $dealloc)
  (func $__wbindgen_object_clone_ref (type 2) (param i32) (result i32)
    (local i32)
    call $alloc
    local.tee 1
    local.get 0
    table.get 0
    table.set 0
    local.get 1)
  (func $foo (type 2) (param i32) (result i32)
    local.get 0
    call $__wbindgen_object_clone_ref)
  (func $alloc (type 0) (result i32)
    i32.const 0)
  (func $dealloc (type 1) (param i32))
  (table (;0;) 32 externref)
  (export "foo" (func $foo_externref_shim)))
;)
