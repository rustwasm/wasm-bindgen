use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ScrollBoxObject` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ScrollBoxObject {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ScrollBoxObject: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ScrollBoxObject {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(83u32);
            inform(99u32);
            inform(114u32);
            inform(111u32);
            inform(108u32);
            inform(108u32);
            inform(66u32);
            inform(111u32);
            inform(120u32);
            inform(79u32);
            inform(98u32);
            inform(106u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for ScrollBoxObject {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ScrollBoxObject {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ScrollBoxObject {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ScrollBoxObject {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ScrollBoxObject {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ScrollBoxObject {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ScrollBoxObject {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ScrollBoxObject {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ScrollBoxObject {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ScrollBoxObject>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ScrollBoxObject {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ScrollBoxObject {
        #[inline]
        fn from(obj: JsValue) -> ScrollBoxObject {
            ScrollBoxObject { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ScrollBoxObject {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ScrollBoxObject> for ScrollBoxObject {
        #[inline]
        fn as_ref(&self) -> &ScrollBoxObject {
            self
        }
    }
    impl From<ScrollBoxObject> for JsValue {
        #[inline]
        fn from(obj: ScrollBoxObject) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ScrollBoxObject {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ScrollBoxObject(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ScrollBoxObject(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ScrollBoxObject(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ScrollBoxObject { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ScrollBoxObject) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ScrollBoxObject> for ::js_sys::Object {
    #[inline]
    fn from(obj: ScrollBoxObject) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ScrollBoxObject {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Element", feature = "ScrollBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ensure_element_is_visible_ScrollBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ScrollBoxObject as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollBoxObject {
    #[cfg(all(feature = "Element", feature = "ScrollBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `ensureElementIsVisible()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/ensureElementIsVisible)\n\n*This API requires the following crate features to be activated: `Element`, `ScrollBoxObject`*"]
    #[allow(clippy::all)]
    pub fn ensure_element_is_visible(
        &self,
        child: &Element,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "ScrollBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ensure_element_is_visible_ScrollBoxObject(
                self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                child: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ensure_element_is_visible_ScrollBoxObject(
            self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            child: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(child);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let child = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(child);
                __widl_f_ensure_element_is_visible_ScrollBoxObject(self_, child)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ScrollBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_by_ScrollBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&ScrollBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollBoxObject {
    #[cfg(all(feature = "ScrollBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `scrollBy()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollBy)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    #[allow(clippy::all)]
    pub fn scroll_by(&self, dx: i32, dy: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ScrollBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_by_ScrollBoxObject(
                self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_by_ScrollBoxObject(
            self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(dx);
            drop(dy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let dx = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                __widl_f_scroll_by_ScrollBoxObject(self_, dx, dy)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ScrollBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_by_index_ScrollBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ScrollBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollBoxObject {
    #[cfg(all(feature = "ScrollBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `scrollByIndex()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollByIndex)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    #[allow(clippy::all)]
    pub fn scroll_by_index(&self, dindexes: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ScrollBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_by_index_ScrollBoxObject(
                self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dindexes: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_by_index_ScrollBoxObject(
            self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dindexes: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(dindexes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let dindexes = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dindexes);
                __widl_f_scroll_by_index_ScrollBoxObject(self_, dindexes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ScrollBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_to_ScrollBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&ScrollBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollBoxObject {
    #[cfg(all(feature = "ScrollBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `scrollTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollTo)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    #[allow(clippy::all)]
    pub fn scroll_to(&self, x: i32, y: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ScrollBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_to_ScrollBoxObject(
                self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_to_ScrollBoxObject(
            self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_scroll_to_ScrollBoxObject(self_, x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "ScrollBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_to_element_ScrollBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ScrollBoxObject as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollBoxObject {
    #[cfg(all(feature = "Element", feature = "ScrollBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `scrollToElement()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollToElement)\n\n*This API requires the following crate features to be activated: `Element`, `ScrollBoxObject`*"]
    #[allow(clippy::all)]
    pub fn scroll_to_element(&self, child: &Element) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "ScrollBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_to_element_ScrollBoxObject(
                self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                child: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_to_element_ScrollBoxObject(
            self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            child: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(child);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let child = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(child);
                __widl_f_scroll_to_element_ScrollBoxObject(self_, child)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ScrollBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_position_x_ScrollBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScrollBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl ScrollBoxObject {
    #[cfg(all(feature = "ScrollBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `positionX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/positionX)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    #[allow(clippy::all)]
    pub fn position_x(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ScrollBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_position_x_ScrollBoxObject(
                self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_position_x_ScrollBoxObject(
            self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_position_x_ScrollBoxObject(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ScrollBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_position_y_ScrollBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScrollBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl ScrollBoxObject {
    #[cfg(all(feature = "ScrollBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `positionY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/positionY)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    #[allow(clippy::all)]
    pub fn position_y(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ScrollBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_position_y_ScrollBoxObject(
                self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_position_y_ScrollBoxObject(
            self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_position_y_ScrollBoxObject(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ScrollBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scrolled_width_ScrollBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScrollBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl ScrollBoxObject {
    #[cfg(all(feature = "ScrollBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `scrolledWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrolledWidth)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    #[allow(clippy::all)]
    pub fn scrolled_width(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ScrollBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scrolled_width_ScrollBoxObject(
                self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scrolled_width_ScrollBoxObject(
            self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scrolled_width_ScrollBoxObject(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ScrollBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scrolled_height_ScrollBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScrollBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl ScrollBoxObject {
    #[cfg(all(feature = "ScrollBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `scrolledHeight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrolledHeight)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    #[allow(clippy::all)]
    pub fn scrolled_height(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ScrollBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scrolled_height_ScrollBoxObject(
                self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scrolled_height_ScrollBoxObject(
            self_: <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scrolled_height_ScrollBoxObject(self_)
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
pub static __WASM_BINDGEN_GENERATED_4a955cf963ae68d8: [u8; 1023usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xBD\x03\0\0\0\0\n\0\0\x02\x0FScrollBoxObject!__widl_instanceof_ScrollBoxObject\0\0\0\02__widl_f_ensure_element_is_visible_ScrollBoxObject\x01\0\0\x01\x0FScrollBoxObject\x01\0\0\x01\x02\x05self_\x05child\x16ensureElementIsVisible\0\0\0\"__widl_f_scroll_by_ScrollBoxObject\x01\0\0\x01\x0FScrollBoxObject\x01\0\0\x01\x03\x05self_\x02dx\x02dy\x08scrollBy\0\0\0(__widl_f_scroll_by_index_ScrollBoxObject\x01\0\0\x01\x0FScrollBoxObject\x01\0\0\x01\x02\x05self_\x08dindexes\rscrollByIndex\0\0\0\"__widl_f_scroll_to_ScrollBoxObject\x01\0\0\x01\x0FScrollBoxObject\x01\0\0\x01\x03\x05self_\x01x\x01y\x08scrollTo\0\0\0*__widl_f_scroll_to_element_ScrollBoxObject\x01\0\0\x01\x0FScrollBoxObject\x01\0\0\x01\x02\x05self_\x05child\x0FscrollToElement\0\0\0#__widl_f_position_x_ScrollBoxObject\x01\0\0\x01\x0FScrollBoxObject\x01\0\x01\tpositionX\x01\x01\x05self_\tpositionX\0\0\0#__widl_f_position_y_ScrollBoxObject\x01\0\0\x01\x0FScrollBoxObject\x01\0\x01\tpositionY\x01\x01\x05self_\tpositionY\0\0\0'__widl_f_scrolled_width_ScrollBoxObject\x01\0\0\x01\x0FScrollBoxObject\x01\0\x01\rscrolledWidth\x01\x01\x05self_\rscrolledWidth\0\0\0(__widl_f_scrolled_height_ScrollBoxObject\x01\0\0\x01\x0FScrollBoxObject\x01\0\x01\x0EscrolledHeight\x01\x01\x05self_\x0EscrolledHeight\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
