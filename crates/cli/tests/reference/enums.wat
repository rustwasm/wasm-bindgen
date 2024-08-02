(module $reference_test.wasm
  (type (;0;) (func (param i32) (result i32)))
  (func $enum_echo (;0;) (type 0) (param i32) (result i32))
  (func $option_enum_echo (;1;) (type 0) (param i32) (result i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "enum_echo" (func $enum_echo))
  (export "option_enum_echo" (func $option_enum_echo))
  (@custom "target_features" (after code) "\02+\0fmutable-globals+\08sign-ext")
)

