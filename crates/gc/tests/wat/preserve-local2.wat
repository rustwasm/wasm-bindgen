(module
  (func $foo (param i32)
    (local i32)
    get_local 0
    set_local 1
    )
  (export "foo" (func $foo))
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func (param i32)))
;;   (func $foo (type 0) (param i32)
;;     (local i32)
;;     get_local 0
;;     set_local 1)
;;   (export "foo" (func $foo)))
;; STDOUT
