use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MutationEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent)\n\n*This API requires the following crate features to be activated: `MutationEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MutationEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MutationEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MutationEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(77u32);
            inform(117u32);
            inform(116u32);
            inform(97u32);
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
    impl core::ops::Deref for MutationEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for MutationEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MutationEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MutationEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MutationEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MutationEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MutationEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MutationEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MutationEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MutationEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MutationEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MutationEvent {
        #[inline]
        fn from(obj: JsValue) -> MutationEvent {
            MutationEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MutationEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MutationEvent> for MutationEvent {
        #[inline]
        fn as_ref(&self) -> &MutationEvent {
            self
        }
    }
    impl From<MutationEvent> for JsValue {
        #[inline]
        fn from(obj: MutationEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MutationEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MutationEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MutationEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MutationEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MutationEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MutationEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MutationEvent> for Event {
    #[inline]
    fn from(obj: MutationEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for MutationEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MutationEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: MutationEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MutationEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MutationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mutation_event_MutationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MutationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MutationEvent {
    #[cfg(all(feature = "MutationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMutationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)\n\n*This API requires the following crate features to be activated: `MutationEvent`*"]
    #[allow(clippy::all)]
    pub fn init_mutation_event(&self, type_: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MutationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mutation_event_MutationEvent(
                self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mutation_event_MutationEvent(
            self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_init_mutation_event_MutationEvent(self_, type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MutationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mutation_event_with_can_bubble_MutationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&MutationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MutationEvent {
    #[cfg(all(feature = "MutationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMutationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)\n\n*This API requires the following crate features to be activated: `MutationEvent`*"]
    #[allow(clippy::all)]
    pub fn init_mutation_event_with_can_bubble(
        &self,
        type_: &str,
        can_bubble: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MutationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mutation_event_with_can_bubble_MutationEvent(
                self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mutation_event_with_can_bubble_MutationEvent(
            self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                __widl_f_init_mutation_event_with_can_bubble_MutationEvent(self_, type_, can_bubble)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MutationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mutation_event_with_can_bubble_and_cancelable_MutationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&MutationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MutationEvent {
    #[cfg(all(feature = "MutationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMutationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)\n\n*This API requires the following crate features to be activated: `MutationEvent`*"]
    #[allow(clippy::all)]
    pub fn init_mutation_event_with_can_bubble_and_cancelable(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MutationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mutation_event_with_can_bubble_and_cancelable_MutationEvent(
                self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mutation_event_with_can_bubble_and_cancelable_MutationEvent(
            self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                __widl_f_init_mutation_event_with_can_bubble_and_cancelable_MutationEvent(
                    self_, type_, can_bubble, cancelable,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MutationEvent", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_MutationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&MutationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Node> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MutationEvent {
    #[cfg(all(feature = "MutationEvent", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `initMutationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)\n\n*This API requires the following crate features to be activated: `MutationEvent`, `Node`*"]
    #[allow(clippy::all)]
    pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        related_node: Option<&Node>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MutationEvent", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_MutationEvent(
                self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                related_node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_MutationEvent(
            self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            related_node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(related_node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let related_node =
                    <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(related_node);
                __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_MutationEvent ( self_ , type_ , can_bubble , cancelable , related_node )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MutationEvent", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_MutationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&MutationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Node> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MutationEvent {
    #[cfg(all(feature = "MutationEvent", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `initMutationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)\n\n*This API requires the following crate features to be activated: `MutationEvent`, `Node`*"]
    #[allow(clippy::all)]
    pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        related_node: Option<&Node>,
        prev_value: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MutationEvent", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_MutationEvent(
                self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                related_node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                prev_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_MutationEvent(
            self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            related_node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            prev_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(related_node);
            drop(prev_value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let related_node =
                    <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(related_node);
                let prev_value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(prev_value);
                __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_MutationEvent ( self_ , type_ , can_bubble , cancelable , related_node , prev_value )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MutationEvent", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_MutationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&MutationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Node> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MutationEvent {
    #[cfg(all(feature = "MutationEvent", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `initMutationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)\n\n*This API requires the following crate features to be activated: `MutationEvent`, `Node`*"]
    #[allow(clippy::all)]
    pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        related_node: Option<&Node>,
        prev_value: &str,
        new_value: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MutationEvent", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_MutationEvent(
                self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                related_node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                prev_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_MutationEvent(
            self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            related_node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            prev_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(related_node);
            drop(prev_value);
            drop(new_value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let related_node =
                    <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(related_node);
                let prev_value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(prev_value);
                let new_value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_value);
                __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_MutationEvent ( self_ , type_ , can_bubble , cancelable , related_node , prev_value , new_value )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MutationEvent", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name_MutationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&MutationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Node> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MutationEvent {
    #[cfg(all(feature = "MutationEvent", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `initMutationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)\n\n*This API requires the following crate features to be activated: `MutationEvent`, `Node`*"]
    #[allow(clippy::all)]
    pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        related_node: Option<&Node>,
        prev_value: &str,
        new_value: &str,
        attr_name: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MutationEvent", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name_MutationEvent(
                self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                related_node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                prev_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                attr_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name_MutationEvent(
            self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            related_node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            prev_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            attr_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(related_node);
            drop(prev_value);
            drop(new_value);
            drop(attr_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let related_node =
                    <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(related_node);
                let prev_value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(prev_value);
                let new_value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_value);
                let attr_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(attr_name);
                __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name_MutationEvent ( self_ , type_ , can_bubble , cancelable , related_node , prev_value , new_value , attr_name )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MutationEvent", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name_and_attr_change_MutationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&MutationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Node> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MutationEvent {
    #[cfg(all(feature = "MutationEvent", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `initMutationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)\n\n*This API requires the following crate features to be activated: `MutationEvent`, `Node`*"]
    #[allow(clippy::all)]
    pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name_and_attr_change(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        related_node: Option<&Node>,
        prev_value: &str,
        new_value: &str,
        attr_name: &str,
        attr_change: u16,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MutationEvent", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name_and_attr_change_MutationEvent(
                self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                related_node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                prev_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                attr_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                attr_change: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name_and_attr_change_MutationEvent(
            self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            related_node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            prev_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            attr_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            attr_change: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(related_node);
            drop(prev_value);
            drop(new_value);
            drop(attr_name);
            drop(attr_change);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let related_node =
                    <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(related_node);
                let prev_value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(prev_value);
                let new_value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_value);
                let attr_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(attr_name);
                let attr_change =
                    <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(attr_change);
                __widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name_and_attr_change_MutationEvent ( self_ , type_ , can_bubble , cancelable , related_node , prev_value , new_value , attr_name , attr_change )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MutationEvent", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_related_node_MutationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationEvent as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl MutationEvent {
    #[cfg(all(feature = "MutationEvent", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `relatedNode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/relatedNode)\n\n*This API requires the following crate features to be activated: `MutationEvent`, `Node`*"]
    #[allow(clippy::all)]
    pub fn related_node(&self) -> Option<Node> {
        #[cfg(all(feature = "MutationEvent", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_related_node_MutationEvent(
                self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_related_node_MutationEvent(
            self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_related_node_MutationEvent(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MutationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prev_value_MutationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MutationEvent {
    #[cfg(all(feature = "MutationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `prevValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/prevValue)\n\n*This API requires the following crate features to be activated: `MutationEvent`*"]
    #[allow(clippy::all)]
    pub fn prev_value(&self) -> String {
        #[cfg(all(feature = "MutationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prev_value_MutationEvent(
                self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prev_value_MutationEvent(
            self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_prev_value_MutationEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MutationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_value_MutationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MutationEvent {
    #[cfg(all(feature = "MutationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `newValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/newValue)\n\n*This API requires the following crate features to be activated: `MutationEvent`*"]
    #[allow(clippy::all)]
    pub fn new_value(&self) -> String {
        #[cfg(all(feature = "MutationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_value_MutationEvent(
                self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_value_MutationEvent(
            self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_new_value_MutationEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MutationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_attr_name_MutationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MutationEvent {
    #[cfg(all(feature = "MutationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `attrName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/attrName)\n\n*This API requires the following crate features to be activated: `MutationEvent`*"]
    #[allow(clippy::all)]
    pub fn attr_name(&self) -> String {
        #[cfg(all(feature = "MutationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_attr_name_MutationEvent(
                self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_attr_name_MutationEvent(
            self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_attr_name_MutationEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MutationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_attr_change_MutationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationEvent as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl MutationEvent {
    #[cfg(all(feature = "MutationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `attrChange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/attrChange)\n\n*This API requires the following crate features to be activated: `MutationEvent`*"]
    #[allow(clippy::all)]
    pub fn attr_change(&self) -> u16 {
        #[cfg(all(feature = "MutationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_attr_change_MutationEvent(
                self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_attr_change_MutationEvent(
            self_: <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_attr_change_MutationEvent(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl MutationEvent {
    pub const MODIFICATION: u16 = 1u64 as u16;
}
impl MutationEvent {
    pub const ADDITION: u16 = 2u64 as u16;
}
impl MutationEvent {
    pub const REMOVAL: u16 = 3u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_79819fed4f91afaa: [u8; 2129usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x0F\x08\0\0\0\0\x0E\0\0\x02\rMutationEvent\x1F__widl_instanceof_MutationEvent\0\0\0\0*__widl_f_init_mutation_event_MutationEvent\x01\0\0\x01\rMutationEvent\x01\0\0\x01\x02\x05self_\x05type_\x11initMutationEvent\0\0\0:__widl_f_init_mutation_event_with_can_bubble_MutationEvent\x01\0\0\x01\rMutationEvent\x01\0\0\x01\x03\x05self_\x05type_\ncan_bubble\x11initMutationEvent\0\0\0I__widl_f_init_mutation_event_with_can_bubble_and_cancelable_MutationEvent\x01\0\0\x01\rMutationEvent\x01\0\0\x01\x04\x05self_\x05type_\ncan_bubble\ncancelable\x11initMutationEvent\0\0\0Z__widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_MutationEvent\x01\0\0\x01\rMutationEvent\x01\0\0\x01\x05\x05self_\x05type_\ncan_bubble\ncancelable\x0Crelated_node\x11initMutationEvent\0\0\0i__widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_MutationEvent\x01\0\0\x01\rMutationEvent\x01\0\0\x01\x06\x05self_\x05type_\ncan_bubble\ncancelable\x0Crelated_node\nprev_value\x11initMutationEvent\0\0\0w__widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_MutationEvent\x01\0\0\x01\rMutationEvent\x01\0\0\x01\x07\x05self_\x05type_\ncan_bubble\ncancelable\x0Crelated_node\nprev_value\tnew_value\x11initMutationEvent\0\0\0\x85\x01__widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name_MutationEvent\x01\0\0\x01\rMutationEvent\x01\0\0\x01\x08\x05self_\x05type_\ncan_bubble\ncancelable\x0Crelated_node\nprev_value\tnew_value\tattr_name\x11initMutationEvent\0\0\0\x95\x01__widl_f_init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name_and_attr_change_MutationEvent\x01\0\0\x01\rMutationEvent\x01\0\0\x01\t\x05self_\x05type_\ncan_bubble\ncancelable\x0Crelated_node\nprev_value\tnew_value\tattr_name\x0Battr_change\x11initMutationEvent\0\0\0#__widl_f_related_node_MutationEvent\0\0\0\x01\rMutationEvent\x01\0\x01\x0BrelatedNode\x01\x01\x05self_\x0BrelatedNode\0\0\0!__widl_f_prev_value_MutationEvent\0\0\0\x01\rMutationEvent\x01\0\x01\tprevValue\x01\x01\x05self_\tprevValue\0\0\0 __widl_f_new_value_MutationEvent\0\0\0\x01\rMutationEvent\x01\0\x01\x08newValue\x01\x01\x05self_\x08newValue\0\0\0 __widl_f_attr_name_MutationEvent\0\0\0\x01\rMutationEvent\x01\0\x01\x08attrName\x01\x01\x05self_\x08attrName\0\0\0\"__widl_f_attr_change_MutationEvent\0\0\0\x01\rMutationEvent\x01\0\x01\nattrChange\x01\x01\x05self_\nattrChange\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
