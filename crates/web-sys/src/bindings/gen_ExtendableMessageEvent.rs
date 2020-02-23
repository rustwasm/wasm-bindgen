use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ExtendableMessageEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ExtendableMessageEvent {
    obj: ExtendableEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ExtendableMessageEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ExtendableMessageEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
            inform(69u32);
            inform(120u32);
            inform(116u32);
            inform(101u32);
            inform(110u32);
            inform(100u32);
            inform(97u32);
            inform(98u32);
            inform(108u32);
            inform(101u32);
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
    impl core::ops::Deref for ExtendableMessageEvent {
        type Target = ExtendableEvent;
        #[inline]
        fn deref(&self) -> &ExtendableEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for ExtendableMessageEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ExtendableMessageEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ExtendableMessageEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ExtendableMessageEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ExtendableMessageEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ExtendableMessageEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ExtendableMessageEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ExtendableMessageEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ExtendableMessageEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ExtendableMessageEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ExtendableMessageEvent {
        #[inline]
        fn from(obj: JsValue) -> ExtendableMessageEvent {
            ExtendableMessageEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ExtendableMessageEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ExtendableMessageEvent> for ExtendableMessageEvent {
        #[inline]
        fn as_ref(&self) -> &ExtendableMessageEvent {
            self
        }
    }
    impl From<ExtendableMessageEvent> for JsValue {
        #[inline]
        fn from(obj: ExtendableMessageEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ExtendableMessageEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ExtendableMessageEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ExtendableMessageEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ExtendableMessageEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ExtendableMessageEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ExtendableMessageEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ExtendableMessageEvent> for ExtendableEvent {
    #[inline]
    fn from(obj: ExtendableMessageEvent) -> ExtendableEvent {
        use wasm_bindgen::JsCast;
        ExtendableEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<ExtendableEvent> for ExtendableMessageEvent {
    #[inline]
    fn as_ref(&self) -> &ExtendableEvent {
        use wasm_bindgen::JsCast;
        ExtendableEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ExtendableMessageEvent> for Event {
    #[inline]
    fn from(obj: ExtendableMessageEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for ExtendableMessageEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ExtendableMessageEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: ExtendableMessageEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ExtendableMessageEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ExtendableMessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_ExtendableMessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <ExtendableMessageEvent as WasmDescribe>::describe();
}
impl ExtendableMessageEvent {
    #[cfg(all(feature = "ExtendableMessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new ExtendableMessageEvent(..)` constructor, creating a new instance of `ExtendableMessageEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ExtendableMessageEvent)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<ExtendableMessageEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ExtendableMessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_ExtendableMessageEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ExtendableMessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_ExtendableMessageEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ExtendableMessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_ExtendableMessageEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ExtendableMessageEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "ExtendableMessageEvent",
    feature = "ExtendableMessageEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_ExtendableMessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&ExtendableMessageEventInit as WasmDescribe>::describe();
    <ExtendableMessageEvent as WasmDescribe>::describe();
}
impl ExtendableMessageEvent {
    #[cfg(all(
        feature = "ExtendableMessageEvent",
        feature = "ExtendableMessageEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new ExtendableMessageEvent(..)` constructor, creating a new instance of `ExtendableMessageEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ExtendableMessageEvent)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`, `ExtendableMessageEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ExtendableMessageEventInit,
    ) -> Result<ExtendableMessageEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ExtendableMessageEvent",
            feature = "ExtendableMessageEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_ExtendableMessageEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & ExtendableMessageEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <ExtendableMessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_ExtendableMessageEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & ExtendableMessageEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <ExtendableMessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&ExtendableMessageEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_ExtendableMessageEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ExtendableMessageEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ExtendableMessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_ExtendableMessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ExtendableMessageEvent as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl ExtendableMessageEvent {
    #[cfg(all(feature = "ExtendableMessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/data)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "ExtendableMessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_ExtendableMessageEvent(
                self_: <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_ExtendableMessageEvent(
            self_: <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_ExtendableMessageEvent(self_)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ExtendableMessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_origin_ExtendableMessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ExtendableMessageEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl ExtendableMessageEvent {
    #[cfg(all(feature = "ExtendableMessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `origin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/origin)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    #[allow(clippy::all)]
    pub fn origin(&self) -> String {
        #[cfg(all(feature = "ExtendableMessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_origin_ExtendableMessageEvent(
                self_: <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_origin_ExtendableMessageEvent(
            self_: <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_origin_ExtendableMessageEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ExtendableMessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_last_event_id_ExtendableMessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ExtendableMessageEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl ExtendableMessageEvent {
    #[cfg(all(feature = "ExtendableMessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `lastEventId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/lastEventId)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    #[allow(clippy::all)]
    pub fn last_event_id(&self) -> String {
        #[cfg(all(feature = "ExtendableMessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_last_event_id_ExtendableMessageEvent(
                self_: <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_last_event_id_ExtendableMessageEvent(
            self_: <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_last_event_id_ExtendableMessageEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ExtendableMessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_source_ExtendableMessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ExtendableMessageEvent as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl ExtendableMessageEvent {
    #[cfg(all(feature = "ExtendableMessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `source` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/source)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    #[allow(clippy::all)]
    pub fn source(&self) -> Option<::js_sys::Object> {
        #[cfg(all(feature = "ExtendableMessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_source_ExtendableMessageEvent(
                self_: <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_source_ExtendableMessageEvent(
            self_: <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_source_ExtendableMessageEvent(self_)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ExtendableMessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ports_ExtendableMessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ExtendableMessageEvent as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl ExtendableMessageEvent {
    #[cfg(all(feature = "ExtendableMessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `ports` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ports)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    #[allow(clippy::all)]
    pub fn ports(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "ExtendableMessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ports_ExtendableMessageEvent(
                self_: <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ports_ExtendableMessageEvent(
            self_: <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ExtendableMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ports_ExtendableMessageEvent(self_)
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
pub static __WASM_BINDGEN_GENERATED_a9c52120e1aea788: [u8; 846usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x0C\x03\0\0\0\0\x08\0\0\x02\x16ExtendableMessageEvent(__widl_instanceof_ExtendableMessageEvent\0\0\0\0#__widl_f_new_ExtendableMessageEvent\x01\0\0\x01\x16ExtendableMessageEvent\0\x01\x01\x05type_\x03new\0\0\08__widl_f_new_with_event_init_dict_ExtendableMessageEvent\x01\0\0\x01\x16ExtendableMessageEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0$__widl_f_data_ExtendableMessageEvent\0\0\0\x01\x16ExtendableMessageEvent\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0&__widl_f_origin_ExtendableMessageEvent\0\0\0\x01\x16ExtendableMessageEvent\x01\0\x01\x06origin\x01\x01\x05self_\x06origin\0\0\0-__widl_f_last_event_id_ExtendableMessageEvent\0\0\0\x01\x16ExtendableMessageEvent\x01\0\x01\x0BlastEventId\x01\x01\x05self_\x0BlastEventId\0\0\0&__widl_f_source_ExtendableMessageEvent\0\0\0\x01\x16ExtendableMessageEvent\x01\0\x01\x06source\x01\x01\x05self_\x06source\0\0\0%__widl_f_ports_ExtendableMessageEvent\0\0\0\x01\x16ExtendableMessageEvent\x01\0\x01\x05ports\x01\x01\x05self_\x05ports\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
