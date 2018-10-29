(module
  (func $foo
    i32.const 0
    call_indirect
    )

  (func $bar)

  (table 0 10 anyfunc)
  (elem (i32.const 0) $bar)
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module)
;; STDOUT
