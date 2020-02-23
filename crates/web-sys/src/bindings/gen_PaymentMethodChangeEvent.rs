use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PaymentMethodChangeEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent)\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PaymentMethodChangeEvent {
    obj: PaymentRequestUpdateEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PaymentMethodChangeEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PaymentMethodChangeEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(24u32);
            inform(80u32);
            inform(97u32);
            inform(121u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(77u32);
            inform(101u32);
            inform(116u32);
            inform(104u32);
            inform(111u32);
            inform(100u32);
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
    impl core::ops::Deref for PaymentMethodChangeEvent {
        type Target = PaymentRequestUpdateEvent;
        #[inline]
        fn deref(&self) -> &PaymentRequestUpdateEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for PaymentMethodChangeEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PaymentMethodChangeEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PaymentMethodChangeEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PaymentMethodChangeEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PaymentMethodChangeEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PaymentMethodChangeEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PaymentMethodChangeEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PaymentMethodChangeEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PaymentMethodChangeEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PaymentMethodChangeEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PaymentMethodChangeEvent {
        #[inline]
        fn from(obj: JsValue) -> PaymentMethodChangeEvent {
            PaymentMethodChangeEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PaymentMethodChangeEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PaymentMethodChangeEvent> for PaymentMethodChangeEvent {
        #[inline]
        fn as_ref(&self) -> &PaymentMethodChangeEvent {
            self
        }
    }
    impl From<PaymentMethodChangeEvent> for JsValue {
        #[inline]
        fn from(obj: PaymentMethodChangeEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PaymentMethodChangeEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PaymentMethodChangeEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PaymentMethodChangeEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PaymentMethodChangeEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PaymentMethodChangeEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PaymentMethodChangeEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PaymentMethodChangeEvent> for PaymentRequestUpdateEvent {
    #[inline]
    fn from(obj: PaymentMethodChangeEvent) -> PaymentRequestUpdateEvent {
        use wasm_bindgen::JsCast;
        PaymentRequestUpdateEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<PaymentRequestUpdateEvent> for PaymentMethodChangeEvent {
    #[inline]
    fn as_ref(&self) -> &PaymentRequestUpdateEvent {
        use wasm_bindgen::JsCast;
        PaymentRequestUpdateEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PaymentMethodChangeEvent> for Event {
    #[inline]
    fn from(obj: PaymentMethodChangeEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for PaymentMethodChangeEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PaymentMethodChangeEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: PaymentMethodChangeEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PaymentMethodChangeEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PaymentMethodChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_PaymentMethodChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <PaymentMethodChangeEvent as WasmDescribe>::describe();
}
impl PaymentMethodChangeEvent {
    #[cfg(all(feature = "PaymentMethodChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new PaymentMethodChangeEvent(..)` constructor, creating a new instance of `PaymentMethodChangeEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/PaymentMethodChangeEvent)\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<PaymentMethodChangeEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PaymentMethodChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_PaymentMethodChangeEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PaymentMethodChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_PaymentMethodChangeEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PaymentMethodChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_PaymentMethodChangeEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PaymentMethodChangeEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "PaymentMethodChangeEvent",
    feature = "PaymentMethodChangeEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_PaymentMethodChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&PaymentMethodChangeEventInit as WasmDescribe>::describe();
    <PaymentMethodChangeEvent as WasmDescribe>::describe();
}
impl PaymentMethodChangeEvent {
    #[cfg(all(
        feature = "PaymentMethodChangeEvent",
        feature = "PaymentMethodChangeEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new PaymentMethodChangeEvent(..)` constructor, creating a new instance of `PaymentMethodChangeEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/PaymentMethodChangeEvent)\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`, `PaymentMethodChangeEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PaymentMethodChangeEventInit,
    ) -> Result<PaymentMethodChangeEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "PaymentMethodChangeEvent",
            feature = "PaymentMethodChangeEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_PaymentMethodChangeEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & PaymentMethodChangeEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <PaymentMethodChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_PaymentMethodChangeEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & PaymentMethodChangeEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <PaymentMethodChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&PaymentMethodChangeEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_PaymentMethodChangeEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PaymentMethodChangeEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PaymentMethodChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_method_name_PaymentMethodChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentMethodChangeEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaymentMethodChangeEvent {
    #[cfg(all(feature = "PaymentMethodChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `methodName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/methodName)\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn method_name(&self) -> String {
        #[cfg(all(feature = "PaymentMethodChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_method_name_PaymentMethodChangeEvent(
                self_: <&PaymentMethodChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_method_name_PaymentMethodChangeEvent(
            self_: <&PaymentMethodChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PaymentMethodChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_method_name_PaymentMethodChangeEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentMethodChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_method_details_PaymentMethodChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentMethodChangeEvent as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl PaymentMethodChangeEvent {
    #[cfg(all(feature = "PaymentMethodChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `methodDetails` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/methodDetails)\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn method_details(&self) -> Option<::js_sys::Object> {
        #[cfg(all(feature = "PaymentMethodChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_method_details_PaymentMethodChangeEvent(
                self_: <&PaymentMethodChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_method_details_PaymentMethodChangeEvent(
            self_: <&PaymentMethodChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PaymentMethodChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_method_details_PaymentMethodChangeEvent(self_)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a47f745e539dbc7b: [u8; 611usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}!\x02\0\0\0\0\x05\0\0\x02\x18PaymentMethodChangeEvent*__widl_instanceof_PaymentMethodChangeEvent\0\0\0\0%__widl_f_new_PaymentMethodChangeEvent\x01\0\0\x01\x18PaymentMethodChangeEvent\0\x01\x01\x05type_\x03new\0\0\0:__widl_f_new_with_event_init_dict_PaymentMethodChangeEvent\x01\0\0\x01\x18PaymentMethodChangeEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0-__widl_f_method_name_PaymentMethodChangeEvent\0\0\0\x01\x18PaymentMethodChangeEvent\x01\0\x01\nmethodName\x01\x01\x05self_\nmethodName\0\0\00__widl_f_method_details_PaymentMethodChangeEvent\0\0\0\x01\x18PaymentMethodChangeEvent\x01\0\x01\rmethodDetails\x01\x01\x05self_\rmethodDetails\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
