(module $reference_test.wasm
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32 i32)))
  (func $__wbg_classconstructor_free (;0;) (type 1) (param i32 i32))
  (func $classconstructor_new (;1;) (type 0) (result i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "__wbg_classconstructor_free" (func $__wbg_classconstructor_free))
  (export "classconstructor_new" (func $classconstructor_new))
  (@custom "target_features" (after code) "\02+\0fmutable-globals+\08sign-ext")
)

