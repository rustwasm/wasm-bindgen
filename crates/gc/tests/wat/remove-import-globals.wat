(module
  (import "" "a" (global i32))
  (import "" "b" (global i32))
  (import "" "c" (global i32))

  (global i32 (i32.const 1))
  (global i32 (i32.const 2))

  (func $foo
    get_global 0
    drop
    get_global 2
    drop
    get_global 4
    drop
    )

  (export "foo" (func $foo))
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func))
;;   (import "" "a" (global (;0;) i32))
;;   (import "" "c" (global (;1;) i32))
;;   (func $foo (type 0)
;;     global.get 0
;;     drop
;;     global.get 1
;;     drop
;;     global.get 2
;;     drop)
;;   (global (;2;) i32 (i32.const 2))
;;   (export "foo" (func $foo)))
;; STDOUT
