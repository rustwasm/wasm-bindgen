use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PromiseNativeHandler` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseNativeHandler)\n\n*This API requires the following crate features to be activated: `PromiseNativeHandler`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PromiseNativeHandler {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PromiseNativeHandler: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PromiseNativeHandler {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(80u32);
            inform(114u32);
            inform(111u32);
            inform(109u32);
            inform(105u32);
            inform(115u32);
            inform(101u32);
            inform(78u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(118u32);
            inform(101u32);
            inform(72u32);
            inform(97u32);
            inform(110u32);
            inform(100u32);
            inform(108u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for PromiseNativeHandler {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PromiseNativeHandler {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PromiseNativeHandler {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PromiseNativeHandler {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PromiseNativeHandler {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PromiseNativeHandler {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PromiseNativeHandler {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PromiseNativeHandler {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PromiseNativeHandler {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PromiseNativeHandler>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PromiseNativeHandler {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PromiseNativeHandler {
        #[inline]
        fn from(obj: JsValue) -> PromiseNativeHandler {
            PromiseNativeHandler { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PromiseNativeHandler {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PromiseNativeHandler> for PromiseNativeHandler {
        #[inline]
        fn as_ref(&self) -> &PromiseNativeHandler {
            self
        }
    }
    impl From<PromiseNativeHandler> for JsValue {
        #[inline]
        fn from(obj: PromiseNativeHandler) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PromiseNativeHandler {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PromiseNativeHandler(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PromiseNativeHandler(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PromiseNativeHandler(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PromiseNativeHandler { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PromiseNativeHandler) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PromiseNativeHandler> for ::js_sys::Object {
    #[inline]
    fn from(obj: PromiseNativeHandler) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PromiseNativeHandler {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5724d71214ad907b: [u8; 169usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}g\0\0\0\0\0\x01\0\0\x02\x14PromiseNativeHandler&__widl_instanceof_PromiseNativeHandler\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
