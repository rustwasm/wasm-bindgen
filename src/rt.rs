use crate::JsValue;
use core::borrow::{Borrow, BorrowMut};
use core::cell::{Cell, UnsafeCell};
use core::convert::Infallible;
use core::mem;
use core::ops::{Deref, DerefMut};
#[cfg(all(target_feature = "atomics", not(feature = "std")))]
use core::sync::atomic::{AtomicU8, Ordering};

pub extern crate alloc;
pub extern crate core;
#[cfg(feature = "std")]
pub extern crate std;

use alloc::alloc::{alloc, dealloc, realloc, Layout};
use alloc::boxed::Box;
use alloc::rc::Rc;

/// Wrapper around [`::once_cell::unsync::Lazy`] adding some compatibility methods with
/// [`std::thread::LocalKey`] and adding `Send + Sync` when `atomics` is not enabled.
#[cfg(not(feature = "std"))]
pub struct LazyCell<T, F = fn() -> T>(::once_cell::unsync::Lazy<T, F>);

#[cfg(all(not(target_feature = "atomics"), not(feature = "std")))]
unsafe impl<T, F> Sync for LazyCell<T, F> {}

#[cfg(all(not(target_feature = "atomics"), not(feature = "std")))]
unsafe impl<T, F> Send for LazyCell<T, F> {}

#[cfg(not(feature = "std"))]
impl<T, F> LazyCell<T, F> {
    pub const fn new(init: F) -> LazyCell<T, F> {
        Self(::once_cell::unsync::Lazy::new(init))
    }
}

#[cfg(not(feature = "std"))]
impl<T, F: FnOnce() -> T> LazyCell<T, F> {
    pub(crate) fn try_with<R>(
        &self,
        f: impl FnOnce(&T) -> R,
    ) -> Result<R, core::convert::Infallible> {
        Ok(f(&self.0))
    }

    pub fn force(this: &Self) -> &T {
        &this.0
    }
}

#[cfg(not(feature = "std"))]
impl<T> Deref for LazyCell<T> {
    type Target = T;

    fn deref(&self) -> &T {
        ::once_cell::unsync::Lazy::force(&self.0)
    }
}

#[cfg(feature = "std")]
pub use once_cell::sync::Lazy as LazyLock;

#[cfg(all(not(target_feature = "atomics"), not(feature = "std")))]
pub use LazyCell as LazyLock;

#[cfg(all(target_feature = "atomics", not(feature = "std")))]
pub struct LazyLock<T, F = fn() -> T> {
    state: AtomicU8,
    data: UnsafeCell<Data<T, F>>,
}

#[cfg(all(target_feature = "atomics", not(feature = "std")))]
enum Data<T, F> {
    Value(T),
    Init(F),
}

#[cfg(all(target_feature = "atomics", not(feature = "std")))]
unsafe impl<T, F> Sync for LazyLock<T, F> {}

#[cfg(all(target_feature = "atomics", not(feature = "std")))]
unsafe impl<T, F> Send for LazyLock<T, F> {}

#[cfg(all(target_feature = "atomics", not(feature = "std")))]
impl<T, F> LazyLock<T, F> {
    const STATE_UNINIT: u8 = 0;
    const STATE_INITIALIZING: u8 = 1;
    const STATE_INIT: u8 = 2;

    pub const fn new(init: F) -> LazyLock<T, F> {
        Self {
            state: AtomicU8::new(Self::STATE_UNINIT),
            data: UnsafeCell::new(Data::Init(init)),
        }
    }
}

#[cfg(all(target_feature = "atomics", not(feature = "std")))]
impl<T> Deref for LazyLock<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let mut state = self.state.load(Ordering::Acquire);

        loop {
            match state {
                Self::STATE_INIT => {
                    let Data::Value(value) = (unsafe { &*self.data.get() }) else {
                        unreachable!()
                    };
                    return value;
                }
                Self::STATE_UNINIT => {
                    if let Err(new_state) = self.state.compare_exchange_weak(
                        Self::STATE_UNINIT,
                        Self::STATE_INITIALIZING,
                        Ordering::Acquire,
                        Ordering::Relaxed,
                    ) {
                        state = new_state;
                        continue;
                    }

                    let data = unsafe { &mut *self.data.get() };
                    let Data::Init(init) = data else {
                        unreachable!()
                    };
                    *data = Data::Value(init());
                    self.state.store(Self::STATE_INIT, Ordering::Release);
                    state = Self::STATE_INIT;
                }
                Self::STATE_INITIALIZING => {
                    // TODO: Block here if possible. This would require
                    // detecting if we can in the first place.
                    state = self.state.load(Ordering::Acquire);
                }
                _ => unreachable!(),
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
#[cfg(all(not(feature = "std"), not(target_feature = "atomics")))]
macro_rules! __wbindgen_thread_local {
    ($wasm_bindgen:tt, $actual_ty:ty) => {{
        static _VAL: $wasm_bindgen::__rt::LazyCell<$actual_ty> =
            $wasm_bindgen::__rt::LazyCell::new(init);
        $wasm_bindgen::JsThreadLocal { __inner: &_VAL }
    }};
}

#[macro_export]
#[doc(hidden)]
#[cfg(all(not(feature = "std"), target_feature = "atomics"))]
#[allow_internal_unstable(thread_local)]
macro_rules! __wbindgen_thread_local {
    ($wasm_bindgen:tt, $actual_ty:ty) => {{
        #[thread_local]
        static _VAL: $wasm_bindgen::__rt::LazyCell<$actual_ty> =
            $wasm_bindgen::__rt::LazyCell::new(init);
        $wasm_bindgen::JsThreadLocal {
            __inner: || unsafe { $wasm_bindgen::__rt::LazyCell::force(&_VAL) as *const $actual_ty },
        }
    }};
}

#[macro_export]
#[doc(hidden)]
#[cfg(not(wasm_bindgen_unstable_test_coverage))]
macro_rules! __wbindgen_coverage {
    ($item:item) => {
        $item
    };
}

#[macro_export]
#[doc(hidden)]
#[cfg(wasm_bindgen_unstable_test_coverage)]
#[allow_internal_unstable(coverage_attribute)]
macro_rules! __wbindgen_coverage {
    ($item:item) => {
        #[coverage(off)]
        $item
    };
}

#[inline]
pub fn assert_not_null<T>(s: *mut T) {
    if s.is_null() {
        throw_null();
    }
}

#[cold]
#[inline(never)]
fn throw_null() -> ! {
    super::throw_str("null pointer passed to rust");
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
pub struct WasmRefCell<T: ?Sized> {
    borrow: Cell<usize>,
    value: UnsafeCell<T>,
}

impl<T: ?Sized> WasmRefCell<T> {
    pub fn new(value: T) -> WasmRefCell<T>
    where
        T: Sized,
    {
        WasmRefCell {
            value: UnsafeCell::new(value),
            borrow: Cell::new(0),
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }

    pub fn borrow(&self) -> Ref<T> {
        unsafe {
            if self.borrow.get() == usize::MAX {
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
            self.borrow.set(usize::MAX);
            RefMut {
                value: &mut *self.value.get(),
                borrow: &self.borrow,
            }
        }
    }

    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        self.value.into_inner()
    }
}

pub struct Ref<'b, T: ?Sized + 'b> {
    value: &'b T,
    borrow: &'b Cell<usize>,
}

impl<T: ?Sized> Deref for Ref<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.value
    }
}

impl<T: ?Sized> Borrow<T> for Ref<'_, T> {
    #[inline]
    fn borrow(&self) -> &T {
        self.value
    }
}

impl<T: ?Sized> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        self.borrow.set(self.borrow.get() - 1);
    }
}

pub struct RefMut<'b, T: ?Sized + 'b> {
    value: &'b mut T,
    borrow: &'b Cell<usize>,
}

impl<T: ?Sized> Deref for RefMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.value
    }
}

impl<T: ?Sized> DerefMut for RefMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<T: ?Sized> Borrow<T> for RefMut<'_, T> {
    #[inline]
    fn borrow(&self) -> &T {
        self.value
    }
}

impl<T: ?Sized> BorrowMut<T> for RefMut<'_, T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<T: ?Sized> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        self.borrow.set(0);
    }
}

fn borrow_fail() -> ! {
    super::throw_str(
        "recursive use of an object detected which would lead to \
		 unsafe aliasing in rust",
    );
}

/// A type that encapsulates an `Rc<WasmRefCell<T>>` as well as a `Ref`
/// to the contents of that `WasmRefCell`.
///
/// The `'static` requirement is an unfortunate consequence of how this
/// is implemented.
pub struct RcRef<T: ?Sized + 'static> {
    // The 'static is a lie.
    //
    // We could get away without storing this, since we're in the same module as
    // `WasmRefCell` and can directly manipulate its `borrow`, but I'm considering
    // turning it into a wrapper around `std`'s `RefCell` to reduce `unsafe` in
    // which case that would stop working. This also requires less `unsafe` as is.
    //
    // It's important that this goes before `Rc` so that it gets dropped first.
    ref_: Ref<'static, T>,
    _rc: Rc<WasmRefCell<T>>,
}

impl<T: ?Sized> RcRef<T> {
    pub fn new(rc: Rc<WasmRefCell<T>>) -> Self {
        let ref_ = unsafe { (*Rc::as_ptr(&rc)).borrow() };
        Self { _rc: rc, ref_ }
    }
}

impl<T: ?Sized> Deref for RcRef<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.ref_
    }
}

impl<T: ?Sized> Borrow<T> for RcRef<T> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.ref_
    }
}

/// A type that encapsulates an `Rc<WasmRefCell<T>>` as well as a
/// `RefMut` to the contents of that `WasmRefCell`.
///
/// The `'static` requirement is an unfortunate consequence of how this
/// is implemented.
pub struct RcRefMut<T: ?Sized + 'static> {
    ref_: RefMut<'static, T>,
    _rc: Rc<WasmRefCell<T>>,
}

impl<T: ?Sized> RcRefMut<T> {
    pub fn new(rc: Rc<WasmRefCell<T>>) -> Self {
        let ref_ = unsafe { (*Rc::as_ptr(&rc)).borrow_mut() };
        Self { _rc: rc, ref_ }
    }
}

impl<T: ?Sized> Deref for RcRefMut<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.ref_
    }
}

impl<T: ?Sized> DerefMut for RcRefMut<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.ref_
    }
}

impl<T: ?Sized> Borrow<T> for RcRefMut<T> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.ref_
    }
}

impl<T: ?Sized> BorrowMut<T> for RcRefMut<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.ref_
    }
}

#[no_mangle]
pub extern "C" fn __wbindgen_malloc(size: usize, align: usize) -> *mut u8 {
    if let Ok(layout) = Layout::from_size_align(size, align) {
        unsafe {
            if layout.size() > 0 {
                let ptr = alloc(layout);
                if !ptr.is_null() {
                    return ptr;
                }
            } else {
                return align as *mut u8;
            }
        }
    }

    malloc_failure();
}

#[no_mangle]
pub unsafe extern "C" fn __wbindgen_realloc(
    ptr: *mut u8,
    old_size: usize,
    new_size: usize,
    align: usize,
) -> *mut u8 {
    debug_assert!(old_size > 0);
    debug_assert!(new_size > 0);
    if let Ok(layout) = Layout::from_size_align(old_size, align) {
        let ptr = realloc(ptr, layout, new_size);
        if !ptr.is_null() {
            return ptr;
        }
    }
    malloc_failure();
}

#[cold]
fn malloc_failure() -> ! {
    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            super::throw_str("invalid malloc request")
        } else if #[cfg(feature = "std")] {
            std::process::abort();
        } else if #[cfg(all(
            target_arch = "wasm32",
            any(target_os = "unknown", target_os = "none")
        ))] {
            core::arch::wasm32::unreachable();
        } else {
            unreachable!()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __wbindgen_free(ptr: *mut u8, size: usize, align: usize) {
    // This happens for zero-length slices, and in that case `ptr` is
    // likely bogus so don't actually send this to the system allocator
    if size == 0 {
        return;
    }
    let layout = Layout::from_size_align_unchecked(size, align);
    dealloc(ptr, layout);
}

/// This is a curious function necessary to get wasm-bindgen working today,
/// and it's a bit of an unfortunate hack.
///
/// The general problem is that somehow we need the above two symbols to
/// exist in the final output binary (__wbindgen_malloc and
/// __wbindgen_free). These symbols may be called by JS for various
/// bindings, so we for sure need to make sure they're exported.
///
/// The problem arises, though, when what if no Rust code uses the symbols?
/// For all intents and purposes it looks to LLVM and the linker like the
/// above two symbols are dead code, so they're completely discarded!
///
/// Specifically what happens is this:
///
/// * The above two symbols are generated into some object file inside of
///   libwasm_bindgen.rlib
/// * The linker, LLD, will not load this object file unless *some* symbol
///   is loaded from the object. In this case, if the Rust code never calls
///   __wbindgen_malloc or __wbindgen_free then the symbols never get linked
///   in.
/// * Later when `wasm-bindgen` attempts to use the symbols they don't
///   exist, causing an error.
///
/// This function is a weird hack for this problem. We inject a call to this
/// function in all generated code. Usage of this function should then
/// ensure that the above two intrinsics are translated.
///
/// Due to how rustc creates object files this function (and anything inside
/// it) will be placed into the same object file as the two intrinsics
/// above. That means if this function is called and referenced we'll pull
/// in the object file and link the intrinsics.
///
/// Ideas for how to improve this are most welcome!
#[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
pub fn link_mem_intrinsics() {
    crate::link::link_intrinsics();
}

#[cfg(feature = "std")]
std::thread_local! {
    static GLOBAL_EXNDATA: Cell<[u32; 2]> = Cell::new([0; 2]);
}
#[cfg(all(not(feature = "std"), not(target_feature = "atomics")))]
static mut GLOBAL_EXNDATA: [u32; 2] = [0; 2];
#[cfg(all(not(feature = "std"), target_feature = "atomics"))]
#[thread_local]
static GLOBAL_EXNDATA: Cell<[u32; 2]> = Cell::new([0; 2]);

struct GlobalExndata;

impl GlobalExndata {
    #[cfg(feature = "std")]
    fn get() -> [u32; 2] {
        GLOBAL_EXNDATA.with(Cell::get)
    }

    #[cfg(all(not(feature = "std"), not(target_feature = "atomics")))]
    fn get() -> [u32; 2] {
        unsafe { GLOBAL_EXNDATA }
    }

    #[cfg(all(not(feature = "std"), target_feature = "atomics"))]
    fn get() -> [u32; 2] {
        GLOBAL_EXNDATA.get()
    }

    #[cfg(feature = "std")]
    fn set(data: [u32; 2]) {
        GLOBAL_EXNDATA.with(|d| d.set(data))
    }

    #[cfg(all(not(feature = "std"), not(target_feature = "atomics")))]
    fn set(data: [u32; 2]) {
        unsafe { GLOBAL_EXNDATA = data };
    }

    #[cfg(all(not(feature = "std"), target_feature = "atomics"))]
    fn set(data: [u32; 2]) {
        GLOBAL_EXNDATA.set(data);
    }
}

#[no_mangle]
pub unsafe extern "C" fn __wbindgen_exn_store(idx: u32) {
    debug_assert_eq!(GlobalExndata::get()[0], 0);
    GlobalExndata::set([1, idx]);
}

pub fn take_last_exception() -> Result<(), super::JsValue> {
    let ret = if GlobalExndata::get()[0] == 1 {
        Err(super::JsValue::_new(GlobalExndata::get()[1]))
    } else {
        Ok(())
    };
    GlobalExndata::set([0, 0]);
    ret
}

/// An internal helper trait for usage in `#[wasm_bindgen]` on `async`
/// functions to convert the return value of the function to
/// `Result<JsValue, JsValue>` which is what we'll return to JS (where an
/// error is a failed future).
pub trait IntoJsResult {
    fn into_js_result(self) -> Result<JsValue, JsValue>;
}

impl IntoJsResult for () {
    fn into_js_result(self) -> Result<JsValue, JsValue> {
        Ok(JsValue::undefined())
    }
}

impl<T: Into<JsValue>> IntoJsResult for T {
    fn into_js_result(self) -> Result<JsValue, JsValue> {
        Ok(self.into())
    }
}

impl<T: Into<JsValue>, E: Into<JsValue>> IntoJsResult for Result<T, E> {
    fn into_js_result(self) -> Result<JsValue, JsValue> {
        match self {
            Ok(e) => Ok(e.into()),
            Err(e) => Err(e.into()),
        }
    }
}

impl<E: Into<JsValue>> IntoJsResult for Result<(), E> {
    fn into_js_result(self) -> Result<JsValue, JsValue> {
        match self {
            Ok(()) => Ok(JsValue::undefined()),
            Err(e) => Err(e.into()),
        }
    }
}

/// An internal helper trait for usage in `#[wasm_bindgen(start)]`
/// functions to throw the error (if it is `Err`).
pub trait Start {
    fn start(self);
}

impl Start for () {
    #[inline]
    fn start(self) {}
}

impl<E: Into<JsValue>> Start for Result<(), E> {
    #[inline]
    fn start(self) {
        if let Err(e) = self {
            crate::throw_val(e.into());
        }
    }
}

/// An internal helper struct for usage in `#[wasm_bindgen(main)]`
/// functions to throw the error (if it is `Err`).
pub struct MainWrapper<T>(pub Option<T>);

pub trait Main {
    fn __wasm_bindgen_main(&mut self);
}

impl Main for &mut &mut MainWrapper<()> {
    #[inline]
    fn __wasm_bindgen_main(&mut self) {}
}

impl Main for &mut &mut MainWrapper<Infallible> {
    #[inline]
    fn __wasm_bindgen_main(&mut self) {}
}

impl<E: Into<JsValue>> Main for &mut &mut MainWrapper<Result<(), E>> {
    #[inline]
    fn __wasm_bindgen_main(&mut self) {
        if let Err(e) = self.0.take().unwrap() {
            crate::throw_val(e.into());
        }
    }
}

impl<E: core::fmt::Debug> Main for &mut MainWrapper<Result<(), E>> {
    #[inline]
    fn __wasm_bindgen_main(&mut self) {
        if let Err(e) = self.0.take().unwrap() {
            crate::throw_str(&alloc::format!("{:?}", e));
        }
    }
}

pub const fn flat_len<T, const SIZE: usize>(slices: [&[T]; SIZE]) -> usize {
    let mut len = 0;
    let mut i = 0;
    while i < slices.len() {
        len += slices[i].len();
        i += 1;
    }
    len
}

pub const fn flat_byte_slices<const RESULT_LEN: usize, const SIZE: usize>(
    slices: [&[u8]; SIZE],
) -> [u8; RESULT_LEN] {
    let mut result = [0; RESULT_LEN];

    let mut slice_index = 0;
    let mut result_offset = 0;

    while slice_index < slices.len() {
        let mut i = 0;
        let slice = slices[slice_index];
        while i < slice.len() {
            result[result_offset] = slice[i];
            i += 1;
            result_offset += 1;
        }
        slice_index += 1;
    }

    result
}

// NOTE: This method is used to encode u32 into a variable-length-integer during the compile-time .
// Generally speaking, the length of the encoded variable-length-integer depends on the size of the integer
// but the maximum capacity can be used here to simplify the amount of code during the compile-time .
pub const fn encode_u32_to_fixed_len_bytes(value: u32) -> [u8; 5] {
    let mut result: [u8; 5] = [0; 5];
    let mut i = 0;
    while i < 4 {
        result[i] = ((value >> (7 * i)) | 0x80) as u8;
        i += 1;
    }
    result[4] = (value >> (7 * 4)) as u8;
    result
}

/// Trait for element types to implement `Into<JsValue>` for vectors of
/// themselves, which isn't possible directly thanks to the orphan rule.
pub trait VectorIntoJsValue: Sized {
    fn vector_into_jsvalue(vector: Box<[Self]>) -> JsValue;
}

impl<T: VectorIntoJsValue> From<Box<[T]>> for JsValue {
    fn from(vector: Box<[T]>) -> Self {
        T::vector_into_jsvalue(vector)
    }
}

pub fn js_value_vector_into_jsvalue<T: Into<JsValue>>(vector: Box<[T]>) -> JsValue {
    let result = unsafe { JsValue::_new(super::__wbindgen_array_new()) };
    for value in vector.into_vec() {
        let js: JsValue = value.into();
        unsafe { super::__wbindgen_array_push(result.idx, js.idx) }
        // `__wbindgen_array_push` takes ownership over `js` and has already dropped it,
        // so don't drop it again.
        mem::forget(js);
    }
    result
}
