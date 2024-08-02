(module $reference_test.wasm
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32 i32)))
  (func $__wbg_classbuilder_free (;0;) (type 1) (param i32 i32))
  (func $classbuilder_builder (;1;) (type 0) (result i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "__wbg_classbuilder_free" (func $__wbg_classbuilder_free))
  (export "classbuilder_builder" (func $classbuilder_builder))
  (@custom "target_features" (after code) "\02+\0fmutable-globals+\08sign-ext")
)

