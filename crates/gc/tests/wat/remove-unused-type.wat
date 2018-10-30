(module
  (type (func))
  (type (func (param i32)))
  (type (func (param i32)))
  (type (func (result i32)))

  (func $f1 (type 0))
  (func $f2 (type 1))
  (func $f3 (type 2))
  (func $f4 (type 3)
    i32.const 0
  )

  (export "a" (func $f1))
  (export "b" (func $f2))
  (export "c" (func $f3))
  (export "d" (func $f4))
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (type (;0;) (func))
;;   (type (;1;) (func (param i32)))
;;   (type (;2;) (func (result i32)))
;;   (func $f1 (type 0))
;;   (func $f2 (type 1) (param i32))
;;   (func $f3 (type 1) (param i32))
;;   (func $f4 (type 2) (result i32)
;;     i32.const 0)
;;   (export "a" (func $f1))
;;   (export "b" (func $f2))
;;   (export "c" (func $f3))
;;   (export "d" (func $f4)))
;; STDOUT
