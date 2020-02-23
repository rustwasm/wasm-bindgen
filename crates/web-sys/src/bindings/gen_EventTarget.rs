use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `EventTarget` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget)\n\n*This API requires the following crate features to be activated: `EventTarget`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct EventTarget {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_EventTarget: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for EventTarget {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(84u32);
            inform(97u32);
            inform(114u32);
            inform(103u32);
            inform(101u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for EventTarget {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for EventTarget {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for EventTarget {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a EventTarget {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for EventTarget {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            EventTarget {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for EventTarget {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a EventTarget {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for EventTarget {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<EventTarget>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(EventTarget {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for EventTarget {
        #[inline]
        fn from(obj: JsValue) -> EventTarget {
            EventTarget { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for EventTarget {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<EventTarget> for EventTarget {
        #[inline]
        fn as_ref(&self) -> &EventTarget {
            self
        }
    }
    impl From<EventTarget> for JsValue {
        #[inline]
        fn from(obj: EventTarget) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for EventTarget {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_EventTarget(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_EventTarget(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_EventTarget(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            EventTarget { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const EventTarget) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<EventTarget> for ::js_sys::Object {
    #[inline]
    fn from(obj: EventTarget) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for EventTarget {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_EventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <EventTarget as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `new EventTarget(..)` constructor, creating a new instance of `EventTarget`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/EventTarget)\n\n*This API requires the following crate features to be activated: `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<EventTarget, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_EventTarget() -> <EventTarget as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_EventTarget(
        ) -> <EventTarget as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_EventTarget() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<EventTarget as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_event_listener_with_callback_EventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `addEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)\n\n*This API requires the following crate features to be activated: `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn add_event_listener_with_callback(
        &self,
        type_: &str,
        listener: &::js_sys::Function,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_event_listener_with_callback_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_event_listener_with_callback_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                __widl_f_add_event_listener_with_callback_EventTarget(self_, type_, listener)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "EventListener", feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_event_listener_with_event_listener_EventTarget()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&EventListener as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "EventListener", feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `addEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)\n\n*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn add_event_listener_with_event_listener(
        &self,
        type_: &str,
        listener: &EventListener,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventListener", feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_event_listener_with_event_listener_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_event_listener_with_event_listener_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                __widl_f_add_event_listener_with_event_listener_EventTarget(self_, type_, listener)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AddEventListenerOptions", feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_event_listener_with_callback_and_add_event_listener_options_EventTarget(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&AddEventListenerOptions as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "AddEventListenerOptions", feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `addEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)\n\n*This API requires the following crate features to be activated: `AddEventListenerOptions`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn add_event_listener_with_callback_and_add_event_listener_options(
        &self,
        type_: &str,
        listener: &::js_sys::Function,
        options: &AddEventListenerOptions,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AddEventListenerOptions", feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_event_listener_with_callback_and_add_event_listener_options_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&AddEventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_event_listener_with_callback_and_add_event_listener_options_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&AddEventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                let options =
                    <&AddEventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_add_event_listener_with_callback_and_add_event_listener_options_EventTarget(
                    self_, type_, listener, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(
    feature = "AddEventListenerOptions",
    feature = "EventListener",
    feature = "EventTarget",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_event_listener_with_event_listener_and_add_event_listener_options_EventTarget(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&EventListener as WasmDescribe>::describe();
    <&AddEventListenerOptions as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(
        feature = "AddEventListenerOptions",
        feature = "EventListener",
        feature = "EventTarget",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)\n\n*This API requires the following crate features to be activated: `AddEventListenerOptions`, `EventListener`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn add_event_listener_with_event_listener_and_add_event_listener_options(
        &self,
        type_: &str,
        listener: &EventListener,
        options: &AddEventListenerOptions,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "AddEventListenerOptions",
            feature = "EventListener",
            feature = "EventTarget",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_event_listener_with_event_listener_and_add_event_listener_options_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&AddEventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_event_listener_with_event_listener_and_add_event_listener_options_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&AddEventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                let options =
                    <&AddEventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_add_event_listener_with_event_listener_and_add_event_listener_options_EventTarget ( self_ , type_ , listener , options )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_event_listener_with_callback_and_bool_EventTarget(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `addEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)\n\n*This API requires the following crate features to be activated: `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn add_event_listener_with_callback_and_bool(
        &self,
        type_: &str,
        listener: &::js_sys::Function,
        options: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_event_listener_with_callback_and_bool_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_event_listener_with_callback_and_bool_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                let options = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_add_event_listener_with_callback_and_bool_EventTarget(
                    self_, type_, listener, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "EventListener", feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_event_listener_with_event_listener_and_bool_EventTarget(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&EventListener as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "EventListener", feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `addEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)\n\n*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn add_event_listener_with_event_listener_and_bool(
        &self,
        type_: &str,
        listener: &EventListener,
        options: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventListener", feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_event_listener_with_event_listener_and_bool_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_event_listener_with_event_listener_and_bool_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                let options = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_add_event_listener_with_event_listener_and_bool_EventTarget(
                    self_, type_, listener, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AddEventListenerOptions", feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_event_listener_with_callback_and_add_event_listener_options_and_wants_untrusted_EventTarget(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&AddEventListenerOptions as WasmDescribe>::describe();
    <Option<bool> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "AddEventListenerOptions", feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `addEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)\n\n*This API requires the following crate features to be activated: `AddEventListenerOptions`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn add_event_listener_with_callback_and_add_event_listener_options_and_wants_untrusted(
        &self,
        type_: &str,
        listener: &::js_sys::Function,
        options: &AddEventListenerOptions,
        wants_untrusted: Option<bool>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AddEventListenerOptions", feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_event_listener_with_callback_and_add_event_listener_options_and_wants_untrusted_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&AddEventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wants_untrusted: <Option<bool> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_event_listener_with_callback_and_add_event_listener_options_and_wants_untrusted_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&AddEventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wants_untrusted: <Option<bool> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            drop(options);
            drop(wants_untrusted);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                let options =
                    <&AddEventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                let wants_untrusted =
                    <Option<bool> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(wants_untrusted);
                __widl_f_add_event_listener_with_callback_and_add_event_listener_options_and_wants_untrusted_EventTarget ( self_ , type_ , listener , options , wants_untrusted )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(
    feature = "AddEventListenerOptions",
    feature = "EventListener",
    feature = "EventTarget",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_event_listener_with_event_listener_and_add_event_listener_options_and_wants_untrusted_EventTarget(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&EventListener as WasmDescribe>::describe();
    <&AddEventListenerOptions as WasmDescribe>::describe();
    <Option<bool> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(
        feature = "AddEventListenerOptions",
        feature = "EventListener",
        feature = "EventTarget",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)\n\n*This API requires the following crate features to be activated: `AddEventListenerOptions`, `EventListener`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn add_event_listener_with_event_listener_and_add_event_listener_options_and_wants_untrusted(
        &self,
        type_: &str,
        listener: &EventListener,
        options: &AddEventListenerOptions,
        wants_untrusted: Option<bool>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "AddEventListenerOptions",
            feature = "EventListener",
            feature = "EventTarget",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_event_listener_with_event_listener_and_add_event_listener_options_and_wants_untrusted_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&AddEventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wants_untrusted: <Option<bool> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_event_listener_with_event_listener_and_add_event_listener_options_and_wants_untrusted_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&AddEventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wants_untrusted: <Option<bool> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            drop(options);
            drop(wants_untrusted);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                let options =
                    <&AddEventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                let wants_untrusted =
                    <Option<bool> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(wants_untrusted);
                __widl_f_add_event_listener_with_event_listener_and_add_event_listener_options_and_wants_untrusted_EventTarget ( self_ , type_ , listener , options , wants_untrusted )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_event_listener_with_callback_and_bool_and_wants_untrusted_EventTarget(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<bool> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `addEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)\n\n*This API requires the following crate features to be activated: `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn add_event_listener_with_callback_and_bool_and_wants_untrusted(
        &self,
        type_: &str,
        listener: &::js_sys::Function,
        options: bool,
        wants_untrusted: Option<bool>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_event_listener_with_callback_and_bool_and_wants_untrusted_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wants_untrusted: <Option<bool> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_event_listener_with_callback_and_bool_and_wants_untrusted_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wants_untrusted: <Option<bool> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            drop(options);
            drop(wants_untrusted);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                let options = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let wants_untrusted =
                    <Option<bool> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(wants_untrusted);
                __widl_f_add_event_listener_with_callback_and_bool_and_wants_untrusted_EventTarget(
                    self_,
                    type_,
                    listener,
                    options,
                    wants_untrusted,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "EventListener", feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_event_listener_with_event_listener_and_bool_and_wants_untrusted_EventTarget(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&EventListener as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<bool> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "EventListener", feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `addEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)\n\n*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn add_event_listener_with_event_listener_and_bool_and_wants_untrusted(
        &self,
        type_: &str,
        listener: &EventListener,
        options: bool,
        wants_untrusted: Option<bool>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventListener", feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_event_listener_with_event_listener_and_bool_and_wants_untrusted_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wants_untrusted: <Option<bool> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_event_listener_with_event_listener_and_bool_and_wants_untrusted_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wants_untrusted: <Option<bool> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            drop(options);
            drop(wants_untrusted);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                let options = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let wants_untrusted =
                    <Option<bool> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(wants_untrusted);
                __widl_f_add_event_listener_with_event_listener_and_bool_and_wants_untrusted_EventTarget ( self_ , type_ , listener , options , wants_untrusted )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Event", feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dispatch_event_EventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&EventTarget as WasmDescribe>::describe();
    <&Event as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "Event", feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `dispatchEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/dispatchEvent)\n\n*This API requires the following crate features to be activated: `Event`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn dispatch_event(&self, event: &Event) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Event", feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dispatch_event_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dispatch_event_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(event);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let event = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(event);
                __widl_f_dispatch_event_EventTarget(self_, event)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_event_listener_with_callback_EventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `removeEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)\n\n*This API requires the following crate features to be activated: `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn remove_event_listener_with_callback(
        &self,
        type_: &str,
        listener: &::js_sys::Function,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_event_listener_with_callback_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_event_listener_with_callback_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                __widl_f_remove_event_listener_with_callback_EventTarget(self_, type_, listener)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "EventListener", feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_event_listener_with_event_listener_EventTarget(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&EventListener as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "EventListener", feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `removeEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)\n\n*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn remove_event_listener_with_event_listener(
        &self,
        type_: &str,
        listener: &EventListener,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventListener", feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_event_listener_with_event_listener_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_event_listener_with_event_listener_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                __widl_f_remove_event_listener_with_event_listener_EventTarget(
                    self_, type_, listener,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "EventListenerOptions", feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_event_listener_with_callback_and_event_listener_options_EventTarget(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&EventListenerOptions as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "EventListenerOptions", feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `removeEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)\n\n*This API requires the following crate features to be activated: `EventListenerOptions`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn remove_event_listener_with_callback_and_event_listener_options(
        &self,
        type_: &str,
        listener: &::js_sys::Function,
        options: &EventListenerOptions,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventListenerOptions", feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_event_listener_with_callback_and_event_listener_options_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&EventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_event_listener_with_callback_and_event_listener_options_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&EventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                let options =
                    <&EventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_remove_event_listener_with_callback_and_event_listener_options_EventTarget(
                    self_, type_, listener, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(
    feature = "EventListener",
    feature = "EventListenerOptions",
    feature = "EventTarget",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_event_listener_with_event_listener_and_event_listener_options_EventTarget(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&EventListener as WasmDescribe>::describe();
    <&EventListenerOptions as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(
        feature = "EventListener",
        feature = "EventListenerOptions",
        feature = "EventTarget",
    ))]
    #[allow(bad_style)]
    #[doc = "The `removeEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)\n\n*This API requires the following crate features to be activated: `EventListener`, `EventListenerOptions`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn remove_event_listener_with_event_listener_and_event_listener_options(
        &self,
        type_: &str,
        listener: &EventListener,
        options: &EventListenerOptions,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "EventListener",
            feature = "EventListenerOptions",
            feature = "EventTarget",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_event_listener_with_event_listener_and_event_listener_options_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&EventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_event_listener_with_event_listener_and_event_listener_options_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&EventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                let options =
                    <&EventListenerOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_remove_event_listener_with_event_listener_and_event_listener_options_EventTarget ( self_ , type_ , listener , options )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_event_listener_with_callback_and_bool_EventTarget(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `removeEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)\n\n*This API requires the following crate features to be activated: `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn remove_event_listener_with_callback_and_bool(
        &self,
        type_: &str,
        listener: &::js_sys::Function,
        options: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_event_listener_with_callback_and_bool_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_event_listener_with_callback_and_bool_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                let options = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_remove_event_listener_with_callback_and_bool_EventTarget(
                    self_, type_, listener, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "EventListener", feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_event_listener_with_event_listener_and_bool_EventTarget(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&EventTarget as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&EventListener as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventTarget {
    #[cfg(all(feature = "EventListener", feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `removeEventListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)\n\n*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn remove_event_listener_with_event_listener_and_bool(
        &self,
        type_: &str,
        listener: &EventListener,
        options: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventListener", feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_event_listener_with_event_listener_and_bool_EventTarget(
                self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_event_listener_with_event_listener_and_bool_EventTarget(
            self_: <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(listener);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let listener =
                    <&EventListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(listener);
                let options = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_remove_event_listener_with_event_listener_and_bool_EventTarget(
                    self_, type_, listener, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_303417fdbf7b6d4b: [u8; 2700usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}J\n\0\0\0\0\x13\0\0\x02\x0BEventTarget\x1D__widl_instanceof_EventTarget\0\0\0\0\x18__widl_f_new_EventTarget\x01\0\0\x01\x0BEventTarget\0\x01\0\x03new\0\0\05__widl_f_add_event_listener_with_callback_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x03\x05self_\x05type_\x08listener\x10addEventListener\0\0\0;__widl_f_add_event_listener_with_event_listener_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x03\x05self_\x05type_\x08listener\x10addEventListener\0\0\0T__widl_f_add_event_listener_with_callback_and_add_event_listener_options_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x04\x05self_\x05type_\x08listener\x07options\x10addEventListener\0\0\0Z__widl_f_add_event_listener_with_event_listener_and_add_event_listener_options_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x04\x05self_\x05type_\x08listener\x07options\x10addEventListener\0\0\0>__widl_f_add_event_listener_with_callback_and_bool_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x04\x05self_\x05type_\x08listener\x07options\x10addEventListener\0\0\0D__widl_f_add_event_listener_with_event_listener_and_bool_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x04\x05self_\x05type_\x08listener\x07options\x10addEventListener\0\0\0h__widl_f_add_event_listener_with_callback_and_add_event_listener_options_and_wants_untrusted_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x05\x05self_\x05type_\x08listener\x07options\x0Fwants_untrusted\x10addEventListener\0\0\0n__widl_f_add_event_listener_with_event_listener_and_add_event_listener_options_and_wants_untrusted_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x05\x05self_\x05type_\x08listener\x07options\x0Fwants_untrusted\x10addEventListener\0\0\0R__widl_f_add_event_listener_with_callback_and_bool_and_wants_untrusted_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x05\x05self_\x05type_\x08listener\x07options\x0Fwants_untrusted\x10addEventListener\0\0\0X__widl_f_add_event_listener_with_event_listener_and_bool_and_wants_untrusted_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x05\x05self_\x05type_\x08listener\x07options\x0Fwants_untrusted\x10addEventListener\0\0\0#__widl_f_dispatch_event_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x02\x05self_\x05event\rdispatchEvent\0\0\08__widl_f_remove_event_listener_with_callback_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x03\x05self_\x05type_\x08listener\x13removeEventListener\0\0\0>__widl_f_remove_event_listener_with_event_listener_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x03\x05self_\x05type_\x08listener\x13removeEventListener\0\0\0S__widl_f_remove_event_listener_with_callback_and_event_listener_options_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x04\x05self_\x05type_\x08listener\x07options\x13removeEventListener\0\0\0Y__widl_f_remove_event_listener_with_event_listener_and_event_listener_options_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x04\x05self_\x05type_\x08listener\x07options\x13removeEventListener\0\0\0A__widl_f_remove_event_listener_with_callback_and_bool_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x04\x05self_\x05type_\x08listener\x07options\x13removeEventListener\0\0\0G__widl_f_remove_event_listener_with_event_listener_and_bool_EventTarget\x01\0\0\x01\x0BEventTarget\x01\0\0\x01\x04\x05self_\x05type_\x08listener\x07options\x13removeEventListener\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
