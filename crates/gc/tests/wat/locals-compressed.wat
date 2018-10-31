(module
  (func $foo
    (local i32 f32 i32 f64 i64 i32 f32 i64 i32 f32 f64)

    get_local 0
    get_local 1
    get_local 2
    get_local 3
    get_local 4
    get_local 5
    get_local 6
    get_local 7
    get_local 8
    get_local 9
    get_local 10
    unreachable
    )
  (export "foo" (func $foo))
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func))
;;   (func $foo (type 0)
;;     (local i32 i32 i32 i32 f32 f32 f32 f64 f64 i64 i64)
;;     get_local 0
;;     get_local 4
;;     get_local 1
;;     get_local 7
;;     get_local 9
;;     get_local 2
;;     get_local 5
;;     get_local 10
;;     get_local 3
;;     get_local 6
;;     get_local 8
;;     unreachable)
;;   (export "foo" (func $foo)))
;; STDOUT
