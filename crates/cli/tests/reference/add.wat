(module $reference_test.wasm
  (type (;0;) (func (param i32 i32) (result i32)))
  (func $add_u32 (;0;) (type 0) (param i32 i32) (result i32))
  (func $add_i32 (;1;) (type 0) (param i32 i32) (result i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "add_u32" (func $add_u32))
  (export "add_i32" (func $add_i32))
  (@custom "target_features" (after code) "\04+\0amultivalue+\0fmutable-globals+\0freference-types+\08sign-ext")
)

