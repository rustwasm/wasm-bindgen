(module $reference_test.wasm
  (memory (;0;) 16)
  (export "memory" (memory 0))
  (@custom "target_features" (after export) "\02+\0fmutable-globals+\08sign-ext")
)

