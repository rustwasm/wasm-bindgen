#![feature(use_extern_macros)]

extern crate wasm_bindgen_macro;

use std::mem;

pub mod prelude {
    pub use wasm_bindgen_macro::wasm_bindgen;
    pub use JsObject;
}

pub struct JsObject {
    idx: u32,
}

impl JsObject {
    #[doc(hidden)]
    pub fn __from_idx(idx: u32) -> JsObject {
        JsObject { idx }
    }

    #[doc(hidden)]
    pub fn __get_idx(&self) -> u32 {
        self.idx
    }

    #[doc(hidden)]
    pub fn __into_idx(self) -> u32 {
        let ret = self.idx;
        mem::forget(self);
        return ret
    }
}

extern {
    fn __wbindgen_object_clone_ref(idx: u32) -> u32;
    fn __wbindgen_object_drop_ref(idx: u32);
}

impl Clone for JsObject {
    fn clone(&self) -> JsObject {
        unsafe {
            let idx = __wbindgen_object_clone_ref(self.idx);
            JsObject { idx }
        }
    }
}

impl Drop for JsObject {
    fn drop(&mut self) {
        unsafe {
            __wbindgen_object_drop_ref(self.idx);
        }
    }
}

#[cold]
pub fn throw(s: &str) -> ! {
    extern {
        fn __wbindgen_throw(a: *const u8, b: usize) -> !;
    }
    unsafe {
        __wbindgen_throw(s.as_ptr(), s.len());
    }
}

#[doc(hidden)]
pub mod __rt {
    use std::cell::{Cell, UnsafeCell};
    use std::ops::{Deref, DerefMut};

    #[inline]
    pub fn assert_not_null<T>(s: *mut T) {
        if s.is_null() {
            super::throw("null pointer passed to rust");
        }
    }

    /// A vendored version of `RefCell` from the standard library.
    ///
    /// Now why, you may ask, would we do that? Surely `RefCell` in libstd is
    /// quite good. And you're right, it is indeed quite good! Functionally
    /// nothing more is needed from `RefCell` in the standard library but for
    /// now this crate is also sort of optimizing for compiled code size.
    ///
    /// One major factor to larger binaries in Rust is when a panic happens.
    /// Panicking in the standard library involves a fair bit of machinery
    /// (formatting, panic hooks, synchronization, etc). It's all worthwhile if
    /// you need it but for something like `WasmRefCell` here we don't actually
    /// need all that!
    ///
    /// This is just a wrapper around all Rust objects passed to JS intended to
    /// guard accidental reentrancy, so this vendored version is intended solely
    /// to not panic in libstd. Instead when it "panics" it calls our `throw`
    /// function in this crate which raises an error in JS.
    pub struct WasmRefCell<T> {
        borrow: Cell<usize>,
        value: UnsafeCell<T>,
    }

    impl<T> WasmRefCell<T> {
        pub fn new(value: T) -> WasmRefCell<T> {
            WasmRefCell {
                value: UnsafeCell::new(value),
                borrow: Cell::new(0),
            }
        }

        pub fn borrow(&self) -> Ref<T> {
            unsafe {
                if self.borrow.get() == usize::max_value() {
                    borrow_fail();
                }
                self.borrow.set(self.borrow.get() + 1);
                Ref {
                    value: &*self.value.get(),
                    borrow: &self.borrow,
                }
            }
        }

        pub fn borrow_mut(&self) -> RefMut<T> {
            unsafe {
                if self.borrow.get() != 0 {
                    borrow_fail();
                }
                self.borrow.set(usize::max_value());
                RefMut {
                    value: &mut *self.value.get(),
                    borrow: &self.borrow,
                }
            }
        }

        pub fn into_inner(self) -> T {
            unsafe {
                self.value.into_inner()
            }
        }
    }

    pub struct Ref<'b, T: 'b> {
        value: &'b T,
        borrow: &'b Cell<usize>,
    }

    impl<'b, T> Deref for Ref<'b, T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            self.value
        }
    }

    impl<'b, T> Drop for Ref<'b, T> {
        fn drop(&mut self) {
            self.borrow.set(self.borrow.get() - 1);
        }
    }

    pub struct RefMut<'b, T: 'b> {
        value: &'b mut T,
        borrow: &'b Cell<usize>,
    }

    impl<'b, T> Deref for RefMut<'b, T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            self.value
        }
    }

    impl<'b, T> DerefMut for RefMut<'b, T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut T {
            self.value
        }
    }

    impl<'b, T> Drop for RefMut<'b, T> {
        fn drop(&mut self) {
            self.borrow.set(0);
        }
    }

    fn borrow_fail() -> ! {
        super::throw("recursive use of an object detected which would lead to \
                      unsafe aliasing in rust");
    }
}
