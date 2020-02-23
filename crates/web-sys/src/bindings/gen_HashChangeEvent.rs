use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HashChangeEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent)\n\n*This API requires the following crate features to be activated: `HashChangeEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HashChangeEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HashChangeEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HashChangeEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(72u32);
            inform(97u32);
            inform(115u32);
            inform(104u32);
            inform(67u32);
            inform(104u32);
            inform(97u32);
            inform(110u32);
            inform(103u32);
            inform(101u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HashChangeEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for HashChangeEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HashChangeEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HashChangeEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HashChangeEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HashChangeEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HashChangeEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HashChangeEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HashChangeEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HashChangeEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HashChangeEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HashChangeEvent {
        #[inline]
        fn from(obj: JsValue) -> HashChangeEvent {
            HashChangeEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HashChangeEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HashChangeEvent> for HashChangeEvent {
        #[inline]
        fn as_ref(&self) -> &HashChangeEvent {
            self
        }
    }
    impl From<HashChangeEvent> for JsValue {
        #[inline]
        fn from(obj: HashChangeEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HashChangeEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HashChangeEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HashChangeEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HashChangeEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HashChangeEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HashChangeEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HashChangeEvent> for Event {
    #[inline]
    fn from(obj: HashChangeEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for HashChangeEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HashChangeEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: HashChangeEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HashChangeEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HashChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_HashChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <HashChangeEvent as WasmDescribe>::describe();
}
impl HashChangeEvent {
    #[cfg(all(feature = "HashChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new HashChangeEvent(..)` constructor, creating a new instance of `HashChangeEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/HashChangeEvent)\n\n*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<HashChangeEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HashChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_HashChangeEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HashChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_HashChangeEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HashChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_HashChangeEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HashChangeEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HashChangeEvent", feature = "HashChangeEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_HashChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&HashChangeEventInit as WasmDescribe>::describe();
    <HashChangeEvent as WasmDescribe>::describe();
}
impl HashChangeEvent {
    #[cfg(all(feature = "HashChangeEvent", feature = "HashChangeEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new HashChangeEvent(..)` constructor, creating a new instance of `HashChangeEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/HashChangeEvent)\n\n*This API requires the following crate features to be activated: `HashChangeEvent`, `HashChangeEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &HashChangeEventInit,
    ) -> Result<HashChangeEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HashChangeEvent", feature = "HashChangeEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_HashChangeEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&HashChangeEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HashChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_HashChangeEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&HashChangeEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HashChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&HashChangeEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_HashChangeEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HashChangeEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HashChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_hash_change_event_HashChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HashChangeEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HashChangeEvent {
    #[cfg(all(feature = "HashChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initHashChangeEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)\n\n*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn init_hash_change_event(&self, type_arg: &str) {
        #[cfg(all(feature = "HashChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_hash_change_event_HashChangeEvent(
                self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_hash_change_event_HashChangeEvent(
            self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                __widl_f_init_hash_change_event_HashChangeEvent(self_, type_arg)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HashChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_hash_change_event_with_can_bubble_arg_HashChangeEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HashChangeEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HashChangeEvent {
    #[cfg(all(feature = "HashChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initHashChangeEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)\n\n*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn init_hash_change_event_with_can_bubble_arg(&self, type_arg: &str, can_bubble_arg: bool) {
        #[cfg(all(feature = "HashChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_hash_change_event_with_can_bubble_arg_HashChangeEvent(
                self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_hash_change_event_with_can_bubble_arg_HashChangeEvent(
            self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                __widl_f_init_hash_change_event_with_can_bubble_arg_HashChangeEvent(
                    self_,
                    type_arg,
                    can_bubble_arg,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "HashChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_HashChangeEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&HashChangeEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HashChangeEvent {
    #[cfg(all(feature = "HashChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initHashChangeEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)\n\n*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn init_hash_change_event_with_can_bubble_arg_and_cancelable_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
    ) {
        #[cfg(all(feature = "HashChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_HashChangeEvent(
                self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_HashChangeEvent(
            self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                __widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_HashChangeEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "HashChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg_HashChangeEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&HashChangeEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HashChangeEvent {
    #[cfg(all(feature = "HashChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initHashChangeEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)\n\n*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        old_url_arg: &str,
    ) {
        #[cfg(all(feature = "HashChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg_HashChangeEvent(
                self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                old_url_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg_HashChangeEvent(
            self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            old_url_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(old_url_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let old_url_arg =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(old_url_arg);
                __widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg_HashChangeEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , old_url_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "HashChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg_and_new_url_arg_HashChangeEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&HashChangeEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HashChangeEvent {
    #[cfg(all(feature = "HashChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initHashChangeEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)\n\n*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg_and_new_url_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        old_url_arg: &str,
        new_url_arg: &str,
    ) {
        #[cfg(all(feature = "HashChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg_and_new_url_arg_HashChangeEvent(
                self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                old_url_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_url_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg_and_new_url_arg_HashChangeEvent(
            self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            old_url_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_url_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(old_url_arg);
            drop(new_url_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let old_url_arg =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(old_url_arg);
                let new_url_arg =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_url_arg);
                __widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg_and_new_url_arg_HashChangeEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , old_url_arg , new_url_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "HashChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_old_url_HashChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HashChangeEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HashChangeEvent {
    #[cfg(all(feature = "HashChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `oldURL` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/oldURL)\n\n*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn old_url(&self) -> String {
        #[cfg(all(feature = "HashChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_old_url_HashChangeEvent(
                self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_old_url_HashChangeEvent(
            self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_old_url_HashChangeEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HashChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_url_HashChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HashChangeEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HashChangeEvent {
    #[cfg(all(feature = "HashChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `newURL` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/newURL)\n\n*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn new_url(&self) -> String {
        #[cfg(all(feature = "HashChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_url_HashChangeEvent(
                self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_url_HashChangeEvent(
            self_: <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HashChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_new_url_HashChangeEvent(self_)
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
pub static __WASM_BINDGEN_GENERATED_42b50df699c8894d: [u8; 1369usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x17\x05\0\0\0\0\n\0\0\x02\x0FHashChangeEvent!__widl_instanceof_HashChangeEvent\0\0\0\0\x1C__widl_f_new_HashChangeEvent\x01\0\0\x01\x0FHashChangeEvent\0\x01\x01\x05type_\x03new\0\0\01__widl_f_new_with_event_init_dict_HashChangeEvent\x01\0\0\x01\x0FHashChangeEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0/__widl_f_init_hash_change_event_HashChangeEvent\0\0\0\x01\x0FHashChangeEvent\x01\0\0\x01\x02\x05self_\x08type_arg\x13initHashChangeEvent\0\0\0C__widl_f_init_hash_change_event_with_can_bubble_arg_HashChangeEvent\0\0\0\x01\x0FHashChangeEvent\x01\0\0\x01\x03\x05self_\x08type_arg\x0Ecan_bubble_arg\x13initHashChangeEvent\0\0\0V__widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_HashChangeEvent\0\0\0\x01\x0FHashChangeEvent\x01\0\0\x01\x04\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x13initHashChangeEvent\0\0\0f__widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg_HashChangeEvent\0\0\0\x01\x0FHashChangeEvent\x01\0\0\x01\x05\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x0Bold_url_arg\x13initHashChangeEvent\0\0\0v__widl_f_init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg_and_new_url_arg_HashChangeEvent\0\0\0\x01\x0FHashChangeEvent\x01\0\0\x01\x06\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x0Bold_url_arg\x0Bnew_url_arg\x13initHashChangeEvent\0\0\0 __widl_f_old_url_HashChangeEvent\0\0\0\x01\x0FHashChangeEvent\x01\0\x01\x06oldURL\x01\x01\x05self_\x06oldURL\0\0\0 __widl_f_new_url_HashChangeEvent\0\0\0\x01\x0FHashChangeEvent\x01\0\x01\x06newURL\x01\x01\x05self_\x06newURL\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
