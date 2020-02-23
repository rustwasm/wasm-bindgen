use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PresentationReceiver` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver)\n\n*This API requires the following crate features to be activated: `PresentationReceiver`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PresentationReceiver {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PresentationReceiver: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PresentationReceiver {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(80u32);
            inform(114u32);
            inform(101u32);
            inform(115u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(82u32);
            inform(101u32);
            inform(99u32);
            inform(101u32);
            inform(105u32);
            inform(118u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for PresentationReceiver {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PresentationReceiver {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PresentationReceiver {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PresentationReceiver {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PresentationReceiver {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PresentationReceiver {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PresentationReceiver {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PresentationReceiver {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PresentationReceiver {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PresentationReceiver>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PresentationReceiver {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PresentationReceiver {
        #[inline]
        fn from(obj: JsValue) -> PresentationReceiver {
            PresentationReceiver { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PresentationReceiver {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PresentationReceiver> for PresentationReceiver {
        #[inline]
        fn as_ref(&self) -> &PresentationReceiver {
            self
        }
    }
    impl From<PresentationReceiver> for JsValue {
        #[inline]
        fn from(obj: PresentationReceiver) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PresentationReceiver {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PresentationReceiver(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PresentationReceiver(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PresentationReceiver(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PresentationReceiver { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PresentationReceiver) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PresentationReceiver> for ::js_sys::Object {
    #[inline]
    fn from(obj: PresentationReceiver) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PresentationReceiver {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PresentationReceiver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connection_list_PresentationReceiver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationReceiver as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl PresentationReceiver {
    #[cfg(all(feature = "PresentationReceiver",))]
    #[allow(bad_style)]
    #[doc = "The `connectionList` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver/connectionList)\n\n*This API requires the following crate features to be activated: `PresentationReceiver`*"]
    #[allow(clippy::all)]
    pub fn connection_list(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PresentationReceiver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connection_list_PresentationReceiver(
                self_: <&PresentationReceiver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connection_list_PresentationReceiver(
            self_: <&PresentationReceiver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PresentationReceiver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_connection_list_PresentationReceiver(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_67f07f7fe83ff2cc: [u8; 284usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xDA\0\0\0\0\0\x02\0\0\x02\x14PresentationReceiver&__widl_instanceof_PresentationReceiver\0\0\0\0-__widl_f_connection_list_PresentationReceiver\x01\0\0\x01\x14PresentationReceiver\x01\0\x01\x0EconnectionList\x01\x01\x05self_\x0EconnectionList\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
