(module $reference_test.wasm
  (type (;0;) (func (param i32) (result i32)))
  (func $docme (;0;) (type 0) (param i32) (result i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "docme" (func $docme))
  (@custom "target_features" (after code) "\02+\0fmutable-globals+\08sign-ext")
)

