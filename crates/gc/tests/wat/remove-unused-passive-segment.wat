(module
  (import "" "" (table 0 1 anyfunc))

  (func $foo)

  (elem passive $foo)
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module)
;; STDOUT
