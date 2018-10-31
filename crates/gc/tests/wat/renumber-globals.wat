(module
  (global i32 (i32.const 0))
  (global i32 (i32.const 0))
  (func (result i32)
    get_global 1)
  (export "foo" (func 0))
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func (result i32)))
;;   (func (;0;) (type 0) (result i32)
;;     get_global 0)
;;   (global (;0;) i32 (i32.const 0))
;;   (export "foo" (func 0)))
;; STDOUT
