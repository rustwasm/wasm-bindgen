use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Touch` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch)\n\n*This API requires the following crate features to be activated: `Touch`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Touch {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Touch: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Touch {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(5u32);
            inform(84u32);
            inform(111u32);
            inform(117u32);
            inform(99u32);
            inform(104u32);
        }
    }
    impl core::ops::Deref for Touch {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Touch {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Touch {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Touch {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Touch {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Touch {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Touch {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Touch {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Touch {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Touch>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Touch {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Touch {
        #[inline]
        fn from(obj: JsValue) -> Touch {
            Touch { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Touch {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Touch> for Touch {
        #[inline]
        fn as_ref(&self) -> &Touch {
            self
        }
    }
    impl From<Touch> for JsValue {
        #[inline]
        fn from(obj: Touch) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Touch {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Touch(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Touch(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Touch(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Touch { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Touch) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Touch> for ::js_sys::Object {
    #[inline]
    fn from(obj: Touch) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Touch {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Touch", feature = "TouchInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Touch() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TouchInit as WasmDescribe>::describe();
    <Touch as WasmDescribe>::describe();
}
impl Touch {
    #[cfg(all(feature = "Touch", feature = "TouchInit",))]
    #[allow(bad_style)]
    #[doc = "The `new Touch(..)` constructor, creating a new instance of `Touch`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/Touch)\n\n*This API requires the following crate features to be activated: `Touch`, `TouchInit`*"]
    #[allow(clippy::all)]
    pub fn new(touch_init_dict: &TouchInit) -> Result<Touch, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Touch", feature = "TouchInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Touch(
                touch_init_dict: <&TouchInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Touch as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Touch(
            touch_init_dict: <&TouchInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Touch as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(touch_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let touch_init_dict =
                    <&TouchInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(touch_init_dict);
                __widl_f_new_Touch(touch_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Touch as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Touch",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_identifier_Touch() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Touch as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Touch {
    #[cfg(all(feature = "Touch",))]
    #[allow(bad_style)]
    #[doc = "The `identifier` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/identifier)\n\n*This API requires the following crate features to be activated: `Touch`*"]
    #[allow(clippy::all)]
    pub fn identifier(&self) -> i32 {
        #[cfg(all(feature = "Touch",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_identifier_Touch(
                self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_identifier_Touch(
            self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Touch as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_identifier_Touch(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "EventTarget", feature = "Touch",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_Touch() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Touch as WasmDescribe>::describe();
    <Option<EventTarget> as WasmDescribe>::describe();
}
impl Touch {
    #[cfg(all(feature = "EventTarget", feature = "Touch",))]
    #[allow(bad_style)]
    #[doc = "The `target` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/target)\n\n*This API requires the following crate features to be activated: `EventTarget`, `Touch`*"]
    #[allow(clippy::all)]
    pub fn target(&self) -> Option<EventTarget> {
        #[cfg(all(feature = "EventTarget", feature = "Touch",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_Touch(
                self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_Touch(
            self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Touch as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_target_Touch(self_)
            };
            <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Touch",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_screen_x_Touch() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Touch as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Touch {
    #[cfg(all(feature = "Touch",))]
    #[allow(bad_style)]
    #[doc = "The `screenX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/screenX)\n\n*This API requires the following crate features to be activated: `Touch`*"]
    #[allow(clippy::all)]
    pub fn screen_x(&self) -> i32 {
        #[cfg(all(feature = "Touch",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_screen_x_Touch(
                self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_screen_x_Touch(
            self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Touch as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_screen_x_Touch(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Touch",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_screen_y_Touch() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Touch as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Touch {
    #[cfg(all(feature = "Touch",))]
    #[allow(bad_style)]
    #[doc = "The `screenY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/screenY)\n\n*This API requires the following crate features to be activated: `Touch`*"]
    #[allow(clippy::all)]
    pub fn screen_y(&self) -> i32 {
        #[cfg(all(feature = "Touch",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_screen_y_Touch(
                self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_screen_y_Touch(
            self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Touch as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_screen_y_Touch(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Touch",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_client_x_Touch() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Touch as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Touch {
    #[cfg(all(feature = "Touch",))]
    #[allow(bad_style)]
    #[doc = "The `clientX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/clientX)\n\n*This API requires the following crate features to be activated: `Touch`*"]
    #[allow(clippy::all)]
    pub fn client_x(&self) -> i32 {
        #[cfg(all(feature = "Touch",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_client_x_Touch(
                self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_client_x_Touch(
            self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Touch as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_client_x_Touch(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Touch",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_client_y_Touch() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Touch as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Touch {
    #[cfg(all(feature = "Touch",))]
    #[allow(bad_style)]
    #[doc = "The `clientY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/clientY)\n\n*This API requires the following crate features to be activated: `Touch`*"]
    #[allow(clippy::all)]
    pub fn client_y(&self) -> i32 {
        #[cfg(all(feature = "Touch",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_client_y_Touch(
                self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_client_y_Touch(
            self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Touch as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_client_y_Touch(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Touch",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_page_x_Touch() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Touch as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Touch {
    #[cfg(all(feature = "Touch",))]
    #[allow(bad_style)]
    #[doc = "The `pageX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/pageX)\n\n*This API requires the following crate features to be activated: `Touch`*"]
    #[allow(clippy::all)]
    pub fn page_x(&self) -> i32 {
        #[cfg(all(feature = "Touch",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_page_x_Touch(
                self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_page_x_Touch(
            self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Touch as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_page_x_Touch(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Touch",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_page_y_Touch() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Touch as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Touch {
    #[cfg(all(feature = "Touch",))]
    #[allow(bad_style)]
    #[doc = "The `pageY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/pageY)\n\n*This API requires the following crate features to be activated: `Touch`*"]
    #[allow(clippy::all)]
    pub fn page_y(&self) -> i32 {
        #[cfg(all(feature = "Touch",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_page_y_Touch(
                self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_page_y_Touch(
            self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Touch as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_page_y_Touch(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Touch",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_radius_x_Touch() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Touch as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Touch {
    #[cfg(all(feature = "Touch",))]
    #[allow(bad_style)]
    #[doc = "The `radiusX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/radiusX)\n\n*This API requires the following crate features to be activated: `Touch`*"]
    #[allow(clippy::all)]
    pub fn radius_x(&self) -> i32 {
        #[cfg(all(feature = "Touch",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_radius_x_Touch(
                self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_radius_x_Touch(
            self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Touch as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_radius_x_Touch(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Touch",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_radius_y_Touch() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Touch as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Touch {
    #[cfg(all(feature = "Touch",))]
    #[allow(bad_style)]
    #[doc = "The `radiusY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/radiusY)\n\n*This API requires the following crate features to be activated: `Touch`*"]
    #[allow(clippy::all)]
    pub fn radius_y(&self) -> i32 {
        #[cfg(all(feature = "Touch",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_radius_y_Touch(
                self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_radius_y_Touch(
            self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Touch as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_radius_y_Touch(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Touch",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotation_angle_Touch() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Touch as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl Touch {
    #[cfg(all(feature = "Touch",))]
    #[allow(bad_style)]
    #[doc = "The `rotationAngle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/rotationAngle)\n\n*This API requires the following crate features to be activated: `Touch`*"]
    #[allow(clippy::all)]
    pub fn rotation_angle(&self) -> f32 {
        #[cfg(all(feature = "Touch",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotation_angle_Touch(
                self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotation_angle_Touch(
            self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Touch as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rotation_angle_Touch(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Touch",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_force_Touch() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Touch as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl Touch {
    #[cfg(all(feature = "Touch",))]
    #[allow(bad_style)]
    #[doc = "The `force` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/force)\n\n*This API requires the following crate features to be activated: `Touch`*"]
    #[allow(clippy::all)]
    pub fn force(&self) -> f32 {
        #[cfg(all(feature = "Touch",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_force_Touch(
                self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_force_Touch(
            self_: <&Touch as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Touch as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_force_Touch(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f2639d0339942a44: [u8; 965usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x83\x03\0\0\0\0\x0E\0\0\x02\x05Touch\x17__widl_instanceof_Touch\0\0\0\0\x12__widl_f_new_Touch\x01\0\0\x01\x05Touch\0\x01\x01\x0Ftouch_init_dict\x03new\0\0\0\x19__widl_f_identifier_Touch\0\0\0\x01\x05Touch\x01\0\x01\nidentifier\x01\x01\x05self_\nidentifier\0\0\0\x15__widl_f_target_Touch\0\0\0\x01\x05Touch\x01\0\x01\x06target\x01\x01\x05self_\x06target\0\0\0\x17__widl_f_screen_x_Touch\0\0\0\x01\x05Touch\x01\0\x01\x07screenX\x01\x01\x05self_\x07screenX\0\0\0\x17__widl_f_screen_y_Touch\0\0\0\x01\x05Touch\x01\0\x01\x07screenY\x01\x01\x05self_\x07screenY\0\0\0\x17__widl_f_client_x_Touch\0\0\0\x01\x05Touch\x01\0\x01\x07clientX\x01\x01\x05self_\x07clientX\0\0\0\x17__widl_f_client_y_Touch\0\0\0\x01\x05Touch\x01\0\x01\x07clientY\x01\x01\x05self_\x07clientY\0\0\0\x15__widl_f_page_x_Touch\0\0\0\x01\x05Touch\x01\0\x01\x05pageX\x01\x01\x05self_\x05pageX\0\0\0\x15__widl_f_page_y_Touch\0\0\0\x01\x05Touch\x01\0\x01\x05pageY\x01\x01\x05self_\x05pageY\0\0\0\x17__widl_f_radius_x_Touch\0\0\0\x01\x05Touch\x01\0\x01\x07radiusX\x01\x01\x05self_\x07radiusX\0\0\0\x17__widl_f_radius_y_Touch\0\0\0\x01\x05Touch\x01\0\x01\x07radiusY\x01\x01\x05self_\x07radiusY\0\0\0\x1D__widl_f_rotation_angle_Touch\0\0\0\x01\x05Touch\x01\0\x01\rrotationAngle\x01\x01\x05self_\rrotationAngle\0\0\0\x14__widl_f_force_Touch\0\0\0\x01\x05Touch\x01\0\x01\x05force\x01\x01\x05self_\x05force\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
