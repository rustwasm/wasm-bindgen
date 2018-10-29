(module
  (import "" "" (table 0 1 anyfunc))

  (func $foo
    i32.const 0
    i32.const 0
    i32.const 0
    table.init 1
  )

  (func $bar)
  (func $bar2)

  (elem passive $bar)
  (elem passive $bar2)

  (start $foo)
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func))
;;   (import "" "" (table (;0;) 0 1 anyfunc))
;;   (func $foo (type 0)
;;     i32.const 0
;;     i32.const 0
;;     i32.const 0
;;     table.init 0)
;;   (func $bar2 (type 0))
;;   (start 0)
;;   (elem (;0;) passive $bar2))
;; STDOUT
