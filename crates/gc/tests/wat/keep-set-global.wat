(module

  (global (mut i32) (i32.const 0))

  (start $foo)

  (func $bar)
  (func $foo
    i32.const 1
    set_global 0
    )
  (func $baz)
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func))
;;   (func $foo (type 0)
;;     i32.const 1
;;     set_global 0)
;;   (global (;0;) (mut i32) (i32.const 0))
;;   (start 0))
;; STDOUT
