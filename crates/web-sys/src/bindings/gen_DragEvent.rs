use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DragEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DragEvent {
    obj: MouseEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DragEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DragEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(68u32);
            inform(114u32);
            inform(97u32);
            inform(103u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for DragEvent {
        type Target = MouseEvent;
        #[inline]
        fn deref(&self) -> &MouseEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for DragEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DragEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DragEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DragEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DragEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DragEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DragEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DragEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DragEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DragEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DragEvent {
        #[inline]
        fn from(obj: JsValue) -> DragEvent {
            DragEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DragEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DragEvent> for DragEvent {
        #[inline]
        fn as_ref(&self) -> &DragEvent {
            self
        }
    }
    impl From<DragEvent> for JsValue {
        #[inline]
        fn from(obj: DragEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DragEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DragEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DragEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DragEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DragEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DragEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DragEvent> for MouseEvent {
    #[inline]
    fn from(obj: DragEvent) -> MouseEvent {
        use wasm_bindgen::JsCast;
        MouseEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<MouseEvent> for DragEvent {
    #[inline]
    fn as_ref(&self) -> &MouseEvent {
        use wasm_bindgen::JsCast;
        MouseEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DragEvent> for UiEvent {
    #[inline]
    fn from(obj: DragEvent) -> UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<UiEvent> for DragEvent {
    #[inline]
    fn as_ref(&self) -> &UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DragEvent> for Event {
    #[inline]
    fn from(obj: DragEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for DragEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DragEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: DragEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DragEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DragEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DragEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <DragEvent as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new DragEvent(..)` constructor, creating a new instance of `DragEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/DragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<DragEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DragEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DragEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DragEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DragEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DragEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_DragEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DragEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DragEvent", feature = "DragEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_DragEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&DragEventInit as WasmDescribe>::describe();
    <DragEvent as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent", feature = "DragEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new DragEvent(..)` constructor, creating a new instance of `DragEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/DragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`, `DragEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &DragEventInit,
    ) -> Result<DragEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DragEvent", feature = "DragEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_DragEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&DragEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DragEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_DragEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&DragEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DragEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&DragEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_DragEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DragEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DragEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_DragEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event(&self, type_: &str) {
        #[cfg(all(feature = "DragEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_init_drag_event_DragEvent(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_DragEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble(&self, type_: &str, can_bubble: bool) {
        #[cfg(all(feature = "DragEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                __widl_f_init_drag_event_with_can_bubble_DragEvent(self_, type_, can_bubble)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    ) {
        #[cfg(all(feature = "DragEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_DragEvent(
                    self_, type_, can_bubble, cancelable,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
    ) {
        #[cfg(all(feature = "DragEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(a_view);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_DragEvent(
                    self_, type_, can_bubble, cancelable, a_view,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
    ) {
        #[cfg(all(feature = "DragEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(a_view);
            drop(a_detail);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_DragEvent ( self_ , type_ , can_bubble , cancelable , a_view , a_detail )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
    ) {
        #[cfg(all(feature = "DragEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(a_view);
            drop(a_detail);
            drop(a_screen_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                let a_screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_x);
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_DragEvent ( self_ , type_ , can_bubble , cancelable , a_view , a_detail , a_screen_x )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
    ) {
        #[cfg(all(feature = "DragEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(a_view);
            drop(a_detail);
            drop(a_screen_x);
            drop(a_screen_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                let a_screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_x);
                let a_screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_y);
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_DragEvent ( self_ , type_ , can_bubble , cancelable , a_view , a_detail , a_screen_x , a_screen_y )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
    ) {
        #[cfg(all(feature = "DragEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(a_view);
            drop(a_detail);
            drop(a_screen_x);
            drop(a_screen_y);
            drop(a_client_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                let a_screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_x);
                let a_screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_y);
                let a_client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_x);
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_DragEvent ( self_ , type_ , can_bubble , cancelable , a_view , a_detail , a_screen_x , a_screen_y , a_client_x )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
    ) {
        #[cfg(all(feature = "DragEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(a_view);
            drop(a_detail);
            drop(a_screen_x);
            drop(a_screen_y);
            drop(a_client_x);
            drop(a_client_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                let a_screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_x);
                let a_screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_y);
                let a_client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_x);
                let a_client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_y);
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_DragEvent ( self_ , type_ , can_bubble , cancelable , a_view , a_detail , a_screen_x , a_screen_y , a_client_x , a_client_y )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(11u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
    ) {
        #[cfg(all(feature = "DragEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(a_view);
            drop(a_detail);
            drop(a_screen_x);
            drop(a_screen_y);
            drop(a_client_x);
            drop(a_client_y);
            drop(a_ctrl_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                let a_screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_x);
                let a_screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_y);
                let a_client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_x);
                let a_client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_y);
                let a_ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_ctrl_key);
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_DragEvent ( self_ , type_ , can_bubble , cancelable , a_view , a_detail , a_screen_x , a_screen_y , a_client_x , a_client_y , a_ctrl_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(12u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
        a_alt_key: bool,
    ) {
        #[cfg(all(feature = "DragEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(a_view);
            drop(a_detail);
            drop(a_screen_x);
            drop(a_screen_y);
            drop(a_client_x);
            drop(a_client_y);
            drop(a_ctrl_key);
            drop(a_alt_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                let a_screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_x);
                let a_screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_y);
                let a_client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_x);
                let a_client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_y);
                let a_ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_ctrl_key);
                let a_alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_alt_key);
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_DragEvent ( self_ , type_ , can_bubble , cancelable , a_view , a_detail , a_screen_x , a_screen_y , a_client_x , a_client_y , a_ctrl_key , a_alt_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(13u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
        a_alt_key: bool,
        a_shift_key: bool,
    ) {
        #[cfg(all(feature = "DragEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(a_view);
            drop(a_detail);
            drop(a_screen_x);
            drop(a_screen_y);
            drop(a_client_x);
            drop(a_client_y);
            drop(a_ctrl_key);
            drop(a_alt_key);
            drop(a_shift_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                let a_screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_x);
                let a_screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_y);
                let a_client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_x);
                let a_client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_y);
                let a_ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_ctrl_key);
                let a_alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_alt_key);
                let a_shift_key =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_shift_key);
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_DragEvent ( self_ , type_ , can_bubble , cancelable , a_view , a_detail , a_screen_x , a_screen_y , a_client_x , a_client_y , a_ctrl_key , a_alt_key , a_shift_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(14u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
        a_alt_key: bool,
        a_shift_key: bool,
        a_meta_key: bool,
    ) {
        #[cfg(all(feature = "DragEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(a_view);
            drop(a_detail);
            drop(a_screen_x);
            drop(a_screen_y);
            drop(a_client_x);
            drop(a_client_y);
            drop(a_ctrl_key);
            drop(a_alt_key);
            drop(a_shift_key);
            drop(a_meta_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                let a_screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_x);
                let a_screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_y);
                let a_client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_x);
                let a_client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_y);
                let a_ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_ctrl_key);
                let a_alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_alt_key);
                let a_shift_key =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_shift_key);
                let a_meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_meta_key);
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_DragEvent ( self_ , type_ , can_bubble , cancelable , a_view , a_detail , a_screen_x , a_screen_y , a_client_x , a_client_y , a_ctrl_key , a_alt_key , a_shift_key , a_meta_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(15u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
        a_alt_key: bool,
        a_shift_key: bool,
        a_meta_key: bool,
        a_button: u16,
    ) {
        #[cfg(all(feature = "DragEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_button: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_button: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(a_view);
            drop(a_detail);
            drop(a_screen_x);
            drop(a_screen_y);
            drop(a_client_x);
            drop(a_client_y);
            drop(a_ctrl_key);
            drop(a_alt_key);
            drop(a_shift_key);
            drop(a_meta_key);
            drop(a_button);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                let a_screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_x);
                let a_screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_y);
                let a_client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_x);
                let a_client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_y);
                let a_ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_ctrl_key);
                let a_alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_alt_key);
                let a_shift_key =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_shift_key);
                let a_meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_meta_key);
                let a_button = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_button);
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_DragEvent ( self_ , type_ , can_bubble , cancelable , a_view , a_detail , a_screen_x , a_screen_y , a_client_x , a_client_y , a_ctrl_key , a_alt_key , a_shift_key , a_meta_key , a_button )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DragEvent", feature = "EventTarget", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(16u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <Option<&EventTarget> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DragEvent", feature = "EventTarget", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DragEvent`, `EventTarget`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
        a_alt_key: bool,
        a_shift_key: bool,
        a_meta_key: bool,
        a_button: u16,
        a_related_target: Option<&EventTarget>,
    ) {
        #[cfg(all(feature = "DragEvent", feature = "EventTarget", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_button: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_related_target: <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_button: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_related_target: <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(a_view);
            drop(a_detail);
            drop(a_screen_x);
            drop(a_screen_y);
            drop(a_client_x);
            drop(a_client_y);
            drop(a_ctrl_key);
            drop(a_alt_key);
            drop(a_shift_key);
            drop(a_meta_key);
            drop(a_button);
            drop(a_related_target);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                let a_screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_x);
                let a_screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_y);
                let a_client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_x);
                let a_client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_y);
                let a_ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_ctrl_key);
                let a_alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_alt_key);
                let a_shift_key =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_shift_key);
                let a_meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_meta_key);
                let a_button = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_button);
                let a_related_target =
                    <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        a_related_target,
                    );
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target_DragEvent ( self_ , type_ , can_bubble , cancelable , a_view , a_detail , a_screen_x , a_screen_y , a_client_x , a_client_y , a_ctrl_key , a_alt_key , a_shift_key , a_meta_key , a_button , a_related_target )
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "DataTransfer",
    feature = "DragEvent",
    feature = "EventTarget",
    feature = "Window",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target_and_a_data_transfer_DragEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(17u32);
    <&DragEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <Option<&EventTarget> as WasmDescribe>::describe();
    <Option<&DataTransfer> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(
        feature = "DataTransfer",
        feature = "DragEvent",
        feature = "EventTarget",
        feature = "Window",
    ))]
    #[allow(bad_style)]
    #[doc = "The `initDragEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)\n\n*This API requires the following crate features to be activated: `DataTransfer`, `DragEvent`, `EventTarget`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target_and_a_data_transfer(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
        a_alt_key: bool,
        a_shift_key: bool,
        a_meta_key: bool,
        a_button: u16,
        a_related_target: Option<&EventTarget>,
        a_data_transfer: Option<&DataTransfer>,
    ) {
        #[cfg(all(
            feature = "DataTransfer",
            feature = "DragEvent",
            feature = "EventTarget",
            feature = "Window",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target_and_a_data_transfer_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_button: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_related_target: <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_data_transfer: <Option<&DataTransfer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target_and_a_data_transfer_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_button: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_related_target: <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_data_transfer: <Option<&DataTransfer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(a_view);
            drop(a_detail);
            drop(a_screen_x);
            drop(a_screen_y);
            drop(a_client_x);
            drop(a_client_y);
            drop(a_ctrl_key);
            drop(a_alt_key);
            drop(a_shift_key);
            drop(a_meta_key);
            drop(a_button);
            drop(a_related_target);
            drop(a_data_transfer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                let a_screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_x);
                let a_screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_screen_y);
                let a_client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_x);
                let a_client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_client_y);
                let a_ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_ctrl_key);
                let a_alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_alt_key);
                let a_shift_key =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_shift_key);
                let a_meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_meta_key);
                let a_button = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_button);
                let a_related_target =
                    <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        a_related_target,
                    );
                let a_data_transfer =
                    <Option<&DataTransfer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        a_data_transfer,
                    );
                __widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target_and_a_data_transfer_DragEvent ( self_ , type_ , can_bubble , cancelable , a_view , a_detail , a_screen_x , a_screen_y , a_client_x , a_client_y , a_ctrl_key , a_alt_key , a_shift_key , a_meta_key , a_button , a_related_target , a_data_transfer )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DataTransfer", feature = "DragEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_transfer_DragEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DragEvent as WasmDescribe>::describe();
    <Option<DataTransfer> as WasmDescribe>::describe();
}
impl DragEvent {
    #[cfg(all(feature = "DataTransfer", feature = "DragEvent",))]
    #[allow(bad_style)]
    #[doc = "The `dataTransfer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/dataTransfer)\n\n*This API requires the following crate features to be activated: `DataTransfer`, `DragEvent`*"]
    #[allow(clippy::all)]
    pub fn data_transfer(&self) -> Option<DataTransfer> {
        #[cfg(all(feature = "DataTransfer", feature = "DragEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_transfer_DragEvent(
                self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DataTransfer> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_transfer_DragEvent(
            self_: <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DataTransfer> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DragEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_transfer_DragEvent(self_)
            };
            <Option<DataTransfer> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_33bcbfb6cc24e062: [u8; 4717usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}+\x12\0\0\0\0\x14\0\0\x02\tDragEvent\x1B__widl_instanceof_DragEvent\0\0\0\0\x16__widl_f_new_DragEvent\x01\0\0\x01\tDragEvent\0\x01\x01\x05type_\x03new\0\0\0+__widl_f_new_with_event_init_dict_DragEvent\x01\0\0\x01\tDragEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\"__widl_f_init_drag_event_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\x02\x05self_\x05type_\rinitDragEvent\0\0\02__widl_f_init_drag_event_with_can_bubble_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\x03\x05self_\x05type_\ncan_bubble\rinitDragEvent\0\0\0A__widl_f_init_drag_event_with_can_bubble_and_cancelable_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\x04\x05self_\x05type_\ncan_bubble\ncancelable\rinitDragEvent\0\0\0L__widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\x05\x05self_\x05type_\ncan_bubble\ncancelable\x06a_view\rinitDragEvent\0\0\0Y__widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\x06\x05self_\x05type_\ncan_bubble\ncancelable\x06a_view\x08a_detail\rinitDragEvent\0\0\0h__widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\x07\x05self_\x05type_\ncan_bubble\ncancelable\x06a_view\x08a_detail\na_screen_x\rinitDragEvent\0\0\0w__widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\x08\x05self_\x05type_\ncan_bubble\ncancelable\x06a_view\x08a_detail\na_screen_x\na_screen_y\rinitDragEvent\0\0\0\x86\x01__widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\t\x05self_\x05type_\ncan_bubble\ncancelable\x06a_view\x08a_detail\na_screen_x\na_screen_y\na_client_x\rinitDragEvent\0\0\0\x95\x01__widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\n\x05self_\x05type_\ncan_bubble\ncancelable\x06a_view\x08a_detail\na_screen_x\na_screen_y\na_client_x\na_client_y\rinitDragEvent\0\0\0\xA4\x01__widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\x0B\x05self_\x05type_\ncan_bubble\ncancelable\x06a_view\x08a_detail\na_screen_x\na_screen_y\na_client_x\na_client_y\na_ctrl_key\rinitDragEvent\0\0\0\xB2\x01__widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\x0C\x05self_\x05type_\ncan_bubble\ncancelable\x06a_view\x08a_detail\na_screen_x\na_screen_y\na_client_x\na_client_y\na_ctrl_key\ta_alt_key\rinitDragEvent\0\0\0\xC2\x01__widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\r\x05self_\x05type_\ncan_bubble\ncancelable\x06a_view\x08a_detail\na_screen_x\na_screen_y\na_client_x\na_client_y\na_ctrl_key\ta_alt_key\x0Ba_shift_key\rinitDragEvent\0\0\0\xD1\x01__widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\x0E\x05self_\x05type_\ncan_bubble\ncancelable\x06a_view\x08a_detail\na_screen_x\na_screen_y\na_client_x\na_client_y\na_ctrl_key\ta_alt_key\x0Ba_shift_key\na_meta_key\rinitDragEvent\0\0\0\xDE\x01__widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\x0F\x05self_\x05type_\ncan_bubble\ncancelable\x06a_view\x08a_detail\na_screen_x\na_screen_y\na_client_x\na_client_y\na_ctrl_key\ta_alt_key\x0Ba_shift_key\na_meta_key\x08a_button\rinitDragEvent\0\0\0\xF3\x01__widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\x10\x05self_\x05type_\ncan_bubble\ncancelable\x06a_view\x08a_detail\na_screen_x\na_screen_y\na_client_x\na_client_y\na_ctrl_key\ta_alt_key\x0Ba_shift_key\na_meta_key\x08a_button\x10a_related_target\rinitDragEvent\0\0\0\x87\x02__widl_f_init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target_and_a_data_transfer_DragEvent\0\0\0\x01\tDragEvent\x01\0\0\x01\x11\x05self_\x05type_\ncan_bubble\ncancelable\x06a_view\x08a_detail\na_screen_x\na_screen_y\na_client_x\na_client_y\na_ctrl_key\ta_alt_key\x0Ba_shift_key\na_meta_key\x08a_button\x10a_related_target\x0Fa_data_transfer\rinitDragEvent\0\0\0 __widl_f_data_transfer_DragEvent\0\0\0\x01\tDragEvent\x01\0\x01\x0CdataTransfer\x01\x01\x05self_\x0CdataTransfer\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
