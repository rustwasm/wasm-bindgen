(module
  (import "" "" (table 0 1 anyfunc))

  (func $foo)

  (elem (i32.const 1) $foo)
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module)
;; STDOUT
