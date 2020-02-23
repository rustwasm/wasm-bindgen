use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `StorageEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct StorageEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_StorageEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for StorageEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(83u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
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
    impl core::ops::Deref for StorageEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for StorageEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for StorageEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a StorageEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for StorageEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            StorageEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for StorageEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a StorageEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for StorageEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<StorageEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(StorageEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for StorageEvent {
        #[inline]
        fn from(obj: JsValue) -> StorageEvent {
            StorageEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for StorageEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<StorageEvent> for StorageEvent {
        #[inline]
        fn as_ref(&self) -> &StorageEvent {
            self
        }
    }
    impl From<StorageEvent> for JsValue {
        #[inline]
        fn from(obj: StorageEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for StorageEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_StorageEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_StorageEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_StorageEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            StorageEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const StorageEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<StorageEvent> for Event {
    #[inline]
    fn from(obj: StorageEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for StorageEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<StorageEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: StorageEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for StorageEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_StorageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <StorageEvent as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new StorageEvent(..)` constructor, creating a new instance of `StorageEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/StorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<StorageEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_StorageEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <StorageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_StorageEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <StorageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_StorageEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<StorageEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "StorageEvent", feature = "StorageEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_StorageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&StorageEventInit as WasmDescribe>::describe();
    <StorageEvent as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "StorageEvent", feature = "StorageEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new StorageEvent(..)` constructor, creating a new instance of `StorageEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/StorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`, `StorageEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &StorageEventInit,
    ) -> Result<StorageEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "StorageEvent", feature = "StorageEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_StorageEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&StorageEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <StorageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_StorageEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&StorageEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <StorageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&StorageEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_StorageEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<StorageEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_storage_event_StorageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&StorageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initStorageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_storage_event(&self, type_: &str) {
        #[cfg(all(feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_storage_event_StorageEvent(
                self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_storage_event_StorageEvent(
            self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_init_storage_event_StorageEvent(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_storage_event_with_can_bubble_StorageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&StorageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initStorageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_storage_event_with_can_bubble(&self, type_: &str, can_bubble: bool) {
        #[cfg(all(feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_storage_event_with_can_bubble_StorageEvent(
                self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_storage_event_with_can_bubble_StorageEvent(
            self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                __widl_f_init_storage_event_with_can_bubble_StorageEvent(self_, type_, can_bubble)
            };
            ()
        }
    }
}
#[cfg(all(feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_storage_event_with_can_bubble_and_cancelable_StorageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&StorageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initStorageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_storage_event_with_can_bubble_and_cancelable(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    ) {
        #[cfg(all(feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_storage_event_with_can_bubble_and_cancelable_StorageEvent(
                self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_storage_event_with_can_bubble_and_cancelable_StorageEvent(
            self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                __widl_f_init_storage_event_with_can_bubble_and_cancelable_StorageEvent(
                    self_, type_, can_bubble, cancelable,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_StorageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&StorageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initStorageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
    ) {
        #[cfg(all(feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_StorageEvent(
                self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_StorageEvent(
            self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let key = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_StorageEvent(
                    self_, type_, can_bubble, cancelable, key,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_StorageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&StorageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initStorageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
        old_value: Option<&str>,
    ) {
        #[cfg(all(feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_StorageEvent(
                self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                old_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_StorageEvent(
            self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            old_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(key);
            drop(old_value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let key = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let old_value =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(old_value);
                __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_StorageEvent ( self_ , type_ , can_bubble , cancelable , key , old_value )
            };
            ()
        }
    }
}
#[cfg(all(feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_StorageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&StorageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initStorageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
        old_value: Option<&str>,
        new_value: Option<&str>,
    ) {
        #[cfg(all(feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_StorageEvent(
                self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                old_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_StorageEvent(
            self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            old_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(key);
            drop(old_value);
            drop(new_value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let key = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let old_value =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(old_value);
                let new_value =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_value);
                __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_StorageEvent ( self_ , type_ , can_bubble , cancelable , key , old_value , new_value )
            };
            ()
        }
    }
}
#[cfg(all(feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_StorageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&StorageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initStorageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
        old_value: Option<&str>,
        new_value: Option<&str>,
        url: Option<&str>,
    ) {
        #[cfg(all(feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_StorageEvent(
                self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                old_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_StorageEvent(
            self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            old_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(key);
            drop(old_value);
            drop(new_value);
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let key = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let old_value =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(old_value);
                let new_value =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_value);
                let url = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_StorageEvent ( self_ , type_ , can_bubble , cancelable , key , old_value , new_value , url )
            };
            ()
        }
    }
}
#[cfg(all(feature = "Storage", feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_and_storage_area_StorageEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&StorageEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<&Storage> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "Storage", feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initStorageEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `Storage`, `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_and_storage_area(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
        old_value: Option<&str>,
        new_value: Option<&str>,
        url: Option<&str>,
        storage_area: Option<&Storage>,
    ) {
        #[cfg(all(feature = "Storage", feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_and_storage_area_StorageEvent(
                self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                old_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                storage_area: <Option<&Storage> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_and_storage_area_StorageEvent(
            self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            old_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            storage_area: <Option<&Storage> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(key);
            drop(old_value);
            drop(new_value);
            drop(url);
            drop(storage_area);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let key = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let old_value =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(old_value);
                let new_value =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_value);
                let url = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let storage_area =
                    <Option<&Storage> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        storage_area,
                    );
                __widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_and_storage_area_StorageEvent ( self_ , type_ , can_bubble , cancelable , key , old_value , new_value , url , storage_area )
            };
            ()
        }
    }
}
#[cfg(all(feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_key_StorageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StorageEvent as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `key` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/key)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn key(&self) -> Option<String> {
        #[cfg(all(feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_key_StorageEvent(
                self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_key_StorageEvent(
            self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_key_StorageEvent(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_old_value_StorageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StorageEvent as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `oldValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/oldValue)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn old_value(&self) -> Option<String> {
        #[cfg(all(feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_old_value_StorageEvent(
                self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_old_value_StorageEvent(
            self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_old_value_StorageEvent(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_value_StorageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StorageEvent as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `newValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/newValue)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn new_value(&self) -> Option<String> {
        #[cfg(all(feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_value_StorageEvent(
                self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_value_StorageEvent(
            self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_new_value_StorageEvent(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_url_StorageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StorageEvent as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `url` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/url)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn url(&self) -> Option<String> {
        #[cfg(all(feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_url_StorageEvent(
                self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_url_StorageEvent(
            self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_url_StorageEvent(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Storage", feature = "StorageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_storage_area_StorageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StorageEvent as WasmDescribe>::describe();
    <Option<Storage> as WasmDescribe>::describe();
}
impl StorageEvent {
    #[cfg(all(feature = "Storage", feature = "StorageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `storageArea` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/storageArea)\n\n*This API requires the following crate features to be activated: `Storage`, `StorageEvent`*"]
    #[allow(clippy::all)]
    pub fn storage_area(&self) -> Option<Storage> {
        #[cfg(all(feature = "Storage", feature = "StorageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_storage_area_StorageEvent(
                self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Storage> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_storage_area_StorageEvent(
            self_: <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Storage> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StorageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_storage_area_StorageEvent(self_)
            };
            <Option<Storage> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_05f1cf075ccd0119: [u8; 2078usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xDC\x07\0\0\0\0\x10\0\0\x02\x0CStorageEvent\x1E__widl_instanceof_StorageEvent\0\0\0\0\x19__widl_f_new_StorageEvent\x01\0\0\x01\x0CStorageEvent\0\x01\x01\x05type_\x03new\0\0\0.__widl_f_new_with_event_init_dict_StorageEvent\x01\0\0\x01\x0CStorageEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0(__widl_f_init_storage_event_StorageEvent\0\0\0\x01\x0CStorageEvent\x01\0\0\x01\x02\x05self_\x05type_\x10initStorageEvent\0\0\08__widl_f_init_storage_event_with_can_bubble_StorageEvent\0\0\0\x01\x0CStorageEvent\x01\0\0\x01\x03\x05self_\x05type_\ncan_bubble\x10initStorageEvent\0\0\0G__widl_f_init_storage_event_with_can_bubble_and_cancelable_StorageEvent\0\0\0\x01\x0CStorageEvent\x01\0\0\x01\x04\x05self_\x05type_\ncan_bubble\ncancelable\x10initStorageEvent\0\0\0O__widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_StorageEvent\0\0\0\x01\x0CStorageEvent\x01\0\0\x01\x05\x05self_\x05type_\ncan_bubble\ncancelable\x03key\x10initStorageEvent\0\0\0]__widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_StorageEvent\0\0\0\x01\x0CStorageEvent\x01\0\0\x01\x06\x05self_\x05type_\ncan_bubble\ncancelable\x03key\told_value\x10initStorageEvent\0\0\0k__widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_StorageEvent\0\0\0\x01\x0CStorageEvent\x01\0\0\x01\x07\x05self_\x05type_\ncan_bubble\ncancelable\x03key\told_value\tnew_value\x10initStorageEvent\0\0\0s__widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_StorageEvent\0\0\0\x01\x0CStorageEvent\x01\0\0\x01\x08\x05self_\x05type_\ncan_bubble\ncancelable\x03key\told_value\tnew_value\x03url\x10initStorageEvent\0\0\0\x84\x01__widl_f_init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_and_storage_area_StorageEvent\0\0\0\x01\x0CStorageEvent\x01\0\0\x01\t\x05self_\x05type_\ncan_bubble\ncancelable\x03key\told_value\tnew_value\x03url\x0Cstorage_area\x10initStorageEvent\0\0\0\x19__widl_f_key_StorageEvent\0\0\0\x01\x0CStorageEvent\x01\0\x01\x03key\x01\x01\x05self_\x03key\0\0\0\x1F__widl_f_old_value_StorageEvent\0\0\0\x01\x0CStorageEvent\x01\0\x01\x08oldValue\x01\x01\x05self_\x08oldValue\0\0\0\x1F__widl_f_new_value_StorageEvent\0\0\0\x01\x0CStorageEvent\x01\0\x01\x08newValue\x01\x01\x05self_\x08newValue\0\0\0\x19__widl_f_url_StorageEvent\0\0\0\x01\x0CStorageEvent\x01\0\x01\x03url\x01\x01\x05self_\x03url\0\0\0\"__widl_f_storage_area_StorageEvent\0\0\0\x01\x0CStorageEvent\x01\0\x01\x0BstorageArea\x01\x01\x05self_\x0BstorageArea\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
