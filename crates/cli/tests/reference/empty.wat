(module $reference_test.wasm
  (memory (;0;) 16)
  (export "memory" (memory 0))
  (@custom "target_features" (after export) "\04+\0amultivalue+\0fmutable-globals+\0freference-types+\08sign-ext")
)

