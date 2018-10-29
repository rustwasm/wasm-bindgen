(module
  (memory 0 1)
  (data (i32.const 0) "foo")
  )

;; STDOUT (update this section with `BLESS_TESTS=1` while running tests)
;; (module
;;   (memory (;0;) 0 1)
;;   (data (;0;) (i32.const 0) "foo"))
;; STDOUT
