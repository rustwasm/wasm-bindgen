use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MessageEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MessageEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MessageEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MessageEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(77u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for MessageEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for MessageEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MessageEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MessageEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MessageEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MessageEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MessageEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MessageEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MessageEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MessageEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MessageEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MessageEvent {
        #[inline]
        fn from(obj: JsValue) -> MessageEvent {
            MessageEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MessageEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MessageEvent> for MessageEvent {
        #[inline]
        fn as_ref(&self) -> &MessageEvent {
            self
        }
    }
    impl From<MessageEvent> for JsValue {
        #[inline]
        fn from(obj: MessageEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MessageEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MessageEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MessageEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MessageEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MessageEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MessageEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MessageEvent> for Event {
    #[inline]
    fn from(obj: MessageEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for MessageEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MessageEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: MessageEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MessageEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <MessageEvent as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new MessageEvent(..)` constructor, creating a new instance of `MessageEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/MessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<MessageEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MessageEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MessageEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_MessageEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MessageEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MessageEvent", feature = "MessageEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&MessageEventInit as WasmDescribe>::describe();
    <MessageEvent as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent", feature = "MessageEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new MessageEvent(..)` constructor, creating a new instance of `MessageEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/MessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`, `MessageEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MessageEventInit,
    ) -> Result<MessageEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MessageEvent", feature = "MessageEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_MessageEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&MessageEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_MessageEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&MessageEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            drop(event_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let event_init_dict =
                    <&MessageEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_MessageEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MessageEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_message_event(&self, type_: &str) {
        #[cfg(all(feature = "MessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_init_message_event_MessageEvent(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles(&self, type_: &str, bubbles: bool) {
        #[cfg(all(feature = "MessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                __widl_f_init_message_event_with_bubbles_MessageEvent(self_, type_, bubbles)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
    ) {
        #[cfg(all(feature = "MessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                __widl_f_init_message_event_with_bubbles_and_cancelable_MessageEvent(
                    self_, type_, bubbles, cancelable,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_and_data_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "MessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_MessageEvent(
                    self_, type_, bubbles, cancelable, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
    ) {
        #[cfg(all(feature = "MessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            drop(data);
            drop(origin);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let origin = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin);
                __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_MessageEvent ( self_ , type_ , bubbles , cancelable , data , origin )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
    ) {
        #[cfg(all(feature = "MessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            drop(data);
            drop(origin);
            drop(last_event_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let origin = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin);
                let last_event_id =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(last_event_id);
                __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_MessageEvent ( self_ , type_ , bubbles , cancelable , data , origin , last_event_id )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MessageEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&Window>,
    ) {
        #[cfg(all(feature = "MessageEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            drop(data);
            drop(origin);
            drop(last_event_id);
            drop(source);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let origin = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin);
                let last_event_id =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(last_event_id);
                let source =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_MessageEvent ( self_ , type_ , bubbles , cancelable , data , origin , last_event_id , source )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MessageEvent", feature = "MessagePort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&MessagePort> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent", feature = "MessagePort",))]
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`, `MessagePort`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&MessagePort>,
    ) {
        #[cfg(all(feature = "MessageEvent", feature = "MessagePort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <Option<&MessagePort> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <Option<&MessagePort> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            drop(data);
            drop(origin);
            drop(last_event_id);
            drop(source);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let origin = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin);
                let last_event_id =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(last_event_id);
                let source =
                    <Option<&MessagePort> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_MessageEvent ( self_ , type_ , bubbles , cancelable , data , origin , last_event_id , source )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MessageEvent", feature = "ServiceWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&ServiceWorker> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent", feature = "ServiceWorker",))]
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`, `ServiceWorker`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&ServiceWorker>,
    ) {
        #[cfg(all(feature = "MessageEvent", feature = "ServiceWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <Option<&ServiceWorker> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <Option<&ServiceWorker> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            drop(data);
            drop(origin);
            drop(last_event_id);
            drop(source);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let origin = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin);
                let last_event_id =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(last_event_id);
                let source =
                    <Option<&ServiceWorker> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        source,
                    );
                __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_MessageEvent ( self_ , type_ , bubbles , cancelable , data , origin , last_event_id , source )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MessageEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_and_ports_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_and_ports(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&Window>,
        ports: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "MessageEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_and_ports_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ports: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_and_ports_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ports: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            drop(data);
            drop(origin);
            drop(last_event_id);
            drop(source);
            drop(ports);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let origin = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin);
                let last_event_id =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(last_event_id);
                let source =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                let ports =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ports,
                    );
                __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_and_ports_MessageEvent ( self_ , type_ , bubbles , cancelable , data , origin , last_event_id , source , ports )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MessageEvent", feature = "MessagePort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_and_ports_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&MessagePort> as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent", feature = "MessagePort",))]
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`, `MessagePort`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_and_ports(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&MessagePort>,
        ports: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "MessageEvent", feature = "MessagePort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_and_ports_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <Option<&MessagePort> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ports: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_and_ports_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <Option<&MessagePort> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ports: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            drop(data);
            drop(origin);
            drop(last_event_id);
            drop(source);
            drop(ports);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let origin = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin);
                let last_event_id =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(last_event_id);
                let source =
                    <Option<&MessagePort> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                let ports =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ports,
                    );
                __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_and_ports_MessageEvent ( self_ , type_ , bubbles , cancelable , data , origin , last_event_id , source , ports )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MessageEvent", feature = "ServiceWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_and_ports_MessageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&MessageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&ServiceWorker> as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent", feature = "ServiceWorker",))]
    #[allow(bad_style)]
    #[doc = "The `initMessageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)\n\n*This API requires the following crate features to be activated: `MessageEvent`, `ServiceWorker`*"]
    #[allow(clippy::all)]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_and_ports(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&ServiceWorker>,
        ports: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "MessageEvent", feature = "ServiceWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_and_ports_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <Option<&ServiceWorker> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ports: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_and_ports_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            last_event_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <Option<&ServiceWorker> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ports: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            drop(data);
            drop(origin);
            drop(last_event_id);
            drop(source);
            drop(ports);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let origin = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin);
                let last_event_id =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(last_event_id);
                let source =
                    <Option<&ServiceWorker> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        source,
                    );
                let ports =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ports,
                    );
                __widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_and_ports_MessageEvent ( self_ , type_ , bubbles , cancelable , data , origin , last_event_id , source , ports )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MessageEvent as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/data)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "MessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_MessageEvent(self_)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_origin_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MessageEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `origin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/origin)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn origin(&self) -> String {
        #[cfg(all(feature = "MessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_origin_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_origin_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_origin_MessageEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_last_event_id_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MessageEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `lastEventId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/lastEventId)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn last_event_id(&self) -> String {
        #[cfg(all(feature = "MessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_last_event_id_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_last_event_id_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_last_event_id_MessageEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_source_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MessageEvent as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `source` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/source)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn source(&self) -> Option<::js_sys::Object> {
        #[cfg(all(feature = "MessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_source_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_source_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_source_MessageEvent(self_)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ports_MessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MessageEvent as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl MessageEvent {
    #[cfg(all(feature = "MessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `ports` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/ports)\n\n*This API requires the following crate features to be activated: `MessageEvent`*"]
    #[allow(clippy::all)]
    pub fn ports(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "MessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ports_MessageEvent(
                self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ports_MessageEvent(
            self_: <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ports_MessageEvent(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d5bae3bbde83e944: [u8; 3023usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x8D\x0B\0\0\0\0\x14\0\0\x02\x0CMessageEvent\x1E__widl_instanceof_MessageEvent\0\0\0\0\x19__widl_f_new_MessageEvent\x01\0\0\x01\x0CMessageEvent\0\x01\x01\x05type_\x03new\0\0\0.__widl_f_new_with_event_init_dict_MessageEvent\x01\0\0\x01\x0CMessageEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0(__widl_f_init_message_event_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x02\x05self_\x05type_\x10initMessageEvent\0\0\05__widl_f_init_message_event_with_bubbles_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x03\x05self_\x05type_\x07bubbles\x10initMessageEvent\0\0\0D__widl_f_init_message_event_with_bubbles_and_cancelable_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x04\x05self_\x05type_\x07bubbles\ncancelable\x10initMessageEvent\0\0\0M__widl_f_init_message_event_with_bubbles_and_cancelable_and_data_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x05\x05self_\x05type_\x07bubbles\ncancelable\x04data\x10initMessageEvent\0\0\0X__widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x06\x05self_\x05type_\x07bubbles\ncancelable\x04data\x06origin\x10initMessageEvent\0\0\0j__widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x07\x05self_\x05type_\x07bubbles\ncancelable\x04data\x06origin\rlast_event_id\x10initMessageEvent\0\0\0y__widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x08\x05self_\x05type_\x07bubbles\ncancelable\x04data\x06origin\rlast_event_id\x06source\x10initMessageEvent\0\0\0\x7F__widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x08\x05self_\x05type_\x07bubbles\ncancelable\x04data\x06origin\rlast_event_id\x06source\x10initMessageEvent\0\0\0\x81\x01__widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\x08\x05self_\x05type_\x07bubbles\ncancelable\x04data\x06origin\rlast_event_id\x06source\x10initMessageEvent\0\0\0\x83\x01__widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_and_ports_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\t\x05self_\x05type_\x07bubbles\ncancelable\x04data\x06origin\rlast_event_id\x06source\x05ports\x10initMessageEvent\0\0\0\x89\x01__widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_and_ports_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\t\x05self_\x05type_\x07bubbles\ncancelable\x04data\x06origin\rlast_event_id\x06source\x05ports\x10initMessageEvent\0\0\0\x8B\x01__widl_f_init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_and_ports_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\0\x01\t\x05self_\x05type_\x07bubbles\ncancelable\x04data\x06origin\rlast_event_id\x06source\x05ports\x10initMessageEvent\0\0\0\x1A__widl_f_data_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0\x1C__widl_f_origin_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\x01\x06origin\x01\x01\x05self_\x06origin\0\0\0#__widl_f_last_event_id_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\x01\x0BlastEventId\x01\x01\x05self_\x0BlastEventId\0\0\0\x1C__widl_f_source_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\x01\x06source\x01\x01\x05self_\x06source\0\0\0\x1B__widl_f_ports_MessageEvent\0\0\0\x01\x0CMessageEvent\x01\0\x01\x05ports\x01\x01\x05self_\x05ports\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
