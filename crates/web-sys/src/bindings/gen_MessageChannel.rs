use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MessageChannel` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel)\n\n*This API requires the following crate features to be activated: `MessageChannel`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MessageChannel {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MessageChannel: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MessageChannel {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(77u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(67u32);
            inform(104u32);
            inform(97u32);
            inform(110u32);
            inform(110u32);
            inform(101u32);
            inform(108u32);
        }
    }
    impl core::ops::Deref for MessageChannel {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MessageChannel {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MessageChannel {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MessageChannel {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MessageChannel {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MessageChannel {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MessageChannel {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MessageChannel {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MessageChannel {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MessageChannel>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MessageChannel {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MessageChannel {
        #[inline]
        fn from(obj: JsValue) -> MessageChannel {
            MessageChannel { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MessageChannel {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MessageChannel> for MessageChannel {
        #[inline]
        fn as_ref(&self) -> &MessageChannel {
            self
        }
    }
    impl From<MessageChannel> for JsValue {
        #[inline]
        fn from(obj: MessageChannel) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MessageChannel {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MessageChannel(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MessageChannel(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MessageChannel(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MessageChannel { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MessageChannel) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MessageChannel> for ::js_sys::Object {
    #[inline]
    fn from(obj: MessageChannel) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MessageChannel {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MessageChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MessageChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <MessageChannel as WasmDescribe>::describe();
}
impl MessageChannel {
    #[cfg(all(feature = "MessageChannel",))]
    #[allow(bad_style)]
    #[doc = "The `new MessageChannel(..)` constructor, creating a new instance of `MessageChannel`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/MessageChannel)\n\n*This API requires the following crate features to be activated: `MessageChannel`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<MessageChannel, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MessageChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MessageChannel(
            ) -> <MessageChannel as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MessageChannel(
        ) -> <MessageChannel as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_MessageChannel() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MessageChannel as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MessageChannel", feature = "MessagePort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_port1_MessageChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MessageChannel as WasmDescribe>::describe();
    <MessagePort as WasmDescribe>::describe();
}
impl MessageChannel {
    #[cfg(all(feature = "MessageChannel", feature = "MessagePort",))]
    #[allow(bad_style)]
    #[doc = "The `port1` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/port1)\n\n*This API requires the following crate features to be activated: `MessageChannel`, `MessagePort`*"]
    #[allow(clippy::all)]
    pub fn port1(&self) -> MessagePort {
        #[cfg(all(feature = "MessageChannel", feature = "MessagePort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_port1_MessageChannel(
                self_: <&MessageChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MessagePort as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_port1_MessageChannel(
            self_: <&MessageChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MessagePort as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_port1_MessageChannel(self_)
            };
            <MessagePort as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MessageChannel", feature = "MessagePort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_port2_MessageChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MessageChannel as WasmDescribe>::describe();
    <MessagePort as WasmDescribe>::describe();
}
impl MessageChannel {
    #[cfg(all(feature = "MessageChannel", feature = "MessagePort",))]
    #[allow(bad_style)]
    #[doc = "The `port2` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/port2)\n\n*This API requires the following crate features to be activated: `MessageChannel`, `MessagePort`*"]
    #[allow(clippy::all)]
    pub fn port2(&self) -> MessagePort {
        #[cfg(all(feature = "MessageChannel", feature = "MessagePort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_port2_MessageChannel(
                self_: <&MessageChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MessagePort as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_port2_MessageChannel(
            self_: <&MessageChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MessagePort as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_port2_MessageChannel(self_)
            };
            <MessagePort as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_1b9cd0f240a6461c: [u8; 364usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\x01\0\0\0\0\x04\0\0\x02\x0EMessageChannel __widl_instanceof_MessageChannel\0\0\0\0\x1B__widl_f_new_MessageChannel\x01\0\0\x01\x0EMessageChannel\0\x01\0\x03new\0\0\0\x1D__widl_f_port1_MessageChannel\0\0\0\x01\x0EMessageChannel\x01\0\x01\x05port1\x01\x01\x05self_\x05port1\0\0\0\x1D__widl_f_port2_MessageChannel\0\0\0\x01\x0EMessageChannel\x01\0\x01\x05port2\x01\x01\x05self_\x05port2\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
