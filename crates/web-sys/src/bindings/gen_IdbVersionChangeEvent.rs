use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBVersionChangeEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent)\n\n*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbVersionChangeEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbVersionChangeEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbVersionChangeEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(86u32);
            inform(101u32);
            inform(114u32);
            inform(115u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
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
    impl core::ops::Deref for IdbVersionChangeEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbVersionChangeEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbVersionChangeEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbVersionChangeEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbVersionChangeEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbVersionChangeEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbVersionChangeEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbVersionChangeEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbVersionChangeEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbVersionChangeEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbVersionChangeEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbVersionChangeEvent {
        #[inline]
        fn from(obj: JsValue) -> IdbVersionChangeEvent {
            IdbVersionChangeEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbVersionChangeEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbVersionChangeEvent> for IdbVersionChangeEvent {
        #[inline]
        fn as_ref(&self) -> &IdbVersionChangeEvent {
            self
        }
    }
    impl From<IdbVersionChangeEvent> for JsValue {
        #[inline]
        fn from(obj: IdbVersionChangeEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbVersionChangeEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBVersionChangeEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBVersionChangeEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBVersionChangeEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbVersionChangeEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbVersionChangeEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbVersionChangeEvent> for Event {
    #[inline]
    fn from(obj: IdbVersionChangeEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for IdbVersionChangeEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IdbVersionChangeEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbVersionChangeEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbVersionChangeEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbVersionChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_IDBVersionChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <IdbVersionChangeEvent as WasmDescribe>::describe();
}
impl IdbVersionChangeEvent {
    #[cfg(all(feature = "IdbVersionChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new IDBVersionChangeEvent(..)` constructor, creating a new instance of `IDBVersionChangeEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/IDBVersionChangeEvent)\n\n*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<IdbVersionChangeEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbVersionChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_IDBVersionChangeEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbVersionChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_IDBVersionChangeEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbVersionChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_IDBVersionChangeEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbVersionChangeEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "IdbVersionChangeEvent",
    feature = "IdbVersionChangeEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_IDBVersionChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&IdbVersionChangeEventInit as WasmDescribe>::describe();
    <IdbVersionChangeEvent as WasmDescribe>::describe();
}
impl IdbVersionChangeEvent {
    #[cfg(all(
        feature = "IdbVersionChangeEvent",
        feature = "IdbVersionChangeEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new IDBVersionChangeEvent(..)` constructor, creating a new instance of `IDBVersionChangeEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/IDBVersionChangeEvent)\n\n*This API requires the following crate features to be activated: `IdbVersionChangeEvent`, `IdbVersionChangeEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &IdbVersionChangeEventInit,
    ) -> Result<IdbVersionChangeEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "IdbVersionChangeEvent",
            feature = "IdbVersionChangeEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_IDBVersionChangeEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & IdbVersionChangeEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <IdbVersionChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_IDBVersionChangeEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & IdbVersionChangeEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <IdbVersionChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&IdbVersionChangeEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_IDBVersionChangeEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbVersionChangeEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbVersionChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_old_version_IDBVersionChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbVersionChangeEvent as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl IdbVersionChangeEvent {
    #[cfg(all(feature = "IdbVersionChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `oldVersion` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/oldVersion)\n\n*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn old_version(&self) -> f64 {
        #[cfg(all(feature = "IdbVersionChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_old_version_IDBVersionChangeEvent(
                self_: <&IdbVersionChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_old_version_IDBVersionChangeEvent(
            self_: <&IdbVersionChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&IdbVersionChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_old_version_IDBVersionChangeEvent(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbVersionChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_version_IDBVersionChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbVersionChangeEvent as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl IdbVersionChangeEvent {
    #[cfg(all(feature = "IdbVersionChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `newVersion` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/newVersion)\n\n*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn new_version(&self) -> Option<f64> {
        #[cfg(all(feature = "IdbVersionChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_version_IDBVersionChangeEvent(
                self_: <&IdbVersionChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_version_IDBVersionChangeEvent(
            self_: <&IdbVersionChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&IdbVersionChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_new_version_IDBVersionChangeEvent(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2a705e01d156d964: [u8; 572usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xFA\x01\0\0\0\0\x05\0\0\x02\x15IDBVersionChangeEvent'__widl_instanceof_IDBVersionChangeEvent\0\0\0\0\"__widl_f_new_IDBVersionChangeEvent\x01\0\0\x01\x15IDBVersionChangeEvent\0\x01\x01\x05type_\x03new\0\0\07__widl_f_new_with_event_init_dict_IDBVersionChangeEvent\x01\0\0\x01\x15IDBVersionChangeEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0*__widl_f_old_version_IDBVersionChangeEvent\0\0\0\x01\x15IDBVersionChangeEvent\x01\0\x01\noldVersion\x01\x01\x05self_\noldVersion\0\0\0*__widl_f_new_version_IDBVersionChangeEvent\0\0\0\x01\x15IDBVersionChangeEvent\x01\0\x01\nnewVersion\x01\x01\x05self_\nnewVersion\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
