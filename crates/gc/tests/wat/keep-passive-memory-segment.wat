(module
  (memory 0 10)

  (func $foo
    i32.const 0
    i32.const 0
    i32.const 0
    memory.init 0
  )

  (data passive "wut")

  (start $foo)
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func))
;;   (func $foo (type 0)
;;     i32.const 0
;;     i32.const 0
;;     i32.const 0
;;     memory.init 0)
;;   (memory (;0;) 0 10)
;;   (start 0)
;;   (data (;0;) passive "wut"))
;; STDOUT
