use core::mem;

use describe::WasmDescribe;
use JsValue;

/// An unsafe trait which represents types that are ABI-safe to pass via wasm
/// arguments.
///
/// This is an unsafe trait to implement as there's no guarantee the type is
/// actually safe to transfer across the was boundary, it's up to you to
/// guarantee this so codegen works correctly.
pub unsafe trait WasmAbi: Sized {
    fn into_js_value<T: WasmDescribe>(self) -> JsValue;
    fn from_js_value<T: WasmDescribe>(val: &JsValue) -> Result<Self, JsValue>;
}

#[repr(C)]
struct Catch {
    thrown: u32,
    val_idx: u32,
}

impl Catch {
    fn new() -> Catch {
        Catch { thrown: 0, val_idx: 0 }
    }

    fn or_else<T>(&self, t: T) -> Result<T, JsValue> {
        if self.thrown != 0 {
            Err(JsValue { idx: self.val_idx })
        } else {
            Ok(t)
        }
    }
}

externs! {
    fn __wbindgen_into_js_u32(val: u32, describe: u32) -> u32;
    fn __wbindgen_into_js_u64(low: u32, high: u32, describe: u32) -> u32;
    fn __wbindgen_into_js_f32(val: f32, describe: u32) -> u32;
    fn __wbindgen_into_js_f64(val: f64, describe: u32) -> u32;
    fn __wbindgen_into_js_slice(ptr: u32, len: u32, describe: u32) -> u32;
    fn __wbindgen_into_js_optional_u32(present: u32, val: u32, describe: u32) -> u32;
    fn __wbindgen_into_js_optional_u64(present: u32, low: u32, high: u32, describe: u32) -> u32;
    fn __wbindgen_into_js_optional_f32(present: u32, val: f32, describe: u32) -> u32;
    fn __wbindgen_into_js_optional_f64(present: u32, val: f64, describe: u32) -> u32;

    fn __wbindgen_from_js_u32(js: u32, catch: *mut Catch, describe: u32) -> u32;
    fn __wbindgen_from_js_u64(outptr: *mut Wasm64, js: u32, catch: *mut Catch, describe: u32) -> ();
    fn __wbindgen_from_js_f32(js: u32, catch: *mut Catch, describe: u32) -> f32;
    fn __wbindgen_from_js_f64(js: u32, catch: *mut Catch, describe: u32) -> f64;
    fn __wbindgen_from_js_slice(outptr: *mut WasmSlice, js: u32, catch: *mut Catch, describe: u32) -> ();
    fn __wbindgen_from_js_optional_u32(outptr: *mut WasmOptionalU32, js: u32, catch: *mut Catch, describe: u32) -> ();
    fn __wbindgen_from_js_optional_u64(outptr: *mut WasmOptional64, js: u32, catch: *mut Catch, describe: u32) -> ();
    fn __wbindgen_from_js_optional_f32(outptr: *mut WasmOptionalF32, js: u32, catch: *mut Catch, describe: u32) -> ();
    fn __wbindgen_from_js_optional_f64(outptr: *mut WasmOptionalF64, js: u32, catch: *mut Catch, describe: u32) -> ();
}

unsafe impl WasmAbi for () {
    fn into_js_value<T: WasmDescribe>(self) -> JsValue {
        JsValue::UNDEFINED
    }
    fn from_js_value<T: WasmDescribe>(_val: &JsValue) -> Result<Self, JsValue> {
        Ok(())
    }
}

unsafe impl WasmAbi for u32 {
    #[inline(never)]
    fn into_js_value<T: WasmDescribe>(self) -> JsValue {
        unsafe {
            JsValue {
                idx: __wbindgen_into_js_u32(self, T::describe as u32),
            }
        }
    }

    fn from_js_value<T: WasmDescribe>(val: &JsValue) -> Result<Self, JsValue> {
        #[inline(never)]
        unsafe fn doit<T: WasmDescribe>(idx: u32, catch: *mut Catch) -> u32 {
            __wbindgen_from_js_u32(idx, catch, T::describe as u32)
        }
        unsafe {
            let mut c = Catch::new();
            let r = doit::<T>(val.idx, &mut c);
            c.or_else(r)
        }
    }
}

unsafe impl WasmAbi for i32 {
    fn into_js_value<T: WasmDescribe>(self) -> JsValue {
        (self as u32).into_js_value::<T>()
    }

    fn from_js_value<T: WasmDescribe>(val: &JsValue) -> Result<Self, JsValue> {
        <u32 as WasmAbi>::from_js_value::<T>(val).map(|i| i as i32)
    }
}

unsafe impl WasmAbi for f32 {
    #[inline(never)]
    fn into_js_value<T: WasmDescribe>(self) -> JsValue {
        unsafe {
            JsValue {
                idx: __wbindgen_into_js_f32(self, T::describe as u32),
            }
        }
    }

    fn from_js_value<T: WasmDescribe>(val: &JsValue) -> Result<Self, JsValue> {
        #[inline(never)]
        unsafe fn doit<T: WasmDescribe>(idx: u32, catch: *mut Catch) -> f32 {
            __wbindgen_from_js_f32(idx, catch, T::describe as u32)
        }
        unsafe {
            let mut c = Catch::new();
            let r = doit::<T>(val.idx, &mut c);
            c.or_else(r)
        }
    }
}

unsafe impl WasmAbi for f64 {
    #[inline(never)]
    fn into_js_value<T: WasmDescribe>(self) -> JsValue {
        unsafe {
            JsValue {
                idx: __wbindgen_into_js_f64(self, T::describe as u32),
            }
        }
    }

    fn from_js_value<T: WasmDescribe>(val: &JsValue) -> Result<Self, JsValue> {
        #[inline(never)]
        unsafe fn doit<T: WasmDescribe>(idx: u32, catch: *mut Catch) -> f64 {
            __wbindgen_from_js_f64(idx, catch, T::describe as u32)
        }
        unsafe {
            let mut c = Catch::new();
            let r = doit::<T>(val.idx, &mut c);
            c.or_else(r)
        }
    }
}

#[repr(C)]
pub struct WasmOptionalI32 {
    pub present: u32,
    pub value: i32,
}

unsafe impl WasmAbi for WasmOptionalI32 {
    fn into_js_value<T: WasmDescribe>(self) -> JsValue {
        WasmOptionalU32 {
            present: self.present,
            value: self.value as u32,
        }.into_js_value::<T>()
    }

    fn from_js_value<T: WasmDescribe>(val: &JsValue) -> Result<Self, JsValue> {
        let val = WasmOptionalU32::from_js_value::<T>(val)?;
        Ok(WasmOptionalI32 {
            present: val.present,
            value: val.value as i32,
        })
    }
}

#[repr(C)]
pub struct WasmOptionalU32 {
    pub present: u32,
    pub value: u32,
}

unsafe impl WasmAbi for WasmOptionalU32 {
    #[inline(never)]
    fn into_js_value<T: WasmDescribe>(self) -> JsValue {
        unsafe {
            JsValue {
                idx: __wbindgen_into_js_optional_u32(
                    self.present,
                    self.value,
                    T::describe as u32,
                ),
            }
        }
    }

    fn from_js_value<T: WasmDescribe>(val: &JsValue) -> Result<Self, JsValue> {
        #[inline(never)]
        unsafe fn doit<T: WasmDescribe>(out: *mut WasmOptionalU32, idx: u32, catch: *mut Catch) {
            __wbindgen_from_js_optional_u32(out, idx, catch, T::describe as u32)
        }

        unsafe {
            let mut c = Catch::new();
            let mut x: WasmOptionalU32 = mem::zeroed();
            doit::<T>(&mut x, val.idx, &mut c);
            c.or_else(x)
        }
    }
}

#[repr(C)]
pub struct WasmOptionalF32 {
    pub present: u32,
    pub value: f32,
}

unsafe impl WasmAbi for WasmOptionalF32 {
    #[inline(never)]
    fn into_js_value<T: WasmDescribe>(self) -> JsValue {
        unsafe {
            JsValue {
                idx: __wbindgen_into_js_optional_f32(
                    self.present,
                    self.value,
                    T::describe as u32,
                ),
            }
        }
    }

    fn from_js_value<T: WasmDescribe>(val: &JsValue) -> Result<Self, JsValue> {
        #[inline(never)]
        unsafe fn doit<T: WasmDescribe>(out: *mut WasmOptionalF32, idx: u32, catch: *mut Catch) {
            __wbindgen_from_js_optional_f32(out, idx, catch, T::describe as u32)
        }

        unsafe {
            let mut c = Catch::new();
            let mut x: WasmOptionalF32 = mem::zeroed();
            doit::<T>(&mut x, val.idx, &mut c);
            c.or_else(x)
        }
    }
}

#[repr(C)]
pub struct WasmOptionalF64 {
    pub present: u32,
    pub value: f64,
}

unsafe impl WasmAbi for WasmOptionalF64 {
    #[inline(never)]
    fn into_js_value<T: WasmDescribe>(self) -> JsValue {
        unsafe {
            JsValue {
                idx: __wbindgen_into_js_optional_f64(
                    self.present,
                    self.value,
                    T::describe as u32,
                ),
            }
        }
    }

    fn from_js_value<T: WasmDescribe>(val: &JsValue) -> Result<Self, JsValue> {
        #[inline(never)]
        unsafe fn doit<T: WasmDescribe>(out: *mut WasmOptionalF64, idx: u32, catch: *mut Catch) {
            __wbindgen_from_js_optional_f64(out, idx, catch, T::describe as u32)
        }

        unsafe {
            let mut c = Catch::new();
            let mut x: WasmOptionalF64 = mem::zeroed();
            doit::<T>(&mut x, val.idx, &mut c);
            c.or_else(x)
        }
    }
}

#[repr(C)]
pub struct Wasm64 {
    pub low: u32,
    pub high: u32,
}

unsafe impl WasmAbi for Wasm64 {
    #[inline(never)]
    fn into_js_value<T: WasmDescribe>(self) -> JsValue {
        unsafe {
            JsValue {
                idx: __wbindgen_into_js_u64(
                    self.low,
                    self.high,
                    T::describe as u32,
                ),
            }
        }
    }

    fn from_js_value<T: WasmDescribe>(val: &JsValue) -> Result<Self, JsValue> {
        #[inline(never)]
        unsafe fn doit<T: WasmDescribe>(out: *mut Wasm64, idx: u32, catch: *mut Catch) {
            __wbindgen_from_js_u64(out, idx, catch, T::describe as u32)
        }

        unsafe {
            let mut c = Catch::new();
            let mut x: Wasm64 = mem::zeroed();
            doit::<T>(&mut x, val.idx, &mut c);
            c.or_else(x)
        }
    }
}

#[repr(C)]
pub struct WasmOptional64 {
    pub present: u32,
    pub padding: u32,
    pub low: u32,
    pub high: u32,
}

unsafe impl WasmAbi for WasmOptional64 {
    #[inline(never)]
    fn into_js_value<T: WasmDescribe>(self) -> JsValue {
        unsafe {
            JsValue {
                idx: __wbindgen_into_js_optional_u64(
                    self.present,
                    self.low,
                    self.high,
                    T::describe as u32,
                ),
            }
        }
    }

    fn from_js_value<T: WasmDescribe>(val: &JsValue) -> Result<Self, JsValue> {
        #[inline(never)]
        unsafe fn doit<T: WasmDescribe>(out: *mut WasmOptional64, idx: u32, catch: *mut Catch) {
            __wbindgen_from_js_optional_u64(out, idx, catch, T::describe as u32)
        }

        unsafe {
            let mut c = Catch::new();
            let mut x: WasmOptional64 = mem::zeroed();
            doit::<T>(&mut x, val.idx, &mut c);
            c.or_else(x)
        }
    }
}

#[repr(C)]
pub struct WasmSlice {
    pub ptr: u32,
    pub len: u32,
}

unsafe impl WasmAbi for WasmSlice {
    #[inline(never)]
    fn into_js_value<T: WasmDescribe>(self) -> JsValue {
        unsafe {
            JsValue {
                idx: __wbindgen_into_js_slice(
                    self.ptr,
                    self.len,
                    T::describe as u32,
                ),
            }
        }
    }

    fn from_js_value<T: WasmDescribe>(val: &JsValue) -> Result<Self, JsValue> {
        #[inline(never)]
        unsafe fn doit<T: WasmDescribe>(out: *mut WasmSlice, idx: u32, catch: *mut Catch) {
            __wbindgen_from_js_slice(out, idx, catch, T::describe as u32)
        }

        unsafe {
            let mut c = Catch::new();
            let mut x: WasmSlice = mem::zeroed();
            doit::<T>(&mut x, val.idx, &mut c);
            c.or_else(x)
        }
    }
}
