(module
  (func $foo
    (local i32)
    )
  (export "foo" (func $foo))
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func))
;;   (func $foo (type 0))
;;   (export "foo" (func $foo)))
;; STDOUT
