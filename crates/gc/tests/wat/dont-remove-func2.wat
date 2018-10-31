(module
  (func $foo
    call $bar
    )
  (func $bar)
  (export "foo" (func $foo))
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func))
;;   (func $foo (type 0)
;;     call $bar)
;;   (func $bar (type 0))
;;   (export "foo" (func $foo)))
;; STDOUT
