(module
  (@interface func (export "nop"))
  (@interface func (export "roundtrip") (param s32) (result s32)
    arg.get 0)
)
