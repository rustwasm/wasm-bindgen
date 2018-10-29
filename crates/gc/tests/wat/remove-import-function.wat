(module
  (import "" "a" (func $i1))
  (import "" "b" (func $i2))
  (import "" "c" (func $i3))

  (func $bar)

  (func $foo
    call $i1
    call $i3)

  (func $baz)

  (export "foo" (func $foo))
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func))
;;   (import "" "a" (func $i1 (type 0)))
;;   (import "" "c" (func $i3 (type 0)))
;;   (func $foo (type 0)
;;     call $i1
;;     call $i3)
;;   (export "foo" (func $foo)))
;; STDOUT
