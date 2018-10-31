(module
  (func
    call 2)
  (func)
  (func)
  (export "foo" (func 0))
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func))
;;   (func (;0;) (type 0)
;;     call 1)
;;   (func (;1;) (type 0))
;;   (export "foo" (func 0)))
;; STDOUT
