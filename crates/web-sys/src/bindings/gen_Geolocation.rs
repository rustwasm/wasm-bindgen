use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Geolocation` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation)\n\n*This API requires the following crate features to be activated: `Geolocation`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Geolocation {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Geolocation: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Geolocation {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(71u32);
            inform(101u32);
            inform(111u32);
            inform(108u32);
            inform(111u32);
            inform(99u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for Geolocation {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Geolocation {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Geolocation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Geolocation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Geolocation {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Geolocation {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Geolocation {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Geolocation {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Geolocation {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Geolocation>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Geolocation {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Geolocation {
        #[inline]
        fn from(obj: JsValue) -> Geolocation {
            Geolocation { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Geolocation {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Geolocation> for Geolocation {
        #[inline]
        fn as_ref(&self) -> &Geolocation {
            self
        }
    }
    impl From<Geolocation> for JsValue {
        #[inline]
        fn from(obj: Geolocation) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Geolocation {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Geolocation(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Geolocation(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Geolocation(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Geolocation { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Geolocation) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Geolocation> for ::js_sys::Object {
    #[inline]
    fn from(obj: Geolocation) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Geolocation {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Geolocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_watch_Geolocation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Geolocation as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Geolocation {
    #[cfg(all(feature = "Geolocation",))]
    #[allow(bad_style)]
    #[doc = "The `clearWatch()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/clearWatch)\n\n*This API requires the following crate features to be activated: `Geolocation`*"]
    #[allow(clippy::all)]
    pub fn clear_watch(&self, watch_id: i32) {
        #[cfg(all(feature = "Geolocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_watch_Geolocation(
                self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                watch_id: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_watch_Geolocation(
            self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            watch_id: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(watch_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let watch_id = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(watch_id);
                __widl_f_clear_watch_Geolocation(self_, watch_id)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Geolocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_current_position_Geolocation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Geolocation as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Geolocation {
    #[cfg(all(feature = "Geolocation",))]
    #[allow(bad_style)]
    #[doc = "The `getCurrentPosition()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)\n\n*This API requires the following crate features to be activated: `Geolocation`*"]
    #[allow(clippy::all)]
    pub fn get_current_position(
        &self,
        success_callback: &::js_sys::Function,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Geolocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_current_position_Geolocation(
                self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_current_position_Geolocation(
            self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(success_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_get_current_position_Geolocation(self_, success_callback)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Geolocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_current_position_with_error_callback_Geolocation(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Geolocation as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Geolocation {
    #[cfg(all(feature = "Geolocation",))]
    #[allow(bad_style)]
    #[doc = "The `getCurrentPosition()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)\n\n*This API requires the following crate features to be activated: `Geolocation`*"]
    #[allow(clippy::all)]
    pub fn get_current_position_with_error_callback(
        &self,
        success_callback: &::js_sys::Function,
        error_callback: Option<&::js_sys::Function>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Geolocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_current_position_with_error_callback_Geolocation(
                self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_current_position_with_error_callback_Geolocation(
            self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(success_callback);
            drop(error_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_get_current_position_with_error_callback_Geolocation(
                    self_,
                    success_callback,
                    error_callback,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Geolocation", feature = "PositionOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_current_position_with_error_callback_and_options_Geolocation(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Geolocation as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <&PositionOptions as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Geolocation {
    #[cfg(all(feature = "Geolocation", feature = "PositionOptions",))]
    #[allow(bad_style)]
    #[doc = "The `getCurrentPosition()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)\n\n*This API requires the following crate features to be activated: `Geolocation`, `PositionOptions`*"]
    #[allow(clippy::all)]
    pub fn get_current_position_with_error_callback_and_options(
        &self,
        success_callback: &::js_sys::Function,
        error_callback: Option<&::js_sys::Function>,
        options: &PositionOptions,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Geolocation", feature = "PositionOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_current_position_with_error_callback_and_options_Geolocation(
                self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                options: <&PositionOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_current_position_with_error_callback_and_options_Geolocation(
            self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            options: <&PositionOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(success_callback);
            drop(error_callback);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                let options =
                    <&PositionOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_get_current_position_with_error_callback_and_options_Geolocation(
                    self_,
                    success_callback,
                    error_callback,
                    options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Geolocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_watch_position_Geolocation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Geolocation as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Geolocation {
    #[cfg(all(feature = "Geolocation",))]
    #[allow(bad_style)]
    #[doc = "The `watchPosition()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)\n\n*This API requires the following crate features to be activated: `Geolocation`*"]
    #[allow(clippy::all)]
    pub fn watch_position(
        &self,
        success_callback: &::js_sys::Function,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Geolocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_watch_position_Geolocation(
                self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_watch_position_Geolocation(
            self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(success_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_watch_position_Geolocation(self_, success_callback)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Geolocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_watch_position_with_error_callback_Geolocation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Geolocation as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Geolocation {
    #[cfg(all(feature = "Geolocation",))]
    #[allow(bad_style)]
    #[doc = "The `watchPosition()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)\n\n*This API requires the following crate features to be activated: `Geolocation`*"]
    #[allow(clippy::all)]
    pub fn watch_position_with_error_callback(
        &self,
        success_callback: &::js_sys::Function,
        error_callback: Option<&::js_sys::Function>,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Geolocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_watch_position_with_error_callback_Geolocation(
                self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_watch_position_with_error_callback_Geolocation(
            self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(success_callback);
            drop(error_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_watch_position_with_error_callback_Geolocation(
                    self_,
                    success_callback,
                    error_callback,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Geolocation", feature = "PositionOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_watch_position_with_error_callback_and_options_Geolocation(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Geolocation as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <&PositionOptions as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Geolocation {
    #[cfg(all(feature = "Geolocation", feature = "PositionOptions",))]
    #[allow(bad_style)]
    #[doc = "The `watchPosition()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)\n\n*This API requires the following crate features to be activated: `Geolocation`, `PositionOptions`*"]
    #[allow(clippy::all)]
    pub fn watch_position_with_error_callback_and_options(
        &self,
        success_callback: &::js_sys::Function,
        error_callback: Option<&::js_sys::Function>,
        options: &PositionOptions,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Geolocation", feature = "PositionOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_watch_position_with_error_callback_and_options_Geolocation(
                self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                options: <&PositionOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_watch_position_with_error_callback_and_options_Geolocation(
            self_: <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            options: <&PositionOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(success_callback);
            drop(error_callback);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Geolocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                let options =
                    <&PositionOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_watch_position_with_error_callback_and_options_Geolocation(
                    self_,
                    success_callback,
                    error_callback,
                    options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_4f560928382430fc: [u8; 1029usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC3\x03\0\0\0\0\x08\0\0\x02\x0BGeolocation\x1D__widl_instanceof_Geolocation\0\0\0\0 __widl_f_clear_watch_Geolocation\0\0\0\x01\x0BGeolocation\x01\0\0\x01\x02\x05self_\x08watch_id\nclearWatch\0\0\0)__widl_f_get_current_position_Geolocation\x01\0\0\x01\x0BGeolocation\x01\0\0\x01\x02\x05self_\x10success_callback\x12getCurrentPosition\0\0\0=__widl_f_get_current_position_with_error_callback_Geolocation\x01\0\0\x01\x0BGeolocation\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\x12getCurrentPosition\0\0\0I__widl_f_get_current_position_with_error_callback_and_options_Geolocation\x01\0\0\x01\x0BGeolocation\x01\0\0\x01\x04\x05self_\x10success_callback\x0Eerror_callback\x07options\x12getCurrentPosition\0\0\0#__widl_f_watch_position_Geolocation\x01\0\0\x01\x0BGeolocation\x01\0\0\x01\x02\x05self_\x10success_callback\rwatchPosition\0\0\07__widl_f_watch_position_with_error_callback_Geolocation\x01\0\0\x01\x0BGeolocation\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\rwatchPosition\0\0\0C__widl_f_watch_position_with_error_callback_and_options_Geolocation\x01\0\0\x01\x0BGeolocation\x01\0\0\x01\x04\x05self_\x10success_callback\x0Eerror_callback\x07options\rwatchPosition\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
