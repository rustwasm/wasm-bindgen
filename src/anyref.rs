use std::slice;
use std::vec::Vec;
use std::ptr;
use std::alloc::{self, Layout};
use std::mem;

use crate::JsValue;

externs! {
    #[link(wasm_import_module = "__wbindgen_anyref_xform__")]
    extern "C" {
        fn __wbindgen_anyref_table_grow(delta: usize) -> i32;
        fn __wbindgen_anyref_table_set_null(idx: usize) -> ();
    }
}

pub struct Slab {
    data: Vec<usize>,
    head: usize,
    base: usize,
}

impl Slab {
    fn new() -> Slab {
        Slab {
            data: Vec::new(),
            head: 0,
            base: 0,
        }
    }

    fn alloc(&mut self) -> usize {
        let ret = self.head;
        if ret == self.data.len() {
            if self.data.len() == self.data.capacity() {
                let extra = 128;
                let r = unsafe {
                    __wbindgen_anyref_table_grow(extra)
                };
                if r == -1 {
                    internal_error("table grow failure")
                }
                if self.base == 0 {
                    self.base = r as usize + (super::JSIDX_RESERVED as usize);
                } else if self.base + self.data.len() != r as usize {
                    internal_error("someone else allocated table entires?")
                }

                // poor man's `try_reserve_exact` until that's stable
                unsafe {
                    let new_cap = self.data.capacity() + extra;
                    let size = mem::size_of::<usize>() * new_cap;
                    let align = mem::align_of::<usize>();
                    let layout = match Layout::from_size_align(size, align) {
                        Ok(l) => l,
                        Err(_) => internal_error("size/align layout failure"),
                    };
                    let ptr = alloc::alloc(layout) as *mut usize;
                    if ptr.is_null() {
                        internal_error("allocation failure");
                    }
                    ptr::copy_nonoverlapping(
                        self.data.as_ptr(),
                        ptr,
                        self.data.len(),
                    );
                    let new_vec = Vec::from_raw_parts(
                        ptr,
                        self.data.len(),
                        new_cap,
                    );
                    let mut old = mem::replace(&mut self.data, new_vec);
                    old.set_len(0);
                }
            }

            // custom condition to ensure `push` below doesn't call `reserve` in
            // optimized builds which pulls in lots of panic infrastructure
            if self.data.len() >= self.data.capacity() {
                internal_error("push should be infallible now")
            }
            self.data.push(ret + 1);
        }

        // usage of `get_mut` thwarts panicking infrastructure in optimized
        // builds
        match self.data.get_mut(ret) {
            Some(slot) => self.head = *slot,
            None => internal_error("ret out of bounds"),
        }
        ret + self.base
    }

    fn dealloc(&mut self, slot: usize) {
        if slot < self.base {
            internal_error("free reserved slot");
        }
        let slot = slot - self.base;

        // usage of `get_mut` thwarts panicking infrastructure in optimized
        // builds
        match self.data.get_mut(slot) {
            Some(ptr) => {
                *ptr = self.head;
                self.head = slot;
            }
            None => internal_error("slot out of bounds"),
        }
    }

    fn live_count(&self) -> u32 {
        let mut free_count = 0;
        let mut next = self.head;
        while next < self.data.len() {
            debug_assert!((free_count as usize) < self.data.len());
            free_count += 1;
            match self.data.get(next) {
                Some(n) => next = *n,
                None => internal_error("slot out of bounds"),
            };
        }
        self.data.len() as u32 - free_count - super::JSIDX_RESERVED
    }
}

fn internal_error(msg: &str) -> ! {
    if cfg!(debug_assertions) {
        super::throw_str(msg)
    } else {
        std::process::abort()
    }
}

// Whoa, there's two `tl` modules here! That's currently intention, but for sort
// of a weird reason. The table here is fundamentally thread local, so we want
// to use the `thread_local!` macro. The implementation of thread locals (as of
// the time of this writing) generates a lot of code as it pulls in panic paths
// in libstd (even when using `try_with`). There's a patch to fix that
// (rust-lang/rust#55518), but in the meantime the stable/beta channels produce
// a lot of code.
//
// Matters are made worse here because this code is almost never used (it's only
// here for an unstable feature). If we were to have panics here, though, then
// we couldn't effectively gc away the panic infrastructure, meaning this unused
// infrastructure would show up in binaries! That's a no-no for wasm-bindgen.
//
// In the meantime, if the atomics feature is turned on (which it never is by
// default) then we use `thread_local!`, otherwise we use a home-grown
// implementation that will be replaced once #55518 lands on stable.
#[cfg(target_feature = "atomics")]
mod tl {
    use std::*; // hack to get `thread_local!` to work
    use super::Slab;
    use std::cell::Cell;

    thread_local!(pub static HEAP_SLAB: Cell<Slab> = Cell::new(Slab::new()));
}

#[cfg(not(target_feature = "atomics"))]
mod tl {
    use std::alloc::{self, Layout};
    use std::cell::Cell;
    use std::ptr;
    use super::Slab;

    pub struct HeapSlab;
    pub static HEAP_SLAB: HeapSlab = HeapSlab;
    static mut SLOT: *mut Cell<Slab> = 0 as *mut Cell<Slab>;

    impl HeapSlab {
        pub fn try_with<R>(&self, f: impl FnOnce(&Cell<Slab>) -> R) -> Result<R, ()> {
            unsafe {
                if SLOT.is_null() {
                    let ptr = alloc::alloc(Layout::new::<Cell<Slab>>());
                    if ptr.is_null() {
                        super::internal_error("allocation failure");
                    }
                    let ptr = ptr as *mut Cell<Slab>;
                    ptr::write(ptr, Cell::new(Slab::new()));
                    SLOT = ptr;
                }
                Ok(f(&*SLOT))
            }
        }
    }
}

#[no_mangle]
pub extern fn __wbindgen_anyref_table_alloc() -> usize {
    tl::HEAP_SLAB.try_with(|slot| {
        let mut slab = slot.replace(Slab::new());
        let ret = slab.alloc();
        slot.replace(slab);
        ret
    }).unwrap_or_else(|_| internal_error("tls access failure"))
}

#[no_mangle]
pub extern fn __wbindgen_anyref_table_dealloc(idx: usize) {
    if idx < super::JSIDX_RESERVED as usize {
        return
    }
    // clear this value from the table so while the table slot is un-allocated
    // we don't keep around a strong reference to a potentially large object
    unsafe {
        __wbindgen_anyref_table_set_null(idx);
    }
    tl::HEAP_SLAB.try_with(|slot| {
        let mut slab = slot.replace(Slab::new());
        slab.dealloc(idx);
        slot.replace(slab);
    }).unwrap_or_else(|_| internal_error("tls access failure"))
}

#[no_mangle]
pub unsafe extern fn __wbindgen_drop_anyref_slice(ptr: *mut JsValue, len: usize) {
    for slot in slice::from_raw_parts_mut(ptr, len) {
        __wbindgen_anyref_table_dealloc(slot.idx as usize);
    }
}

// Implementation of `__wbindgen_anyref_heap_live_count` for when we are using
// `anyref` instead of the JS `heap`.
#[no_mangle]
pub unsafe extern "C" fn __wbindgen_anyref_heap_live_count_impl() -> u32 {
    tl::HEAP_SLAB
        .try_with(|slot| {
            let slab = slot.replace(Slab::new());
            let count = slab.live_count();
            slot.replace(slab);
            count
        })
        .unwrap_or_else(|_| internal_error("tls access failure"))
}

// see comment in module above this in `link_mem_intrinsics`
#[inline(never)]
pub fn link_intrinsics() {
}
