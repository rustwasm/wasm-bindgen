(module
  (func $foo (result i32)
    (local i32 i32)
    get_local 1
    )
  (export "foo" (func $foo))
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func (result i32)))
;;   (func $foo (type 0) (result i32)
;;     (local i32)
;;     get_local 0)
;;   (export "foo" (func $foo)))
;; STDOUT
