(module $reference_test.wasm
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i32) (result i32)))
  (func $exported (;0;) (type 0) (param i32))
  (func $__wbindgen_exn_store (;1;) (type 0) (param i32))
  (func $__wbindgen_add_to_stack_pointer (;2;) (type 1) (param i32) (result i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "exported" (func $exported))
  (export "__wbindgen_exn_store" (func $__wbindgen_exn_store))
  (export "__wbindgen_add_to_stack_pointer" (func $__wbindgen_add_to_stack_pointer))
  (@custom "target_features" (after code) "\02+\0fmutable-globals+\08sign-ext")
)

