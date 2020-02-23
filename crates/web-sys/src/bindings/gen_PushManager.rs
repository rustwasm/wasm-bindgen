use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PushManager` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager)\n\n*This API requires the following crate features to be activated: `PushManager`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PushManager {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PushManager: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PushManager {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(80u32);
            inform(117u32);
            inform(115u32);
            inform(104u32);
            inform(77u32);
            inform(97u32);
            inform(110u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for PushManager {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PushManager {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PushManager {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PushManager {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PushManager {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PushManager {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PushManager {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PushManager {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PushManager {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PushManager>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PushManager {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PushManager {
        #[inline]
        fn from(obj: JsValue) -> PushManager {
            PushManager { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PushManager {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PushManager> for PushManager {
        #[inline]
        fn as_ref(&self) -> &PushManager {
            self
        }
    }
    impl From<PushManager> for JsValue {
        #[inline]
        fn from(obj: PushManager) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PushManager {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PushManager(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PushManager(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PushManager(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PushManager { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PushManager) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PushManager> for ::js_sys::Object {
    #[inline]
    fn from(obj: PushManager) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PushManager {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PushManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_subscription_PushManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PushManager as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl PushManager {
    #[cfg(all(feature = "PushManager",))]
    #[allow(bad_style)]
    #[doc = "The `getSubscription()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/getSubscription)\n\n*This API requires the following crate features to be activated: `PushManager`*"]
    #[allow(clippy::all)]
    pub fn get_subscription(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_subscription_PushManager(
                self_: <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_subscription_PushManager(
            self_: <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_subscription_PushManager(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PushManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_permission_state_PushManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PushManager as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl PushManager {
    #[cfg(all(feature = "PushManager",))]
    #[allow(bad_style)]
    #[doc = "The `permissionState()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/permissionState)\n\n*This API requires the following crate features to be activated: `PushManager`*"]
    #[allow(clippy::all)]
    pub fn permission_state(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_permission_state_PushManager(
                self_: <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_permission_state_PushManager(
            self_: <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_permission_state_PushManager(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PushManager", feature = "PushSubscriptionOptionsInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_permission_state_with_options_PushManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PushManager as WasmDescribe>::describe();
    <&PushSubscriptionOptionsInit as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl PushManager {
    #[cfg(all(feature = "PushManager", feature = "PushSubscriptionOptionsInit",))]
    #[allow(bad_style)]
    #[doc = "The `permissionState()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/permissionState)\n\n*This API requires the following crate features to be activated: `PushManager`, `PushSubscriptionOptionsInit`*"]
    #[allow(clippy::all)]
    pub fn permission_state_with_options(
        &self,
        options: &PushSubscriptionOptionsInit,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushManager", feature = "PushSubscriptionOptionsInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_permission_state_with_options_PushManager(
                self_: <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&PushSubscriptionOptionsInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_permission_state_with_options_PushManager(
            self_: <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&PushSubscriptionOptionsInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&PushSubscriptionOptionsInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_permission_state_with_options_PushManager(self_, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PushManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_subscribe_PushManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PushManager as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl PushManager {
    #[cfg(all(feature = "PushManager",))]
    #[allow(bad_style)]
    #[doc = "The `subscribe()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/subscribe)\n\n*This API requires the following crate features to be activated: `PushManager`*"]
    #[allow(clippy::all)]
    pub fn subscribe(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_subscribe_PushManager(
                self_: <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_subscribe_PushManager(
            self_: <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_subscribe_PushManager(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PushManager", feature = "PushSubscriptionOptionsInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_subscribe_with_options_PushManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PushManager as WasmDescribe>::describe();
    <&PushSubscriptionOptionsInit as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl PushManager {
    #[cfg(all(feature = "PushManager", feature = "PushSubscriptionOptionsInit",))]
    #[allow(bad_style)]
    #[doc = "The `subscribe()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/subscribe)\n\n*This API requires the following crate features to be activated: `PushManager`, `PushSubscriptionOptionsInit`*"]
    #[allow(clippy::all)]
    pub fn subscribe_with_options(
        &self,
        options: &PushSubscriptionOptionsInit,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushManager", feature = "PushSubscriptionOptionsInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_subscribe_with_options_PushManager(
                self_: <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&PushSubscriptionOptionsInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_subscribe_with_options_PushManager(
            self_: <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&PushSubscriptionOptionsInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PushManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&PushSubscriptionOptionsInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_subscribe_with_options_PushManager(self_, options)
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
pub static __WASM_BINDGEN_GENERATED_87ee792f5d9305d1: [u8; 587usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\t\x02\0\0\0\0\x06\0\0\x02\x0BPushManager\x1D__widl_instanceof_PushManager\0\0\0\0%__widl_f_get_subscription_PushManager\x01\0\0\x01\x0BPushManager\x01\0\0\x01\x01\x05self_\x0FgetSubscription\0\0\0%__widl_f_permission_state_PushManager\x01\0\0\x01\x0BPushManager\x01\0\0\x01\x01\x05self_\x0FpermissionState\0\0\02__widl_f_permission_state_with_options_PushManager\x01\0\0\x01\x0BPushManager\x01\0\0\x01\x02\x05self_\x07options\x0FpermissionState\0\0\0\x1E__widl_f_subscribe_PushManager\x01\0\0\x01\x0BPushManager\x01\0\0\x01\x01\x05self_\tsubscribe\0\0\0+__widl_f_subscribe_with_options_PushManager\x01\0\0\x01\x0BPushManager\x01\0\0\x01\x02\x05self_\x07options\tsubscribe\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
