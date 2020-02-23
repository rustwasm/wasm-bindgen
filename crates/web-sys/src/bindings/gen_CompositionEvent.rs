use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CompositionEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CompositionEvent {
    obj: UiEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CompositionEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CompositionEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(67u32);
            inform(111u32);
            inform(109u32);
            inform(112u32);
            inform(111u32);
            inform(115u32);
            inform(105u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for CompositionEvent {
        type Target = UiEvent;
        #[inline]
        fn deref(&self) -> &UiEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for CompositionEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CompositionEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CompositionEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CompositionEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CompositionEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CompositionEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CompositionEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CompositionEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CompositionEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CompositionEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CompositionEvent {
        #[inline]
        fn from(obj: JsValue) -> CompositionEvent {
            CompositionEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CompositionEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CompositionEvent> for CompositionEvent {
        #[inline]
        fn as_ref(&self) -> &CompositionEvent {
            self
        }
    }
    impl From<CompositionEvent> for JsValue {
        #[inline]
        fn from(obj: CompositionEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CompositionEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CompositionEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CompositionEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CompositionEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CompositionEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CompositionEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CompositionEvent> for UiEvent {
    #[inline]
    fn from(obj: CompositionEvent) -> UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<UiEvent> for CompositionEvent {
    #[inline]
    fn as_ref(&self) -> &UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CompositionEvent> for Event {
    #[inline]
    fn from(obj: CompositionEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for CompositionEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CompositionEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: CompositionEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CompositionEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CompositionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_CompositionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <CompositionEvent as WasmDescribe>::describe();
}
impl CompositionEvent {
    #[cfg(all(feature = "CompositionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new CompositionEvent(..)` constructor, creating a new instance of `CompositionEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/CompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<CompositionEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CompositionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_CompositionEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CompositionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_CompositionEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CompositionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_CompositionEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<CompositionEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CompositionEvent", feature = "CompositionEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_CompositionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&CompositionEventInit as WasmDescribe>::describe();
    <CompositionEvent as WasmDescribe>::describe();
}
impl CompositionEvent {
    #[cfg(all(feature = "CompositionEvent", feature = "CompositionEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new CompositionEvent(..)` constructor, creating a new instance of `CompositionEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/CompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`, `CompositionEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &CompositionEventInit,
    ) -> Result<CompositionEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CompositionEvent", feature = "CompositionEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_CompositionEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&CompositionEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CompositionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_CompositionEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&CompositionEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CompositionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&CompositionEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_CompositionEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<CompositionEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CompositionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_composition_event_CompositionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CompositionEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CompositionEvent {
    #[cfg(all(feature = "CompositionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initCompositionEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
    #[allow(clippy::all)]
    pub fn init_composition_event(&self, type_arg: &str) {
        #[cfg(all(feature = "CompositionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_composition_event_CompositionEvent(
                self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_composition_event_CompositionEvent(
            self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                __widl_f_init_composition_event_CompositionEvent(self_, type_arg)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CompositionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_composition_event_with_can_bubble_arg_CompositionEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CompositionEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CompositionEvent {
    #[cfg(all(feature = "CompositionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initCompositionEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
    #[allow(clippy::all)]
    pub fn init_composition_event_with_can_bubble_arg(&self, type_arg: &str, can_bubble_arg: bool) {
        #[cfg(all(feature = "CompositionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_composition_event_with_can_bubble_arg_CompositionEvent(
                self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_composition_event_with_can_bubble_arg_CompositionEvent(
            self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                __widl_f_init_composition_event_with_can_bubble_arg_CompositionEvent(
                    self_,
                    type_arg,
                    can_bubble_arg,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "CompositionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_CompositionEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CompositionEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CompositionEvent {
    #[cfg(all(feature = "CompositionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initCompositionEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
    #[allow(clippy::all)]
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
    ) {
        #[cfg(all(feature = "CompositionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_CompositionEvent(
                self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_CompositionEvent(
            self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                __widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_CompositionEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "CompositionEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_CompositionEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CompositionEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CompositionEvent {
    #[cfg(all(feature = "CompositionEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initCompositionEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
    ) {
        #[cfg(all(feature = "CompositionEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_CompositionEvent(
                self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_CompositionEvent(
            self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                __widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_CompositionEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "CompositionEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_CompositionEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CompositionEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CompositionEvent {
    #[cfg(all(feature = "CompositionEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initCompositionEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        data_arg: Option<&str>,
    ) {
        #[cfg(all(feature = "CompositionEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_CompositionEvent(
                self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_arg: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_CompositionEvent(
            self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_arg: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(data_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let data_arg =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data_arg);
                __widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_CompositionEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg , data_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "CompositionEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_and_locale_arg_CompositionEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CompositionEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CompositionEvent {
    #[cfg(all(feature = "CompositionEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initCompositionEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_and_locale_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        data_arg: Option<&str>,
        locale_arg: &str,
    ) {
        #[cfg(all(feature = "CompositionEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_and_locale_arg_CompositionEvent(
                self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_arg: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                locale_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_and_locale_arg_CompositionEvent(
            self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_arg: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            locale_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(data_arg);
            drop(locale_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let data_arg =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data_arg);
                let locale_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(locale_arg);
                __widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_and_locale_arg_CompositionEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg , data_arg , locale_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "CompositionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_CompositionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CompositionEvent as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl CompositionEvent {
    #[cfg(all(feature = "CompositionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/data)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> Option<String> {
        #[cfg(all(feature = "CompositionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_CompositionEvent(
                self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_CompositionEvent(
            self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_CompositionEvent(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CompositionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_locale_CompositionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CompositionEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CompositionEvent {
    #[cfg(all(feature = "CompositionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `locale` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/locale)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
    #[allow(clippy::all)]
    pub fn locale(&self) -> String {
        #[cfg(all(feature = "CompositionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_locale_CompositionEvent(
                self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_locale_CompositionEvent(
            self_: <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CompositionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_locale_CompositionEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_39e27032764aade1: [u8; 1622usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x14\x06\0\0\0\0\x0B\0\0\x02\x10CompositionEvent\"__widl_instanceof_CompositionEvent\0\0\0\0\x1D__widl_f_new_CompositionEvent\x01\0\0\x01\x10CompositionEvent\0\x01\x01\x05type_\x03new\0\0\02__widl_f_new_with_event_init_dict_CompositionEvent\x01\0\0\x01\x10CompositionEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\00__widl_f_init_composition_event_CompositionEvent\0\0\0\x01\x10CompositionEvent\x01\0\0\x01\x02\x05self_\x08type_arg\x14initCompositionEvent\0\0\0D__widl_f_init_composition_event_with_can_bubble_arg_CompositionEvent\0\0\0\x01\x10CompositionEvent\x01\0\0\x01\x03\x05self_\x08type_arg\x0Ecan_bubble_arg\x14initCompositionEvent\0\0\0W__widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_CompositionEvent\0\0\0\x01\x10CompositionEvent\x01\0\0\x01\x04\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x14initCompositionEvent\0\0\0d__widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_CompositionEvent\0\0\0\x01\x10CompositionEvent\x01\0\0\x01\x05\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\x14initCompositionEvent\0\0\0q__widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_CompositionEvent\0\0\0\x01\x10CompositionEvent\x01\0\0\x01\x06\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\x08data_arg\x14initCompositionEvent\0\0\0\x80\x01__widl_f_init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_and_locale_arg_CompositionEvent\0\0\0\x01\x10CompositionEvent\x01\0\0\x01\x07\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\x08data_arg\nlocale_arg\x14initCompositionEvent\0\0\0\x1E__widl_f_data_CompositionEvent\0\0\0\x01\x10CompositionEvent\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0 __widl_f_locale_CompositionEvent\0\0\0\x01\x10CompositionEvent\x01\0\x01\x06locale\x01\x01\x05self_\x06locale\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
