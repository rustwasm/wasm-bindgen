use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PushSubscriptionOptions` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionOptions)\n\n*This API requires the following crate features to be activated: `PushSubscriptionOptions`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PushSubscriptionOptions {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PushSubscriptionOptions: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PushSubscriptionOptions {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(23u32);
            inform(80u32);
            inform(117u32);
            inform(115u32);
            inform(104u32);
            inform(83u32);
            inform(117u32);
            inform(98u32);
            inform(115u32);
            inform(99u32);
            inform(114u32);
            inform(105u32);
            inform(112u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(79u32);
            inform(112u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for PushSubscriptionOptions {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PushSubscriptionOptions {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PushSubscriptionOptions {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PushSubscriptionOptions {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PushSubscriptionOptions {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PushSubscriptionOptions {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PushSubscriptionOptions {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PushSubscriptionOptions {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PushSubscriptionOptions {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PushSubscriptionOptions>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PushSubscriptionOptions {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PushSubscriptionOptions {
        #[inline]
        fn from(obj: JsValue) -> PushSubscriptionOptions {
            PushSubscriptionOptions { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PushSubscriptionOptions {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PushSubscriptionOptions> for PushSubscriptionOptions {
        #[inline]
        fn as_ref(&self) -> &PushSubscriptionOptions {
            self
        }
    }
    impl From<PushSubscriptionOptions> for JsValue {
        #[inline]
        fn from(obj: PushSubscriptionOptions) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PushSubscriptionOptions {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PushSubscriptionOptions(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PushSubscriptionOptions(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PushSubscriptionOptions(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PushSubscriptionOptions { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PushSubscriptionOptions) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PushSubscriptionOptions> for ::js_sys::Object {
    #[inline]
    fn from(obj: PushSubscriptionOptions) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PushSubscriptionOptions {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PushSubscriptionOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_application_server_key_PushSubscriptionOptions() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PushSubscriptionOptions as WasmDescribe>::describe();
    <Option<::js_sys::ArrayBuffer> as WasmDescribe>::describe();
}
impl PushSubscriptionOptions {
    #[cfg(all(feature = "PushSubscriptionOptions",))]
    #[allow(bad_style)]
    #[doc = "The `applicationServerKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionOptions/applicationServerKey)\n\n*This API requires the following crate features to be activated: `PushSubscriptionOptions`*"]
    #[allow(clippy::all)]
    pub fn application_server_key(
        &self,
    ) -> Result<Option<::js_sys::ArrayBuffer>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushSubscriptionOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_application_server_key_PushSubscriptionOptions(
                self_: <&PushSubscriptionOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::ArrayBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_application_server_key_PushSubscriptionOptions(
            self_: <&PushSubscriptionOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::ArrayBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PushSubscriptionOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_application_server_key_PushSubscriptionOptions(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(
                <Option<::js_sys::ArrayBuffer> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    _ret,
                ),
            )
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5127db0c48281d39: [u8; 315usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xF9\0\0\0\0\0\x02\0\0\x02\x17PushSubscriptionOptions)__widl_instanceof_PushSubscriptionOptions\0\0\0\07__widl_f_application_server_key_PushSubscriptionOptions\x01\0\0\x01\x17PushSubscriptionOptions\x01\0\x01\x14applicationServerKey\x01\x01\x05self_\x14applicationServerKey\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
