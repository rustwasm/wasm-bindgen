(module
  (type (;0;) (func (param i32) (result i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32 i32) (result i32 i32)))
  (type (;3;) (func (param i32 i32 i32 i32)))
  (func $__wbindgen_malloc (type 0) (param i32) (result i32))
  (func $many_strings (type 3) (param i32 i32 i32 i32))
  (func $__wbindgen_free (type 1) (param i32 i32))
  (func $strings multivalue shim (type 2) (param i32 i32) (result i32 i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "strings" (func $strings multivalue shim))
  (export "many_strings" (func $many_strings))
  (@interface type (;0;) (func (param string) (result string)))
  (@interface type (;1;) (func (param string) (param string)))
  (@interface func (;0;) (type 0)
    arg.get 0
    string-to-memory $__wbindgen_malloc
    call-core $strings multivalue shim
    defer-call-core $__wbindgen_free
    memory-to-string)
  (@interface func (;1;) (type 1)
    arg.get 0
    string-to-memory $__wbindgen_malloc
    arg.get 1
    string-to-memory $__wbindgen_malloc
    call-core $many_strings)
  (@interface export "strings" (func 0))
  (@interface export "many_strings" (func 1)))
