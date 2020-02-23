use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CustomEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CustomEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CustomEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CustomEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(67u32);
            inform(117u32);
            inform(115u32);
            inform(116u32);
            inform(111u32);
            inform(109u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for CustomEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for CustomEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CustomEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CustomEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CustomEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CustomEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CustomEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CustomEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CustomEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CustomEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CustomEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CustomEvent {
        #[inline]
        fn from(obj: JsValue) -> CustomEvent {
            CustomEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CustomEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CustomEvent> for CustomEvent {
        #[inline]
        fn as_ref(&self) -> &CustomEvent {
            self
        }
    }
    impl From<CustomEvent> for JsValue {
        #[inline]
        fn from(obj: CustomEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CustomEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CustomEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CustomEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CustomEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CustomEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CustomEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CustomEvent> for Event {
    #[inline]
    fn from(obj: CustomEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for CustomEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CustomEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: CustomEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CustomEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CustomEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_CustomEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <CustomEvent as WasmDescribe>::describe();
}
impl CustomEvent {
    #[cfg(all(feature = "CustomEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new CustomEvent(..)` constructor, creating a new instance of `CustomEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/CustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<CustomEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CustomEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_CustomEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CustomEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_CustomEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CustomEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_CustomEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<CustomEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CustomEvent", feature = "CustomEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_CustomEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&CustomEventInit as WasmDescribe>::describe();
    <CustomEvent as WasmDescribe>::describe();
}
impl CustomEvent {
    #[cfg(all(feature = "CustomEvent", feature = "CustomEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new CustomEvent(..)` constructor, creating a new instance of `CustomEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/CustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`, `CustomEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &CustomEventInit,
    ) -> Result<CustomEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CustomEvent", feature = "CustomEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_CustomEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&CustomEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CustomEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_CustomEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&CustomEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CustomEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&CustomEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_CustomEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<CustomEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CustomEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_custom_event_CustomEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CustomEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CustomEvent {
    #[cfg(all(feature = "CustomEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initCustomEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
    #[allow(clippy::all)]
    pub fn init_custom_event(&self, type_: &str) {
        #[cfg(all(feature = "CustomEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_custom_event_CustomEvent(
                self_: <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_custom_event_CustomEvent(
            self_: <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_init_custom_event_CustomEvent(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CustomEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_custom_event_with_can_bubble_CustomEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CustomEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CustomEvent {
    #[cfg(all(feature = "CustomEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initCustomEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
    #[allow(clippy::all)]
    pub fn init_custom_event_with_can_bubble(&self, type_: &str, can_bubble: bool) {
        #[cfg(all(feature = "CustomEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_custom_event_with_can_bubble_CustomEvent(
                self_: <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_custom_event_with_can_bubble_CustomEvent(
            self_: <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                __widl_f_init_custom_event_with_can_bubble_CustomEvent(self_, type_, can_bubble)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CustomEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_custom_event_with_can_bubble_and_cancelable_CustomEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CustomEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CustomEvent {
    #[cfg(all(feature = "CustomEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initCustomEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
    #[allow(clippy::all)]
    pub fn init_custom_event_with_can_bubble_and_cancelable(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    ) {
        #[cfg(all(feature = "CustomEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_custom_event_with_can_bubble_and_cancelable_CustomEvent(
                self_: <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_custom_event_with_can_bubble_and_cancelable_CustomEvent(
            self_: <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                __widl_f_init_custom_event_with_can_bubble_and_cancelable_CustomEvent(
                    self_, type_, can_bubble, cancelable,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "CustomEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_custom_event_with_can_bubble_and_cancelable_and_detail_CustomEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CustomEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CustomEvent {
    #[cfg(all(feature = "CustomEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initCustomEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
    #[allow(clippy::all)]
    pub fn init_custom_event_with_can_bubble_and_cancelable_and_detail(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        detail: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "CustomEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_custom_event_with_can_bubble_and_cancelable_and_detail_CustomEvent(
                self_: <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_custom_event_with_can_bubble_and_cancelable_and_detail_CustomEvent(
            self_: <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(detail);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let detail =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        detail,
                    );
                __widl_f_init_custom_event_with_can_bubble_and_cancelable_and_detail_CustomEvent(
                    self_, type_, can_bubble, cancelable, detail,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "CustomEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_detail_CustomEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CustomEvent as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl CustomEvent {
    #[cfg(all(feature = "CustomEvent",))]
    #[allow(bad_style)]
    #[doc = "The `detail` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/detail)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
    #[allow(clippy::all)]
    pub fn detail(&self) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "CustomEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_detail_CustomEvent(
                self_: <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_detail_CustomEvent(
            self_: <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CustomEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_detail_CustomEvent(self_)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5f2592aa104bc533: [u8; 889usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}7\x03\0\0\0\0\x08\0\0\x02\x0BCustomEvent\x1D__widl_instanceof_CustomEvent\0\0\0\0\x18__widl_f_new_CustomEvent\x01\0\0\x01\x0BCustomEvent\0\x01\x01\x05type_\x03new\0\0\0-__widl_f_new_with_event_init_dict_CustomEvent\x01\0\0\x01\x0BCustomEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0&__widl_f_init_custom_event_CustomEvent\0\0\0\x01\x0BCustomEvent\x01\0\0\x01\x02\x05self_\x05type_\x0FinitCustomEvent\0\0\06__widl_f_init_custom_event_with_can_bubble_CustomEvent\0\0\0\x01\x0BCustomEvent\x01\0\0\x01\x03\x05self_\x05type_\ncan_bubble\x0FinitCustomEvent\0\0\0E__widl_f_init_custom_event_with_can_bubble_and_cancelable_CustomEvent\0\0\0\x01\x0BCustomEvent\x01\0\0\x01\x04\x05self_\x05type_\ncan_bubble\ncancelable\x0FinitCustomEvent\0\0\0P__widl_f_init_custom_event_with_can_bubble_and_cancelable_and_detail_CustomEvent\0\0\0\x01\x0BCustomEvent\x01\0\0\x01\x05\x05self_\x05type_\ncan_bubble\ncancelable\x06detail\x0FinitCustomEvent\0\0\0\x1B__widl_f_detail_CustomEvent\0\0\0\x01\x0BCustomEvent\x01\0\x01\x06detail\x01\x01\x05self_\x06detail\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
