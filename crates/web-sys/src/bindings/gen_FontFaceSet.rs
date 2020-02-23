use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FontFaceSet` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FontFaceSet {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FontFaceSet: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FontFaceSet {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(70u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(70u32);
            inform(97u32);
            inform(99u32);
            inform(101u32);
            inform(83u32);
            inform(101u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for FontFaceSet {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for FontFaceSet {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FontFaceSet {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FontFaceSet {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FontFaceSet {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FontFaceSet {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FontFaceSet {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FontFaceSet {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FontFaceSet {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FontFaceSet>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FontFaceSet {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FontFaceSet {
        #[inline]
        fn from(obj: JsValue) -> FontFaceSet {
            FontFaceSet { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FontFaceSet {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FontFaceSet> for FontFaceSet {
        #[inline]
        fn as_ref(&self) -> &FontFaceSet {
            self
        }
    }
    impl From<FontFaceSet> for JsValue {
        #[inline]
        fn from(obj: FontFaceSet) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FontFaceSet {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FontFaceSet(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FontFaceSet(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FontFaceSet(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FontFaceSet { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FontFaceSet) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FontFaceSet> for EventTarget {
    #[inline]
    fn from(obj: FontFaceSet) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for FontFaceSet {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<FontFaceSet> for ::js_sys::Object {
    #[inline]
    fn from(obj: FontFaceSet) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FontFaceSet {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "FontFace", feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <&FontFace as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFace", feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/add)\n\n*This API requires the following crate features to be activated: `FontFace`, `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn add(&self, font: &FontFace) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFace", feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                font: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            font: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(font);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let font = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(font);
                __widl_f_add_FontFaceSet(self_, font)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_check_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `check()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/check)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn check(&self, font: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_check_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                font: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_check_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            font: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(font);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let font = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(font);
                __widl_f_check_FontFaceSet(self_, font)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_check_with_text_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `check()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/check)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn check_with_text(&self, font: &str, text: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_check_with_text_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                font: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_check_with_text_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            font: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(font);
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let font = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(font);
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_check_with_text_FontFaceSet(self_, font, text)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `clear()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/clear)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn clear(&self) {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_FontFaceSet(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FontFace", feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <&FontFace as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFace", feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `delete()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/delete)\n\n*This API requires the following crate features to be activated: `FontFace`, `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn delete(&self, font: &FontFace) -> bool {
        #[cfg(all(feature = "FontFace", feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                font: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            font: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(font);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let font = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(font);
                __widl_f_delete_FontFaceSet(self_, font)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFaceSet", feature = "FontFaceSetIterator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_entries_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <FontFaceSetIterator as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet", feature = "FontFaceSetIterator",))]
    #[allow(bad_style)]
    #[doc = "The `entries()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/entries)\n\n*This API requires the following crate features to be activated: `FontFaceSet`, `FontFaceSetIterator`*"]
    #[allow(clippy::all)]
    pub fn entries(&self) -> FontFaceSetIterator {
        #[cfg(all(feature = "FontFaceSet", feature = "FontFaceSetIterator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_entries_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFaceSetIterator as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_entries_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFaceSetIterator as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_entries_FontFaceSet(self_)
            };
            <FontFaceSetIterator as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_for_each_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `forEach()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/forEach)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn for_each(&self, cb: &::js_sys::Function) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_for_each_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cb: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_for_each_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cb: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cb);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cb = <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cb);
                __widl_f_for_each_FontFaceSet(self_, cb)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_for_each_with_this_arg_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `forEach()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/forEach)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn for_each_with_this_arg(
        &self,
        cb: &::js_sys::Function,
        this_arg: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_for_each_with_this_arg_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cb: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                this_arg: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_for_each_with_this_arg_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cb: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            this_arg: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cb);
            drop(this_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cb = <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cb);
                let this_arg =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        this_arg,
                    );
                __widl_f_for_each_with_this_arg_FontFaceSet(self_, cb, this_arg)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "FontFace", feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <&FontFace as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFace", feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `has()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/has)\n\n*This API requires the following crate features to be activated: `FontFace`, `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn has(&self, font: &FontFace) -> bool {
        #[cfg(all(feature = "FontFace", feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                font: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            font: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(font);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let font = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(font);
                __widl_f_has_FontFaceSet(self_, font)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_load_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `load()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/load)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn load(&self, font: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_load_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                font: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_load_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            font: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(font);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let font = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(font);
                __widl_f_load_FontFaceSet(self_, font)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_load_with_text_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `load()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/load)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn load_with_text(&self, font: &str, text: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_load_with_text_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                font: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_load_with_text_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            font: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(font);
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let font = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(font);
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_load_with_text_FontFaceSet(self_, font, text)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFaceSet", feature = "FontFaceSetIterator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_values_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <FontFaceSetIterator as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet", feature = "FontFaceSetIterator",))]
    #[allow(bad_style)]
    #[doc = "The `values()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/values)\n\n*This API requires the following crate features to be activated: `FontFaceSet`, `FontFaceSetIterator`*"]
    #[allow(clippy::all)]
    pub fn values(&self) -> FontFaceSetIterator {
        #[cfg(all(feature = "FontFaceSet", feature = "FontFaceSetIterator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_values_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFaceSetIterator as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_values_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFaceSetIterator as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_values_FontFaceSet(self_)
            };
            <FontFaceSetIterator as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_size_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `size` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/size)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn size(&self) -> u32 {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_size_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_size_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_size_FontFaceSet(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloading_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `onloading` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloading)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn onloading(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloading_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloading_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloading_FontFaceSet(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloading_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `onloading` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloading)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn set_onloading(&self, onloading: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloading_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloading: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloading_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloading: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloading);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloading =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloading,
                    );
                __widl_f_set_onloading_FontFaceSet(self_, onloading)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadingdone_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `onloadingdone` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingdone)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn onloadingdone(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadingdone_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadingdone_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadingdone_FontFaceSet(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadingdone_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `onloadingdone` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingdone)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn set_onloadingdone(&self, onloadingdone: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadingdone_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadingdone : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadingdone_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadingdone: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadingdone);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadingdone =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadingdone,
                    );
                __widl_f_set_onloadingdone_FontFaceSet(self_, onloadingdone)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadingerror_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `onloadingerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingerror)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn onloadingerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadingerror_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadingerror_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadingerror_FontFaceSet(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadingerror_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `onloadingerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingerror)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn set_onloadingerror(&self, onloadingerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadingerror_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadingerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadingerror_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadingerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onloadingerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadingerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadingerror,
                    );
                __widl_f_set_onloadingerror_FontFaceSet(self_, onloadingerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `ready` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/ready)\n\n*This API requires the following crate features to be activated: `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn ready(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_FontFaceSet(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "FontFaceSet", feature = "FontFaceSetLoadStatus",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_status_FontFaceSet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFaceSet as WasmDescribe>::describe();
    <FontFaceSetLoadStatus as WasmDescribe>::describe();
}
impl FontFaceSet {
    #[cfg(all(feature = "FontFaceSet", feature = "FontFaceSetLoadStatus",))]
    #[allow(bad_style)]
    #[doc = "The `status` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/status)\n\n*This API requires the following crate features to be activated: `FontFaceSet`, `FontFaceSetLoadStatus`*"]
    #[allow(clippy::all)]
    pub fn status(&self) -> FontFaceSetLoadStatus {
        #[cfg(all(feature = "FontFaceSet", feature = "FontFaceSetLoadStatus",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_status_FontFaceSet(
                self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFaceSetLoadStatus as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_status_FontFaceSet(
            self_: <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFaceSetLoadStatus as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFaceSet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_status_FontFaceSet(self_)
            };
            <FontFaceSetLoadStatus as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d56ff4320ded66ed: [u8; 1805usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCB\x06\0\0\0\0\x16\0\0\x02\x0BFontFaceSet\x1D__widl_instanceof_FontFaceSet\0\0\0\0\x18__widl_f_add_FontFaceSet\x01\0\0\x01\x0BFontFaceSet\x01\0\0\x01\x02\x05self_\x04font\x03add\0\0\0\x1A__widl_f_check_FontFaceSet\x01\0\0\x01\x0BFontFaceSet\x01\0\0\x01\x02\x05self_\x04font\x05check\0\0\0$__widl_f_check_with_text_FontFaceSet\x01\0\0\x01\x0BFontFaceSet\x01\0\0\x01\x03\x05self_\x04font\x04text\x05check\0\0\0\x1A__widl_f_clear_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\0\x01\x01\x05self_\x05clear\0\0\0\x1B__widl_f_delete_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\0\x01\x02\x05self_\x04font\x06delete\0\0\0\x1C__widl_f_entries_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\0\x01\x01\x05self_\x07entries\0\0\0\x1D__widl_f_for_each_FontFaceSet\x01\0\0\x01\x0BFontFaceSet\x01\0\0\x01\x02\x05self_\x02cb\x07forEach\0\0\0+__widl_f_for_each_with_this_arg_FontFaceSet\x01\0\0\x01\x0BFontFaceSet\x01\0\0\x01\x03\x05self_\x02cb\x08this_arg\x07forEach\0\0\0\x18__widl_f_has_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\0\x01\x02\x05self_\x04font\x03has\0\0\0\x19__widl_f_load_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\0\x01\x02\x05self_\x04font\x04load\0\0\0#__widl_f_load_with_text_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\0\x01\x03\x05self_\x04font\x04text\x04load\0\0\0\x1B__widl_f_values_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\0\x01\x01\x05self_\x06values\0\0\0\x19__widl_f_size_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\x01\x04size\x01\x01\x05self_\x04size\0\0\0\x1E__widl_f_onloading_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\x01\tonloading\x01\x01\x05self_\tonloading\0\0\0\"__widl_f_set_onloading_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\x02\tonloading\x01\x02\x05self_\tonloading\tonloading\0\0\0\"__widl_f_onloadingdone_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\x01\ronloadingdone\x01\x01\x05self_\ronloadingdone\0\0\0&__widl_f_set_onloadingdone_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\x02\ronloadingdone\x01\x02\x05self_\ronloadingdone\ronloadingdone\0\0\0#__widl_f_onloadingerror_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\x01\x0Eonloadingerror\x01\x01\x05self_\x0Eonloadingerror\0\0\0'__widl_f_set_onloadingerror_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\x02\x0Eonloadingerror\x01\x02\x05self_\x0Eonloadingerror\x0Eonloadingerror\0\0\0\x1A__widl_f_ready_FontFaceSet\x01\0\0\x01\x0BFontFaceSet\x01\0\x01\x05ready\x01\x01\x05self_\x05ready\0\0\0\x1B__widl_f_status_FontFaceSet\0\0\0\x01\x0BFontFaceSet\x01\0\x01\x06status\x01\x01\x05self_\x06status\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
