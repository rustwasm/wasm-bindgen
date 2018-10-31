(module
  (memory 0 10)

  (func $foo
    i32.const 0
    i32.const 0
    i32.const 0
    memory.init 0
  )

  (data passive "wut")

  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (memory (;0;) 0 10))
;; STDOUT
