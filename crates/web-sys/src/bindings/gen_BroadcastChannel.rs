use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `BroadcastChannel` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct BroadcastChannel {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_BroadcastChannel: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for BroadcastChannel {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(66u32);
            inform(114u32);
            inform(111u32);
            inform(97u32);
            inform(100u32);
            inform(99u32);
            inform(97u32);
            inform(115u32);
            inform(116u32);
            inform(67u32);
            inform(104u32);
            inform(97u32);
            inform(110u32);
            inform(110u32);
            inform(101u32);
            inform(108u32);
        }
    }
    impl core::ops::Deref for BroadcastChannel {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for BroadcastChannel {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for BroadcastChannel {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a BroadcastChannel {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for BroadcastChannel {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            BroadcastChannel {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for BroadcastChannel {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a BroadcastChannel {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for BroadcastChannel {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<BroadcastChannel>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(BroadcastChannel {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for BroadcastChannel {
        #[inline]
        fn from(obj: JsValue) -> BroadcastChannel {
            BroadcastChannel { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for BroadcastChannel {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<BroadcastChannel> for BroadcastChannel {
        #[inline]
        fn as_ref(&self) -> &BroadcastChannel {
            self
        }
    }
    impl From<BroadcastChannel> for JsValue {
        #[inline]
        fn from(obj: BroadcastChannel) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for BroadcastChannel {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_BroadcastChannel(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_BroadcastChannel(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_BroadcastChannel(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            BroadcastChannel { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const BroadcastChannel) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<BroadcastChannel> for EventTarget {
    #[inline]
    fn from(obj: BroadcastChannel) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for BroadcastChannel {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<BroadcastChannel> for ::js_sys::Object {
    #[inline]
    fn from(obj: BroadcastChannel) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for BroadcastChannel {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BroadcastChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_BroadcastChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <BroadcastChannel as WasmDescribe>::describe();
}
impl BroadcastChannel {
    #[cfg(all(feature = "BroadcastChannel",))]
    #[allow(bad_style)]
    #[doc = "The `new BroadcastChannel(..)` constructor, creating a new instance of `BroadcastChannel`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/BroadcastChannel)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    #[allow(clippy::all)]
    pub fn new(channel: &str) -> Result<BroadcastChannel, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BroadcastChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_BroadcastChannel(
                channel: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <BroadcastChannel as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_BroadcastChannel(
            channel: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <BroadcastChannel as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(channel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let channel = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(channel);
                __widl_f_new_BroadcastChannel(channel)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<BroadcastChannel as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "BroadcastChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_BroadcastChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BroadcastChannel as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BroadcastChannel {
    #[cfg(all(feature = "BroadcastChannel",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/close)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    #[allow(clippy::all)]
    pub fn close(&self) {
        #[cfg(all(feature = "BroadcastChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_BroadcastChannel(
                self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_BroadcastChannel(
            self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_BroadcastChannel(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "BroadcastChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_post_message_BroadcastChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BroadcastChannel as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BroadcastChannel {
    #[cfg(all(feature = "BroadcastChannel",))]
    #[allow(bad_style)]
    #[doc = "The `postMessage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/postMessage)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    #[allow(clippy::all)]
    pub fn post_message(
        &self,
        message: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BroadcastChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_post_message_BroadcastChannel(
                self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_post_message_BroadcastChannel(
            self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(message);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let message =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        message,
                    );
                __widl_f_post_message_BroadcastChannel(self_, message)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "BroadcastChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_BroadcastChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BroadcastChannel as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl BroadcastChannel {
    #[cfg(all(feature = "BroadcastChannel",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/name)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "BroadcastChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_BroadcastChannel(
                self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_BroadcastChannel(
            self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_BroadcastChannel(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BroadcastChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_BroadcastChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BroadcastChannel as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl BroadcastChannel {
    #[cfg(all(feature = "BroadcastChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessage)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "BroadcastChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_BroadcastChannel(
                self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_BroadcastChannel(
            self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessage_BroadcastChannel(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BroadcastChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_BroadcastChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BroadcastChannel as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BroadcastChannel {
    #[cfg(all(feature = "BroadcastChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessage)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "BroadcastChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_BroadcastChannel(
                self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_BroadcastChannel(
            self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmessage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_BroadcastChannel(self_, onmessage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "BroadcastChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessageerror_BroadcastChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BroadcastChannel as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl BroadcastChannel {
    #[cfg(all(feature = "BroadcastChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onmessageerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessageerror)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    #[allow(clippy::all)]
    pub fn onmessageerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "BroadcastChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessageerror_BroadcastChannel(
                self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessageerror_BroadcastChannel(
            self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessageerror_BroadcastChannel(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BroadcastChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessageerror_BroadcastChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BroadcastChannel as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BroadcastChannel {
    #[cfg(all(feature = "BroadcastChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onmessageerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessageerror)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    #[allow(clippy::all)]
    pub fn set_onmessageerror(&self, onmessageerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "BroadcastChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessageerror_BroadcastChannel(
                self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessageerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessageerror_BroadcastChannel(
            self_: <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmessageerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onmessageerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BroadcastChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessageerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessageerror,
                    );
                __widl_f_set_onmessageerror_BroadcastChannel(self_, onmessageerror)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5b7019e51178dbc4: [u8; 900usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}B\x03\0\0\0\0\t\0\0\x02\x10BroadcastChannel\"__widl_instanceof_BroadcastChannel\0\0\0\0\x1D__widl_f_new_BroadcastChannel\x01\0\0\x01\x10BroadcastChannel\0\x01\x01\x07channel\x03new\0\0\0\x1F__widl_f_close_BroadcastChannel\0\0\0\x01\x10BroadcastChannel\x01\0\0\x01\x01\x05self_\x05close\0\0\0&__widl_f_post_message_BroadcastChannel\x01\0\0\x01\x10BroadcastChannel\x01\0\0\x01\x02\x05self_\x07message\x0BpostMessage\0\0\0\x1E__widl_f_name_BroadcastChannel\0\0\0\x01\x10BroadcastChannel\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0#__widl_f_onmessage_BroadcastChannel\0\0\0\x01\x10BroadcastChannel\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\0'__widl_f_set_onmessage_BroadcastChannel\0\0\0\x01\x10BroadcastChannel\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\0(__widl_f_onmessageerror_BroadcastChannel\0\0\0\x01\x10BroadcastChannel\x01\0\x01\x0Eonmessageerror\x01\x01\x05self_\x0Eonmessageerror\0\0\0,__widl_f_set_onmessageerror_BroadcastChannel\0\0\0\x01\x10BroadcastChannel\x01\0\x02\x0Eonmessageerror\x01\x02\x05self_\x0Eonmessageerror\x0Eonmessageerror\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
