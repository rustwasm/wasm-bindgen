(module $reference_test.wasm
  (type (;0;) (func))
  (func $nop (;0;) (type 0))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "nop" (func $nop))
  (@custom "target_features" (after code) "\02+\0fmutable-globals+\08sign-ext")
)

