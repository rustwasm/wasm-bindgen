(module $reference_test.wasm
  (type (;0;) (func (param i32) (result i32)))
  (func $docme (;0;) (type 0) (param i32) (result i32))
  (func $i_has_docs (;1;) (type 0) (param i32) (result i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "docme" (func $docme))
  (export "i_has_docs" (func $i_has_docs))
  (@custom "target_features" (after code) "\04+\0amultivalue+\0fmutable-globals+\0freference-types+\08sign-ext")
)

