(module $reference_test.wasm
  (type (;0;) (func (param f64 i32 f64)))
  (type (;1;) (func (param f64 f64 f64)))
  (func $all_optional (;0;) (type 1) (param f64 f64 f64))
  (func $some_optional (;1;) (type 0) (param f64 i32 f64))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "all_optional" (func $all_optional))
  (export "some_optional" (func $some_optional))
  (@custom "target_features" (after code) "\04+\0amultivalue+\0fmutable-globals+\0freference-types+\08sign-ext")
)

