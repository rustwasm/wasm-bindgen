(module
  (func $foo
    i32.const 0
    call_indirect
    )

  (func $bar)

  (table 0 10 anyfunc)
  (elem (i32.const 0) $bar)

  (export "foo" (func $foo))
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func))
;;   (func $foo (type 0)
;;     i32.const 0
;;     call_indirect (type 0))
;;   (func $bar (type 0))
;;   (table (;0;) 0 10 anyfunc)
;;   (export "foo" (func $foo))
;;   (elem (;0;) (i32.const 0) $bar))
;; STDOUT
