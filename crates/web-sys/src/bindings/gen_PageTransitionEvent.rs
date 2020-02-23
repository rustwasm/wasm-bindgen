use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PageTransitionEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent)\n\n*This API requires the following crate features to be activated: `PageTransitionEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PageTransitionEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PageTransitionEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PageTransitionEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(80u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(110u32);
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
    impl core::ops::Deref for PageTransitionEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for PageTransitionEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PageTransitionEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PageTransitionEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PageTransitionEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PageTransitionEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PageTransitionEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PageTransitionEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PageTransitionEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PageTransitionEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PageTransitionEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PageTransitionEvent {
        #[inline]
        fn from(obj: JsValue) -> PageTransitionEvent {
            PageTransitionEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PageTransitionEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PageTransitionEvent> for PageTransitionEvent {
        #[inline]
        fn as_ref(&self) -> &PageTransitionEvent {
            self
        }
    }
    impl From<PageTransitionEvent> for JsValue {
        #[inline]
        fn from(obj: PageTransitionEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PageTransitionEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PageTransitionEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PageTransitionEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PageTransitionEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PageTransitionEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PageTransitionEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PageTransitionEvent> for Event {
    #[inline]
    fn from(obj: PageTransitionEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for PageTransitionEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PageTransitionEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: PageTransitionEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PageTransitionEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PageTransitionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_PageTransitionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <PageTransitionEvent as WasmDescribe>::describe();
}
impl PageTransitionEvent {
    #[cfg(all(feature = "PageTransitionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new PageTransitionEvent(..)` constructor, creating a new instance of `PageTransitionEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/PageTransitionEvent)\n\n*This API requires the following crate features to be activated: `PageTransitionEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<PageTransitionEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PageTransitionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_PageTransitionEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PageTransitionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_PageTransitionEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PageTransitionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_PageTransitionEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PageTransitionEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PageTransitionEvent", feature = "PageTransitionEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_PageTransitionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&PageTransitionEventInit as WasmDescribe>::describe();
    <PageTransitionEvent as WasmDescribe>::describe();
}
impl PageTransitionEvent {
    #[cfg(all(feature = "PageTransitionEvent", feature = "PageTransitionEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new PageTransitionEvent(..)` constructor, creating a new instance of `PageTransitionEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/PageTransitionEvent)\n\n*This API requires the following crate features to be activated: `PageTransitionEvent`, `PageTransitionEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PageTransitionEventInit,
    ) -> Result<PageTransitionEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PageTransitionEvent", feature = "PageTransitionEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_PageTransitionEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & PageTransitionEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <PageTransitionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_PageTransitionEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&PageTransitionEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PageTransitionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&PageTransitionEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_PageTransitionEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PageTransitionEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PageTransitionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_persisted_PageTransitionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PageTransitionEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl PageTransitionEvent {
    #[cfg(all(feature = "PageTransitionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `persisted` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/persisted)\n\n*This API requires the following crate features to be activated: `PageTransitionEvent`*"]
    #[allow(clippy::all)]
    pub fn persisted(&self) -> bool {
        #[cfg(all(feature = "PageTransitionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_persisted_PageTransitionEvent(
                self_: <&PageTransitionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_persisted_PageTransitionEvent(
            self_: <&PageTransitionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PageTransitionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_persisted_PageTransitionEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_400ab81fc0811638: [u8; 447usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}}\x01\0\0\0\0\x04\0\0\x02\x13PageTransitionEvent%__widl_instanceof_PageTransitionEvent\0\0\0\0 __widl_f_new_PageTransitionEvent\x01\0\0\x01\x13PageTransitionEvent\0\x01\x01\x05type_\x03new\0\0\05__widl_f_new_with_event_init_dict_PageTransitionEvent\x01\0\0\x01\x13PageTransitionEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0&__widl_f_persisted_PageTransitionEvent\0\0\0\x01\x13PageTransitionEvent\x01\0\x01\tpersisted\x01\x01\x05self_\tpersisted\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
