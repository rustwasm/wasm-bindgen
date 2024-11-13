(module $reference_test.wasm
  (type (;0;) (func (param i32 i32) (result i32)))
  (func $add_that_might_fail (;0;) (type 0) (param i32 i32) (result i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "add_that_might_fail" (func $add_that_might_fail))
  (@custom "target_features" (after code) "\04+\0amultivalue+\0fmutable-globals+\0freference-types+\08sign-ext")
)

