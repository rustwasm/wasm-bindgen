//! See <https://github.com/alexcrichton/scoped-tls/blob/0b6f66a582340bb3363f21a84b70ff40caa98774/src/lib.rs>.

use core::cell::Cell;
use core::marker::PhantomData;

/// `no_std` polyfill for [`scoped_tls`](https://crates.io/crates/scoped-tls).
#[macro_export]
macro_rules! scoped_thread_local {
    (static $name:ident: $ty:ty) => {
        static $name: scoped_tls::ScopedKey<$ty> = unsafe {
            static FOO: scoped_tls::Wrapper<::core::cell::Cell<*const ()>> =
                scoped_tls::Wrapper::new(::core::cell::Cell::new(::core::ptr::null()));
            // Safety: nothing else can access FOO since it's hidden in its own scope
            scoped_tls::ScopedKey::new(&FOO)
        };
    };
}

pub(super) struct Wrapper<T>(T);

impl<T> Wrapper<T> {
    pub(super) const fn new(value: T) -> Self {
        Self(value)
    }
}

unsafe impl<T> Sync for Wrapper<T> {}

pub struct ScopedKey<T> {
    inner: &'static Wrapper<Cell<*const ()>>,
    _marker: PhantomData<T>,
}

unsafe impl<T> Sync for ScopedKey<T> {}

impl<T> ScopedKey<T> {
    #[doc(hidden)]
    /// # Safety
    /// `inner` must only be accessed through `ScopedKey`'s API
    pub(super) const unsafe fn new(inner: &'static Wrapper<Cell<*const ()>>) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    pub fn set<F, R>(&'static self, t: &T, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        struct Reset {
            key: &'static Wrapper<Cell<*const ()>>,
            val: *const (),
        }
        impl Drop for Reset {
            fn drop(&mut self) {
                self.key.0.set(self.val);
            }
        }
        let prev = self.inner.0.get();
        self.inner.0.set(t as *const T as *const ());
        let _reset = Reset {
            key: self.inner,
            val: prev,
        };
        f()
    }

    pub fn with<F, R>(&'static self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let val = self.inner.0.get();
        assert!(
            !val.is_null(),
            "cannot access a scoped thread local variable without calling `set` first"
        );
        unsafe { f(&*(val as *const T)) }
    }

    /// Test whether this TLS key has been `set` for the current thread.
    pub fn is_set(&'static self) -> bool {
        !self.inner.0.get().is_null()
    }
}
