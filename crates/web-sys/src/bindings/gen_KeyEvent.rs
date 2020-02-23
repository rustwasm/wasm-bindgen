use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `KeyEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent)\n\n*This API requires the following crate features to be activated: `KeyEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct KeyEvent {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_KeyEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for KeyEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(75u32);
            inform(101u32);
            inform(121u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for KeyEvent {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for KeyEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for KeyEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a KeyEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for KeyEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            KeyEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for KeyEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a KeyEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for KeyEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<KeyEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(KeyEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for KeyEvent {
        #[inline]
        fn from(obj: JsValue) -> KeyEvent {
            KeyEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for KeyEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<KeyEvent> for KeyEvent {
        #[inline]
        fn as_ref(&self) -> &KeyEvent {
            self
        }
    }
    impl From<KeyEvent> for JsValue {
        #[inline]
        fn from(obj: KeyEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for KeyEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_KeyEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_KeyEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_KeyEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            KeyEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const KeyEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<KeyEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: KeyEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for KeyEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "KeyEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_key_event_KeyEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&KeyEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyEvent {
    #[cfg(all(feature = "KeyEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)\n\n*This API requires the following crate features to be activated: `KeyEvent`*"]
    #[allow(clippy::all)]
    pub fn init_key_event(&self, type_: &str) {
        #[cfg(all(feature = "KeyEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_key_event_KeyEvent(
                self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_key_event_KeyEvent(
            self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_init_key_event_KeyEvent(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "KeyEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_key_event_with_can_bubble_KeyEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&KeyEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyEvent {
    #[cfg(all(feature = "KeyEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)\n\n*This API requires the following crate features to be activated: `KeyEvent`*"]
    #[allow(clippy::all)]
    pub fn init_key_event_with_can_bubble(&self, type_: &str, can_bubble: bool) {
        #[cfg(all(feature = "KeyEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_key_event_with_can_bubble_KeyEvent(
                self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_key_event_with_can_bubble_KeyEvent(
            self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                __widl_f_init_key_event_with_can_bubble_KeyEvent(self_, type_, can_bubble)
            };
            ()
        }
    }
}
#[cfg(all(feature = "KeyEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_key_event_with_can_bubble_and_cancelable_KeyEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&KeyEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyEvent {
    #[cfg(all(feature = "KeyEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)\n\n*This API requires the following crate features to be activated: `KeyEvent`*"]
    #[allow(clippy::all)]
    pub fn init_key_event_with_can_bubble_and_cancelable(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    ) {
        #[cfg(all(feature = "KeyEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_key_event_with_can_bubble_and_cancelable_KeyEvent(
                self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_key_event_with_can_bubble_and_cancelable_KeyEvent(
            self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                __widl_f_init_key_event_with_can_bubble_and_cancelable_KeyEvent(
                    self_, type_, can_bubble, cancelable,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "KeyEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_KeyEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&KeyEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyEvent {
    #[cfg(all(feature = "KeyEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)\n\n*This API requires the following crate features to be activated: `KeyEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
    ) {
        #[cfg(all(feature = "KeyEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_KeyEvent(
                self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_KeyEvent(
            self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_KeyEvent(
                    self_, type_, can_bubble, cancelable, view,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "KeyEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_KeyEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&KeyEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyEvent {
    #[cfg(all(feature = "KeyEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)\n\n*This API requires the following crate features to be activated: `KeyEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        ctrl_key: bool,
    ) {
        #[cfg(all(feature = "KeyEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_KeyEvent(
                self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_KeyEvent(
            self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(ctrl_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_KeyEvent ( self_ , type_ , can_bubble , cancelable , view , ctrl_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "KeyEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_KeyEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&KeyEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyEvent {
    #[cfg(all(feature = "KeyEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)\n\n*This API requires the following crate features to be activated: `KeyEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        ctrl_key: bool,
        alt_key: bool,
    ) {
        #[cfg(all(feature = "KeyEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_KeyEvent(
                self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_KeyEvent(
            self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(ctrl_key);
            drop(alt_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_KeyEvent ( self_ , type_ , can_bubble , cancelable , view , ctrl_key , alt_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "KeyEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_KeyEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&KeyEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyEvent {
    #[cfg(all(feature = "KeyEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)\n\n*This API requires the following crate features to be activated: `KeyEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
    ) {
        #[cfg(all(feature = "KeyEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_KeyEvent(
                self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_KeyEvent(
            self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(ctrl_key);
            drop(alt_key);
            drop(shift_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_KeyEvent ( self_ , type_ , can_bubble , cancelable , view , ctrl_key , alt_key , shift_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "KeyEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_KeyEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&KeyEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyEvent {
    #[cfg(all(feature = "KeyEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)\n\n*This API requires the following crate features to be activated: `KeyEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
    ) {
        #[cfg(all(feature = "KeyEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_KeyEvent(
                self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_KeyEvent(
            self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(ctrl_key);
            drop(alt_key);
            drop(shift_key);
            drop(meta_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                let meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key);
                __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_KeyEvent ( self_ , type_ , can_bubble , cancelable , view , ctrl_key , alt_key , shift_key , meta_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "KeyEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code_KeyEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&KeyEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyEvent {
    #[cfg(all(feature = "KeyEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)\n\n*This API requires the following crate features to be activated: `KeyEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        key_code: u32,
    ) {
        #[cfg(all(feature = "KeyEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code_KeyEvent(
                self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_code: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code_KeyEvent(
            self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_code: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(ctrl_key);
            drop(alt_key);
            drop(shift_key);
            drop(meta_key);
            drop(key_code);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                let meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key);
                let key_code = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_code);
                __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code_KeyEvent ( self_ , type_ , can_bubble , cancelable , view , ctrl_key , alt_key , shift_key , meta_key , key_code )
            };
            ()
        }
    }
}
#[cfg(all(feature = "KeyEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code_and_char_code_KeyEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(11u32);
    <&KeyEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyEvent {
    #[cfg(all(feature = "KeyEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)\n\n*This API requires the following crate features to be activated: `KeyEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code_and_char_code(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        key_code: u32,
        char_code: u32,
    ) {
        #[cfg(all(feature = "KeyEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code_and_char_code_KeyEvent(
                self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_code: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                char_code: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code_and_char_code_KeyEvent(
            self_: <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_code: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            char_code: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(ctrl_key);
            drop(alt_key);
            drop(shift_key);
            drop(meta_key);
            drop(key_code);
            drop(char_code);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                let meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key);
                let key_code = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_code);
                let char_code = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(char_code);
                __widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code_and_char_code_KeyEvent ( self_ , type_ , can_bubble , cancelable , view , ctrl_key , alt_key , shift_key , meta_key , key_code , char_code )
            };
            ()
        }
    }
}
impl KeyEvent {
    pub const DOM_VK_CANCEL: u32 = 3u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_HELP: u32 = 6u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_BACK_SPACE: u32 = 8u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_TAB: u32 = 9u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_CLEAR: u32 = 12u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_RETURN: u32 = 13u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_SHIFT: u32 = 16u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_CONTROL: u32 = 17u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_ALT: u32 = 18u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_PAUSE: u32 = 19u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_CAPS_LOCK: u32 = 20u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_KANA: u32 = 21u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_HANGUL: u32 = 21u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_EISU: u32 = 22u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_JUNJA: u32 = 23u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_FINAL: u32 = 24u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_HANJA: u32 = 25u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_KANJI: u32 = 25u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_ESCAPE: u32 = 27u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_CONVERT: u32 = 28u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_NONCONVERT: u32 = 29u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_ACCEPT: u32 = 30u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_MODECHANGE: u32 = 31u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_SPACE: u32 = 32u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_PAGE_UP: u32 = 33u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_PAGE_DOWN: u32 = 34u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_END: u32 = 35u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_HOME: u32 = 36u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_LEFT: u32 = 37u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_UP: u32 = 38u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_RIGHT: u32 = 39u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_DOWN: u32 = 40u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_SELECT: u32 = 41u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_PRINT: u32 = 42u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_EXECUTE: u32 = 43u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_PRINTSCREEN: u32 = 44u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_INSERT: u32 = 45u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_DELETE: u32 = 46u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_0: u32 = 48u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_1: u32 = 49u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_2: u32 = 50u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_3: u32 = 51u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_4: u32 = 52u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_5: u32 = 53u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_6: u32 = 54u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_7: u32 = 55u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_8: u32 = 56u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_9: u32 = 57u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_COLON: u32 = 58u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_SEMICOLON: u32 = 59u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_LESS_THAN: u32 = 60u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_EQUALS: u32 = 61u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_GREATER_THAN: u32 = 62u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_QUESTION_MARK: u32 = 63u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_AT: u32 = 64u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_A: u32 = 65u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_B: u32 = 66u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_C: u32 = 67u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_D: u32 = 68u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_E: u32 = 69u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F: u32 = 70u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_G: u32 = 71u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_H: u32 = 72u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_I: u32 = 73u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_J: u32 = 74u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_K: u32 = 75u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_L: u32 = 76u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_M: u32 = 77u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_N: u32 = 78u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_O: u32 = 79u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_P: u32 = 80u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_Q: u32 = 81u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_R: u32 = 82u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_S: u32 = 83u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_T: u32 = 84u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_U: u32 = 85u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_V: u32 = 86u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_W: u32 = 87u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_X: u32 = 88u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_Y: u32 = 89u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_Z: u32 = 90u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN: u32 = 91u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_CONTEXT_MENU: u32 = 93u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_SLEEP: u32 = 95u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_NUMPAD0: u32 = 96u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_NUMPAD1: u32 = 97u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_NUMPAD2: u32 = 98u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_NUMPAD3: u32 = 99u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_NUMPAD4: u32 = 100u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_NUMPAD5: u32 = 101u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_NUMPAD6: u32 = 102u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_NUMPAD7: u32 = 103u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_NUMPAD8: u32 = 104u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_NUMPAD9: u32 = 105u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_MULTIPLY: u32 = 106u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_ADD: u32 = 107u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_SEPARATOR: u32 = 108u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_SUBTRACT: u32 = 109u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_DECIMAL: u32 = 110u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_DIVIDE: u32 = 111u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F1: u32 = 112u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F2: u32 = 113u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F3: u32 = 114u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F4: u32 = 115u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F5: u32 = 116u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F6: u32 = 117u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F7: u32 = 118u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F8: u32 = 119u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F9: u32 = 120u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F10: u32 = 121u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F11: u32 = 122u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F12: u32 = 123u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F13: u32 = 124u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F14: u32 = 125u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F15: u32 = 126u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F16: u32 = 127u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F17: u32 = 128u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F18: u32 = 129u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F19: u32 = 130u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F20: u32 = 131u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F21: u32 = 132u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F22: u32 = 133u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F23: u32 = 134u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_F24: u32 = 135u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_NUM_LOCK: u32 = 144u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_SCROLL_LOCK: u32 = 145u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_FJ_JISHO: u32 = 146u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_FJ_MASSHOU: u32 = 147u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_FJ_TOUROKU: u32 = 148u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_FJ_LOYA: u32 = 149u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_FJ_ROYA: u32 = 150u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_CIRCUMFLEX: u32 = 160u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_EXCLAMATION: u32 = 161u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_DOUBLE_QUOTE: u32 = 162u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_HASH: u32 = 163u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_DOLLAR: u32 = 164u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_PERCENT: u32 = 165u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_AMPERSAND: u32 = 166u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_UNDERSCORE: u32 = 167u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_OPEN_PAREN: u32 = 168u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_CLOSE_PAREN: u32 = 169u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_ASTERISK: u32 = 170u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_PLUS: u32 = 171u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_PIPE: u32 = 172u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_HYPHEN_MINUS: u32 = 173u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_OPEN_CURLY_BRACKET: u32 = 174u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_CLOSE_CURLY_BRACKET: u32 = 175u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_TILDE: u32 = 176u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_VOLUME_MUTE: u32 = 181u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_VOLUME_DOWN: u32 = 182u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_VOLUME_UP: u32 = 183u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_COMMA: u32 = 188u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_PERIOD: u32 = 190u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_SLASH: u32 = 191u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_BACK_QUOTE: u32 = 192u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_OPEN_BRACKET: u32 = 219u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_BACK_SLASH: u32 = 220u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_CLOSE_BRACKET: u32 = 221u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_QUOTE: u32 = 222u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_META: u32 = 224u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_ALTGR: u32 = 225u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_ICO_HELP: u32 = 227u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_ICO_00: u32 = 228u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_PROCESSKEY: u32 = 229u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_ICO_CLEAR: u32 = 230u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_RESET: u32 = 233u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_JUMP: u32 = 234u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_PA1: u32 = 235u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_PA2: u32 = 236u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_PA3: u32 = 237u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_WSCTRL: u32 = 238u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_CUSEL: u32 = 239u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_ATTN: u32 = 240u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_FINISH: u32 = 241u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_COPY: u32 = 242u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_AUTO: u32 = 243u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_ENLW: u32 = 244u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_BACKTAB: u32 = 245u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_ATTN: u32 = 246u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_CRSEL: u32 = 247u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_EXSEL: u32 = 248u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_EREOF: u32 = 249u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_PLAY: u32 = 250u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_ZOOM: u32 = 251u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_PA1: u32 = 253u64 as u32;
}
impl KeyEvent {
    pub const DOM_VK_WIN_OEM_CLEAR: u32 = 254u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_3db02160ad826638: [u8; 1948usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}Z\x07\0\0\0\0\x0B\0\0\x02\x08KeyEvent\x1A__widl_instanceof_KeyEvent\0\0\0\0 __widl_f_init_key_event_KeyEvent\0\0\0\x01\x08KeyEvent\x01\0\0\x01\x02\x05self_\x05type_\x0CinitKeyEvent\0\0\00__widl_f_init_key_event_with_can_bubble_KeyEvent\0\0\0\x01\x08KeyEvent\x01\0\0\x01\x03\x05self_\x05type_\ncan_bubble\x0CinitKeyEvent\0\0\0?__widl_f_init_key_event_with_can_bubble_and_cancelable_KeyEvent\0\0\0\x01\x08KeyEvent\x01\0\0\x01\x04\x05self_\x05type_\ncan_bubble\ncancelable\x0CinitKeyEvent\0\0\0H__widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_KeyEvent\0\0\0\x01\x08KeyEvent\x01\0\0\x01\x05\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x0CinitKeyEvent\0\0\0U__widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_KeyEvent\0\0\0\x01\x08KeyEvent\x01\0\0\x01\x06\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x08ctrl_key\x0CinitKeyEvent\0\0\0a__widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_KeyEvent\0\0\0\x01\x08KeyEvent\x01\0\0\x01\x07\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x08ctrl_key\x07alt_key\x0CinitKeyEvent\0\0\0o__widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_KeyEvent\0\0\0\x01\x08KeyEvent\x01\0\0\x01\x08\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x08ctrl_key\x07alt_key\tshift_key\x0CinitKeyEvent\0\0\0|__widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_KeyEvent\0\0\0\x01\x08KeyEvent\x01\0\0\x01\t\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x08ctrl_key\x07alt_key\tshift_key\x08meta_key\x0CinitKeyEvent\0\0\0\x89\x01__widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code_KeyEvent\0\0\0\x01\x08KeyEvent\x01\0\0\x01\n\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x08ctrl_key\x07alt_key\tshift_key\x08meta_key\x08key_code\x0CinitKeyEvent\0\0\0\x97\x01__widl_f_init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code_and_char_code_KeyEvent\0\0\0\x01\x08KeyEvent\x01\0\0\x01\x0B\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x08ctrl_key\x07alt_key\tshift_key\x08meta_key\x08key_code\tchar_code\x0CinitKeyEvent\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
