(module
  (type (;0;) (func (param externref externref) (result externref)))
  (func $externref_in_out externref shim (type 0) (param externref externref) (result externref))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "externref_in_out" (func $externref_in_out externref shim))
  (@interface type (;0;) (func (param externref) (param externref) (result externref)))
  (@interface func (;0;) (type 0)
    arg.get 0
    arg.get 1
    call-core $externref_in_out externref shim)
  (@interface export "externref_in_out" (func 0)))
