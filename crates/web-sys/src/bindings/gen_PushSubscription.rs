use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PushSubscription` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription)\n\n*This API requires the following crate features to be activated: `PushSubscription`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PushSubscription {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PushSubscription: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PushSubscription {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
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
        }
    }
    impl core::ops::Deref for PushSubscription {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PushSubscription {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PushSubscription {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PushSubscription {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PushSubscription {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PushSubscription {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PushSubscription {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PushSubscription {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PushSubscription {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PushSubscription>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PushSubscription {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PushSubscription {
        #[inline]
        fn from(obj: JsValue) -> PushSubscription {
            PushSubscription { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PushSubscription {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PushSubscription> for PushSubscription {
        #[inline]
        fn as_ref(&self) -> &PushSubscription {
            self
        }
    }
    impl From<PushSubscription> for JsValue {
        #[inline]
        fn from(obj: PushSubscription) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PushSubscription {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PushSubscription(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PushSubscription(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PushSubscription(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PushSubscription { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PushSubscription) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PushSubscription> for ::js_sys::Object {
    #[inline]
    fn from(obj: PushSubscription) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PushSubscription {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PushEncryptionKeyName", feature = "PushSubscription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_key_PushSubscription() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PushSubscription as WasmDescribe>::describe();
    <PushEncryptionKeyName as WasmDescribe>::describe();
    <Option<::js_sys::ArrayBuffer> as WasmDescribe>::describe();
}
impl PushSubscription {
    #[cfg(all(feature = "PushEncryptionKeyName", feature = "PushSubscription",))]
    #[allow(bad_style)]
    #[doc = "The `getKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/getKey)\n\n*This API requires the following crate features to be activated: `PushEncryptionKeyName`, `PushSubscription`*"]
    #[allow(clippy::all)]
    pub fn get_key(
        &self,
        name: PushEncryptionKeyName,
    ) -> Result<Option<::js_sys::ArrayBuffer>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushEncryptionKeyName", feature = "PushSubscription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_key_PushSubscription(
                self_: <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <PushEncryptionKeyName as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::ArrayBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_key_PushSubscription(
            self_: <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <PushEncryptionKeyName as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::ArrayBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name =
                    <PushEncryptionKeyName as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_key_PushSubscription(self_, name)
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
#[cfg(all(feature = "PushSubscription", feature = "PushSubscriptionJson",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_PushSubscription() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PushSubscription as WasmDescribe>::describe();
    <PushSubscriptionJson as WasmDescribe>::describe();
}
impl PushSubscription {
    #[cfg(all(feature = "PushSubscription", feature = "PushSubscriptionJson",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/toJSON)\n\n*This API requires the following crate features to be activated: `PushSubscription`, `PushSubscriptionJson`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> Result<PushSubscriptionJson, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushSubscription", feature = "PushSubscriptionJson",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_PushSubscription(
                self_: <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PushSubscriptionJson as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_PushSubscription(
            self_: <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PushSubscriptionJson as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_PushSubscription(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PushSubscriptionJson as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PushSubscription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unsubscribe_PushSubscription() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PushSubscription as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl PushSubscription {
    #[cfg(all(feature = "PushSubscription",))]
    #[allow(bad_style)]
    #[doc = "The `unsubscribe()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/unsubscribe)\n\n*This API requires the following crate features to be activated: `PushSubscription`*"]
    #[allow(clippy::all)]
    pub fn unsubscribe(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushSubscription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unsubscribe_PushSubscription(
                self_: <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unsubscribe_PushSubscription(
            self_: <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_unsubscribe_PushSubscription(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PushSubscription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_endpoint_PushSubscription() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PushSubscription as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PushSubscription {
    #[cfg(all(feature = "PushSubscription",))]
    #[allow(bad_style)]
    #[doc = "The `endpoint` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/endpoint)\n\n*This API requires the following crate features to be activated: `PushSubscription`*"]
    #[allow(clippy::all)]
    pub fn endpoint(&self) -> String {
        #[cfg(all(feature = "PushSubscription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_endpoint_PushSubscription(
                self_: <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_endpoint_PushSubscription(
            self_: <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_endpoint_PushSubscription(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PushSubscription", feature = "PushSubscriptionOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_options_PushSubscription() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PushSubscription as WasmDescribe>::describe();
    <PushSubscriptionOptions as WasmDescribe>::describe();
}
impl PushSubscription {
    #[cfg(all(feature = "PushSubscription", feature = "PushSubscriptionOptions",))]
    #[allow(bad_style)]
    #[doc = "The `options` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/options)\n\n*This API requires the following crate features to be activated: `PushSubscription`, `PushSubscriptionOptions`*"]
    #[allow(clippy::all)]
    pub fn options(&self) -> PushSubscriptionOptions {
        #[cfg(all(feature = "PushSubscription", feature = "PushSubscriptionOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_options_PushSubscription(
                self_: <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PushSubscriptionOptions as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_options_PushSubscription(
            self_: <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PushSubscriptionOptions as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PushSubscription as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_options_PushSubscription(self_)
            };
            <PushSubscriptionOptions as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7e2648fd19dcf02d: [u8; 576usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xFE\x01\0\0\0\0\x06\0\0\x02\x10PushSubscription\"__widl_instanceof_PushSubscription\0\0\0\0!__widl_f_get_key_PushSubscription\x01\0\0\x01\x10PushSubscription\x01\0\0\x01\x02\x05self_\x04name\x06getKey\0\0\0!__widl_f_to_json_PushSubscription\x01\0\0\x01\x10PushSubscription\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0%__widl_f_unsubscribe_PushSubscription\x01\0\0\x01\x10PushSubscription\x01\0\0\x01\x01\x05self_\x0Bunsubscribe\0\0\0\"__widl_f_endpoint_PushSubscription\0\0\0\x01\x10PushSubscription\x01\0\x01\x08endpoint\x01\x01\x05self_\x08endpoint\0\0\0!__widl_f_options_PushSubscription\0\0\0\x01\x10PushSubscription\x01\0\x01\x07options\x01\x01\x05self_\x07options\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
