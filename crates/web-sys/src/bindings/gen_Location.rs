use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Location` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location)\n\n*This API requires the following crate features to be activated: `Location`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Location {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Location: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Location {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(76u32);
            inform(111u32);
            inform(99u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for Location {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Location {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Location {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Location {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Location {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Location {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Location {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Location {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Location {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Location>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Location {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Location {
        #[inline]
        fn from(obj: JsValue) -> Location {
            Location { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Location {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Location> for Location {
        #[inline]
        fn as_ref(&self) -> &Location {
            self
        }
    }
    impl From<Location> for JsValue {
        #[inline]
        fn from(obj: Location) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Location {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Location(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Location(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Location(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Location { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Location) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Location> for ::js_sys::Object {
    #[inline]
    fn from(obj: Location) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Location {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_assign_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Location as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `assign()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/assign)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn assign(&self, url: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assign_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assign_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_assign_Location(self_, url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reload_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Location as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `reload()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/reload)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn reload(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reload_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reload_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_reload_Location(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reload_with_forceget_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Location as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `reload()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/reload)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn reload_with_forceget(&self, forceget: bool) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reload_with_forceget_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                forceget: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reload_with_forceget_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            forceget: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(forceget);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let forceget = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(forceget);
                __widl_f_reload_with_forceget_Location(self_, forceget)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Location as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `replace()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/replace)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn replace(&self, url: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_replace_Location(self_, url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_href_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Location as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `href` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/href)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn href(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_href_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_href_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_href_Location(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_href_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Location as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `href` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/href)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn set_href(&self, href: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_href_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                href: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_href_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            href: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(href);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let href = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(href);
                __widl_f_set_href_Location(self_, href)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_origin_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Location as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `origin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/origin)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn origin(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_origin_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_origin_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_origin_Location(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_protocol_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Location as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `protocol` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn protocol(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_protocol_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_protocol_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_protocol_Location(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_protocol_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Location as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `protocol` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn set_protocol(&self, protocol: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_protocol_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                protocol: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_protocol_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            protocol: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(protocol);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let protocol = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(protocol);
                __widl_f_set_protocol_Location(self_, protocol)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_host_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Location as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `host` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/host)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn host(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_host_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_host_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_host_Location(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_host_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Location as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `host` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/host)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn set_host(&self, host: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_host_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                host: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_host_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            host: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(host);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let host = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(host);
                __widl_f_set_host_Location(self_, host)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hostname_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Location as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `hostname` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn hostname(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hostname_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hostname_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hostname_Location(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hostname_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Location as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `hostname` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn set_hostname(&self, hostname: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hostname_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hostname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hostname_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hostname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(hostname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hostname = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hostname);
                __widl_f_set_hostname_Location(self_, hostname)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_port_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Location as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `port` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/port)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn port(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_port_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_port_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_port_Location(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_port_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Location as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `port` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/port)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn set_port(&self, port: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_port_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                port: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_port_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            port: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(port);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let port = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(port);
                __widl_f_set_port_Location(self_, port)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pathname_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Location as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `pathname` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn pathname(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pathname_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pathname_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pathname_Location(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_pathname_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Location as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `pathname` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn set_pathname(&self, pathname: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_pathname_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pathname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_pathname_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pathname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(pathname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pathname = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pathname);
                __widl_f_set_pathname_Location(self_, pathname)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_search_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Location as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `search` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/search)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn search(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_search_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_search_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_search_Location(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_search_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Location as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `search` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/search)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn set_search(&self, search: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_search_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                search: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_search_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            search: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(search);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let search = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(search);
                __widl_f_set_search_Location(self_, search)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hash_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Location as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `hash` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hash)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn hash(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hash_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hash_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hash_Location(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hash_Location() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Location as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Location {
    #[cfg(all(feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `hash` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hash)\n\n*This API requires the following crate features to be activated: `Location`*"]
    #[allow(clippy::all)]
    pub fn set_hash(&self, hash: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hash_Location(
                self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hash: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hash_Location(
            self_: <&Location as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hash: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(hash);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Location as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hash = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hash);
                __widl_f_set_hash_Location(self_, hash)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_397f6e32e3499042: [u8; 1610usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x08\x06\0\0\0\0\x16\0\0\x02\x08Location\x1A__widl_instanceof_Location\0\0\0\0\x18__widl_f_assign_Location\x01\0\0\x01\x08Location\x01\0\0\x01\x02\x05self_\x03url\x06assign\0\0\0\x18__widl_f_reload_Location\x01\0\0\x01\x08Location\x01\0\0\x01\x01\x05self_\x06reload\0\0\0&__widl_f_reload_with_forceget_Location\x01\0\0\x01\x08Location\x01\0\0\x01\x02\x05self_\x08forceget\x06reload\0\0\0\x19__widl_f_replace_Location\x01\0\0\x01\x08Location\x01\0\0\x01\x02\x05self_\x03url\x07replace\0\0\0\x16__widl_f_href_Location\x01\0\0\x01\x08Location\x01\0\x01\x04href\x01\x01\x05self_\x04href\0\0\0\x1A__widl_f_set_href_Location\x01\0\0\x01\x08Location\x01\0\x02\x04href\x01\x02\x05self_\x04href\x04href\0\0\0\x18__widl_f_origin_Location\x01\0\0\x01\x08Location\x01\0\x01\x06origin\x01\x01\x05self_\x06origin\0\0\0\x1A__widl_f_protocol_Location\x01\0\0\x01\x08Location\x01\0\x01\x08protocol\x01\x01\x05self_\x08protocol\0\0\0\x1E__widl_f_set_protocol_Location\x01\0\0\x01\x08Location\x01\0\x02\x08protocol\x01\x02\x05self_\x08protocol\x08protocol\0\0\0\x16__widl_f_host_Location\x01\0\0\x01\x08Location\x01\0\x01\x04host\x01\x01\x05self_\x04host\0\0\0\x1A__widl_f_set_host_Location\x01\0\0\x01\x08Location\x01\0\x02\x04host\x01\x02\x05self_\x04host\x04host\0\0\0\x1A__widl_f_hostname_Location\x01\0\0\x01\x08Location\x01\0\x01\x08hostname\x01\x01\x05self_\x08hostname\0\0\0\x1E__widl_f_set_hostname_Location\x01\0\0\x01\x08Location\x01\0\x02\x08hostname\x01\x02\x05self_\x08hostname\x08hostname\0\0\0\x16__widl_f_port_Location\x01\0\0\x01\x08Location\x01\0\x01\x04port\x01\x01\x05self_\x04port\0\0\0\x1A__widl_f_set_port_Location\x01\0\0\x01\x08Location\x01\0\x02\x04port\x01\x02\x05self_\x04port\x04port\0\0\0\x1A__widl_f_pathname_Location\x01\0\0\x01\x08Location\x01\0\x01\x08pathname\x01\x01\x05self_\x08pathname\0\0\0\x1E__widl_f_set_pathname_Location\x01\0\0\x01\x08Location\x01\0\x02\x08pathname\x01\x02\x05self_\x08pathname\x08pathname\0\0\0\x18__widl_f_search_Location\x01\0\0\x01\x08Location\x01\0\x01\x06search\x01\x01\x05self_\x06search\0\0\0\x1C__widl_f_set_search_Location\x01\0\0\x01\x08Location\x01\0\x02\x06search\x01\x02\x05self_\x06search\x06search\0\0\0\x16__widl_f_hash_Location\x01\0\0\x01\x08Location\x01\0\x01\x04hash\x01\x01\x05self_\x04hash\0\0\0\x1A__widl_f_set_hash_Location\x01\0\0\x01\x08Location\x01\0\x02\x04hash\x01\x02\x05self_\x04hash\x04hash\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
