(module $reference_test.wasm
  (type (;0;) (func (result i32 i32 i32 i32)))
  (type (;1;) (func (param i32 i32 i32)))
  (func $__wbindgen_free (;0;) (type 1) (param i32 i32 i32))
  (func $"exported multivalue shim" (;1;) (type 0) (result i32 i32 i32 i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "exported" (func $"exported multivalue shim"))
  (export "__wbindgen_free" (func $__wbindgen_free))
  (@custom "target_features" (after code) "\04+\0amultivalue+\0fmutable-globals+\0freference-types+\08sign-ext")
)

