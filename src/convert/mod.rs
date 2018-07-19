//! This is mostly an internal module, no stability guarantees are provided. Use
//! at your own risk.

mod traits;
mod impls;
mod slices;
mod closures;

pub use self::slices::WasmSlice;
pub use self::traits::*;

pub struct GlobalStack {
    next: usize,
}

impl GlobalStack {
    #[inline]
    pub unsafe fn new() -> GlobalStack {
        GlobalStack { next: 0 }
    }
}

impl Stack for GlobalStack {
    #[inline]
    fn push(&mut self, val: u32) {
        use __rt::{
            __wbindgen_global_argument_ptr as global_ptr,
            GLOBAL_STACK_CAP,
        };
        unsafe {
            assert!(self.next < GLOBAL_STACK_CAP);
            *global_ptr().offset(self.next as isize) = val;
            self.next += 1;
        }
    }
}
