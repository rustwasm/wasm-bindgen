(module
  (start $foo)

  (func $bar)
  (func $foo)
  (func $baz)
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func))
;;   (func $foo (type 0))
;;   (start 0))
;; STDOUT
