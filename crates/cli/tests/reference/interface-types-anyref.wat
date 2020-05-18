(module
  (type (;0;) (func (param externref externref) (result externref)))
  (func $anyref_in_out anyref shim (type 0) (param externref externref) (result externref))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "anyref_in_out" (func $anyref_in_out anyref shim))
  (@interface type (;0;) (func (param anyref) (param anyref) (result anyref)))
  (@interface func (;0;) (type 0)
    arg.get 0
    arg.get 1
    call-core $anyref_in_out anyref shim)
  (@interface export "anyref_in_out" (func 0)))
