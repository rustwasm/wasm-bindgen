(module
  (import "" "" (func (param i32)))

  (func $foo
    i32.const 0
    call 0
    )

  (start $foo)
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func (param i32)))
;;   (type (;1;) (func))
;;   (import "" "" (func (;0;) (type 0)))
;;   (func $foo (type 1)
;;     i32.const 0
;;     call 0)
;;   (start 1))
;; STDOUT
