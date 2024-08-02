(module $reference_test.wasm
  (type (;0;) (func (param i32) (result i32)))
  (func $const_pointer (;0;) (type 0) (param i32) (result i32))
  (func $mut_pointer (;1;) (type 0) (param i32) (result i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "const_pointer" (func $const_pointer))
  (export "mut_pointer" (func $mut_pointer))
  (@custom "target_features" (after code) "\02+\0fmutable-globals+\08sign-ext")
)

