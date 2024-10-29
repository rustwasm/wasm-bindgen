(module $reference_test.wasm
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32 i32)))
  (type (;4;) (func (param i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
  (func $__wbindgen_realloc (;0;) (type 6) (param i32 i32 i32 i32) (result i32))
  (func $__wbindgen_malloc (;1;) (type 4) (param i32 i32) (result i32))
  (func $__wbg_set_foo_y (;2;) (type 5) (param i32 i32 i32))
  (func $__wbg_get_foo_y (;3;) (type 3) (param i32 i32))
  (func $foo_z (;4;) (type 3) (param i32 i32))
  (func $foo_lone_getter (;5;) (type 3) (param i32 i32))
  (func $foo_set_weird (;6;) (type 5) (param i32 i32 i32))
  (func $foo_set_z (;7;) (type 5) (param i32 i32 i32))
  (func $foo_set_lone_setter (;8;) (type 5) (param i32 i32 i32))
  (func $__wbg_get_foo_x (;9;) (type 2) (param i32) (result i32))
  (func $__wbg_set_foo_x (;10;) (type 3) (param i32 i32))
  (func $foo_weird (;11;) (type 2) (param i32) (result i32))
  (func $foo_x_static (;12;) (type 0) (result i32))
  (func $__wbg_foo_free (;13;) (type 3) (param i32 i32))
  (func $foo_set_x_static (;14;) (type 1) (param i32))
  (func $__wbindgen_add_to_stack_pointer (;15;) (type 2) (param i32) (result i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "__wbg_foo_free" (func $__wbg_foo_free))
  (export "__wbg_get_foo_x" (func $__wbg_get_foo_x))
  (export "__wbg_set_foo_x" (func $__wbg_set_foo_x))
  (export "__wbg_get_foo_y" (func $__wbg_get_foo_y))
  (export "__wbg_set_foo_y" (func $__wbg_set_foo_y))
  (export "foo_z" (func $foo_z))
  (export "foo_set_z" (func $foo_set_z))
  (export "foo_lone_getter" (func $foo_lone_getter))
  (export "foo_set_lone_setter" (func $foo_set_lone_setter))
  (export "foo_weird" (func $foo_weird))
  (export "foo_set_weird" (func $foo_set_weird))
  (export "foo_x_static" (func $foo_x_static))
  (export "foo_set_x_static" (func $foo_set_x_static))
  (export "__wbindgen_add_to_stack_pointer" (func $__wbindgen_add_to_stack_pointer))
  (export "__wbindgen_malloc" (func $__wbindgen_malloc))
  (export "__wbindgen_realloc" (func $__wbindgen_realloc))
  (@custom "target_features" (after code) "\02+\0fmutable-globals+\08sign-ext")
)

