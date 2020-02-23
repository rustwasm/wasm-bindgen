use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBCursorWithValue` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue)\n\n*This API requires the following crate features to be activated: `IdbCursorWithValue`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbCursorWithValue {
    obj: IdbCursor,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbCursorWithValue: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbCursorWithValue {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(67u32);
            inform(117u32);
            inform(114u32);
            inform(115u32);
            inform(111u32);
            inform(114u32);
            inform(87u32);
            inform(105u32);
            inform(116u32);
            inform(104u32);
            inform(86u32);
            inform(97u32);
            inform(108u32);
            inform(117u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for IdbCursorWithValue {
        type Target = IdbCursor;
        #[inline]
        fn deref(&self) -> &IdbCursor {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbCursorWithValue {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbCursorWithValue {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbCursorWithValue {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbCursorWithValue {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbCursorWithValue {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbCursorWithValue {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbCursorWithValue {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbCursorWithValue {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbCursorWithValue>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbCursorWithValue {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbCursorWithValue {
        #[inline]
        fn from(obj: JsValue) -> IdbCursorWithValue {
            IdbCursorWithValue { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbCursorWithValue {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbCursorWithValue> for IdbCursorWithValue {
        #[inline]
        fn as_ref(&self) -> &IdbCursorWithValue {
            self
        }
    }
    impl From<IdbCursorWithValue> for JsValue {
        #[inline]
        fn from(obj: IdbCursorWithValue) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbCursorWithValue {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBCursorWithValue(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBCursorWithValue(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBCursorWithValue(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbCursorWithValue { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbCursorWithValue) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbCursorWithValue> for IdbCursor {
    #[inline]
    fn from(obj: IdbCursorWithValue) -> IdbCursor {
        use wasm_bindgen::JsCast;
        IdbCursor::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<IdbCursor> for IdbCursorWithValue {
    #[inline]
    fn as_ref(&self) -> &IdbCursor {
        use wasm_bindgen::JsCast;
        IdbCursor::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IdbCursorWithValue> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbCursorWithValue) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbCursorWithValue {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbCursorWithValue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_IDBCursorWithValue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbCursorWithValue as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl IdbCursorWithValue {
    #[cfg(all(feature = "IdbCursorWithValue",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue/value)\n\n*This API requires the following crate features to be activated: `IdbCursorWithValue`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbCursorWithValue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_IDBCursorWithValue(
                self_: <&IdbCursorWithValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_IDBCursorWithValue(
            self_: <&IdbCursorWithValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&IdbCursorWithValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_IDBCursorWithValue(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_0fb05499c0613dc4: [u8; 248usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB6\0\0\0\0\0\x02\0\0\x02\x12IDBCursorWithValue$__widl_instanceof_IDBCursorWithValue\0\0\0\0!__widl_f_value_IDBCursorWithValue\x01\0\0\x01\x12IDBCursorWithValue\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
