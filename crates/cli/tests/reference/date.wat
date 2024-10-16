(module $reference_test.wasm
  (type (;0;) (func (result i64)))
  (func $date_now (;0;) (type 0) (result i64))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "date_now" (func $date_now))
  (@custom "target_features" (after code) "\02+\0fmutable-globals+\08sign-ext")
)

