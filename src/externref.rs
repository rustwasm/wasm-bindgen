use crate::JsValue;

use alloc::slice;
use alloc::vec::Vec;
use core::cell::Cell;
use core::cmp::max;

externs! {
    #[link(wasm_import_module = "__wbindgen_externref_xform__")]
    extern "C" {
        fn __wbindgen_externref_table_grow(delta: usize) -> i32;
        fn __wbindgen_externref_table_set_null(idx: usize) -> ();
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
            let curr_len = self.data.len();
            if curr_len == self.data.capacity() {
                let extra = max(128, curr_len);
                let r = unsafe { __wbindgen_externref_table_grow(extra) };
                if r == -1 {
                    internal_error("table grow failure")
                }
                if self.base == 0 {
                    self.base = r as usize;
                } else if self.base + self.data.len() != r as usize {
                    internal_error("someone else allocated table entries?")
                }

                if self.data.try_reserve_exact(extra).is_err() {
                    internal_error("allocation failure");
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
        self.data.len() as u32 - free_count
    }
}

fn internal_error(msg: &str) -> ! {
    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            super::throw_str(msg)
        } else if #[cfg(feature = "std")] {
            std::process::abort();
        } else if #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))] {
            core::arch::wasm32::unreachable();
        } else {
            unreachable!()
        }
    }
}

// Management of `externref` is always thread local since an `externref` value
// can't cross threads in wasm. Indices as a result are always thread-local.
std::thread_local!(pub static HEAP_SLAB: Cell<Slab> = Cell::new(Slab::new()));

#[no_mangle]
pub extern "C" fn __externref_table_alloc() -> usize {
    HEAP_SLAB
        .try_with(|slot| {
            let mut slab = slot.replace(Slab::new());
            let ret = slab.alloc();
            slot.replace(slab);
            ret
        })
        .unwrap_or_else(|_| internal_error("tls access failure"))
}

#[no_mangle]
pub extern "C" fn __externref_table_dealloc(idx: usize) {
    if idx < super::JSIDX_RESERVED as usize {
        return;
    }
    // clear this value from the table so while the table slot is un-allocated
    // we don't keep around a strong reference to a potentially large object
    unsafe {
        __wbindgen_externref_table_set_null(idx);
    }
    HEAP_SLAB
        .try_with(|slot| {
            let mut slab = slot.replace(Slab::new());
            slab.dealloc(idx);
            slot.replace(slab);
        })
        .unwrap_or_else(|_| internal_error("tls access failure"))
}

#[no_mangle]
pub unsafe extern "C" fn __externref_drop_slice(ptr: *mut JsValue, len: usize) {
    for slot in slice::from_raw_parts_mut(ptr, len) {
        __externref_table_dealloc(slot.idx as usize);
    }
}

// Implementation of `__wbindgen_externref_heap_live_count` for when we are using
// `externref` instead of the JS `heap`.
#[no_mangle]
pub unsafe extern "C" fn __externref_heap_live_count() -> u32 {
    HEAP_SLAB
        .try_with(|slot| {
            let slab = slot.replace(Slab::new());
            let count = slab.live_count();
            slot.replace(slab);
            count
        })
        .unwrap_or_else(|_| internal_error("tls access failure"))
}
