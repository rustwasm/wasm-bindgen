use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Screen` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen)\n\n*This API requires the following crate features to be activated: `Screen`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Screen {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Screen: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Screen {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(6u32);
            inform(83u32);
            inform(99u32);
            inform(114u32);
            inform(101u32);
            inform(101u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for Screen {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for Screen {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Screen {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Screen {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Screen {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Screen {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Screen {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Screen {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Screen {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Screen>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Screen {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Screen {
        #[inline]
        fn from(obj: JsValue) -> Screen {
            Screen { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Screen {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Screen> for Screen {
        #[inline]
        fn as_ref(&self) -> &Screen {
            self
        }
    }
    impl From<Screen> for JsValue {
        #[inline]
        fn from(obj: Screen) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Screen {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Screen(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Screen(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Screen(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Screen { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Screen) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Screen> for EventTarget {
    #[inline]
    fn from(obj: Screen) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for Screen {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Screen> for ::js_sys::Object {
    #[inline]
    fn from(obj: Screen) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Screen {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Screen",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_avail_width_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen",))]
    #[allow(bad_style)]
    #[doc = "The `availWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availWidth)\n\n*This API requires the following crate features to be activated: `Screen`*"]
    #[allow(clippy::all)]
    pub fn avail_width(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Screen",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_avail_width_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_avail_width_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_avail_width_Screen(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Screen",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_avail_height_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen",))]
    #[allow(bad_style)]
    #[doc = "The `availHeight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availHeight)\n\n*This API requires the following crate features to be activated: `Screen`*"]
    #[allow(clippy::all)]
    pub fn avail_height(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Screen",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_avail_height_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_avail_height_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_avail_height_Screen(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Screen",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/width)\n\n*This API requires the following crate features to be activated: `Screen`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Screen",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_Screen(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Screen",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/height)\n\n*This API requires the following crate features to be activated: `Screen`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Screen",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_Screen(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Screen",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_color_depth_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen",))]
    #[allow(bad_style)]
    #[doc = "The `colorDepth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/colorDepth)\n\n*This API requires the following crate features to be activated: `Screen`*"]
    #[allow(clippy::all)]
    pub fn color_depth(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Screen",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_color_depth_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_color_depth_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_color_depth_Screen(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Screen",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pixel_depth_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen",))]
    #[allow(bad_style)]
    #[doc = "The `pixelDepth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/pixelDepth)\n\n*This API requires the following crate features to be activated: `Screen`*"]
    #[allow(clippy::all)]
    pub fn pixel_depth(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Screen",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pixel_depth_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pixel_depth_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pixel_depth_Screen(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Screen",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_top_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen",))]
    #[allow(bad_style)]
    #[doc = "The `top` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/top)\n\n*This API requires the following crate features to be activated: `Screen`*"]
    #[allow(clippy::all)]
    pub fn top(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Screen",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_top_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_top_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_top_Screen(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Screen",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_left_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen",))]
    #[allow(bad_style)]
    #[doc = "The `left` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/left)\n\n*This API requires the following crate features to be activated: `Screen`*"]
    #[allow(clippy::all)]
    pub fn left(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Screen",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_left_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_left_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_left_Screen(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Screen",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_avail_top_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen",))]
    #[allow(bad_style)]
    #[doc = "The `availTop` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availTop)\n\n*This API requires the following crate features to be activated: `Screen`*"]
    #[allow(clippy::all)]
    pub fn avail_top(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Screen",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_avail_top_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_avail_top_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_avail_top_Screen(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Screen",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_avail_left_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen",))]
    #[allow(bad_style)]
    #[doc = "The `availLeft` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availLeft)\n\n*This API requires the following crate features to be activated: `Screen`*"]
    #[allow(clippy::all)]
    pub fn avail_left(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Screen",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_avail_left_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_avail_left_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_avail_left_Screen(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Screen", feature = "ScreenOrientation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_orientation_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <ScreenOrientation as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen", feature = "ScreenOrientation",))]
    #[allow(bad_style)]
    #[doc = "The `orientation` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/orientation)\n\n*This API requires the following crate features to be activated: `Screen`, `ScreenOrientation`*"]
    #[allow(clippy::all)]
    pub fn orientation(&self) -> ScreenOrientation {
        #[cfg(all(feature = "Screen", feature = "ScreenOrientation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_orientation_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ScreenOrientation as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_orientation_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ScreenOrientation as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_orientation_Screen(self_)
            };
            <ScreenOrientation as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Screen", feature = "ScreenColorGamut",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_color_gamut_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <ScreenColorGamut as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen", feature = "ScreenColorGamut",))]
    #[allow(bad_style)]
    #[doc = "The `colorGamut` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/colorGamut)\n\n*This API requires the following crate features to be activated: `Screen`, `ScreenColorGamut`*"]
    #[allow(clippy::all)]
    pub fn color_gamut(&self) -> ScreenColorGamut {
        #[cfg(all(feature = "Screen", feature = "ScreenColorGamut",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_color_gamut_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ScreenColorGamut as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_color_gamut_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ScreenColorGamut as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_color_gamut_Screen(self_)
            };
            <ScreenColorGamut as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Screen", feature = "ScreenLuminance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_luminance_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <Option<ScreenLuminance> as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen", feature = "ScreenLuminance",))]
    #[allow(bad_style)]
    #[doc = "The `luminance` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/luminance)\n\n*This API requires the following crate features to be activated: `Screen`, `ScreenLuminance`*"]
    #[allow(clippy::all)]
    pub fn luminance(&self) -> Option<ScreenLuminance> {
        #[cfg(all(feature = "Screen", feature = "ScreenLuminance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_luminance_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<ScreenLuminance> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_luminance_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<ScreenLuminance> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_luminance_Screen(self_)
            };
            <Option<ScreenLuminance> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Screen",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onchange_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Screen as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/onchange)\n\n*This API requires the following crate features to be activated: `Screen`*"]
    #[allow(clippy::all)]
    pub fn onchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Screen",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onchange_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onchange_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onchange_Screen(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Screen",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onchange_Screen() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Screen as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Screen {
    #[cfg(all(feature = "Screen",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/onchange)\n\n*This API requires the following crate features to be activated: `Screen`*"]
    #[allow(clippy::all)]
    pub fn set_onchange(&self, onchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Screen",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onchange_Screen(
                self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onchange_Screen(
            self_: <&Screen as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Screen as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onchange,
                    );
                __widl_f_set_onchange_Screen(self_, onchange)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_aec94949d81d4c48: [u8; 1187usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}a\x04\0\0\0\0\x10\0\0\x02\x06Screen\x18__widl_instanceof_Screen\0\0\0\0\x1B__widl_f_avail_width_Screen\x01\0\0\x01\x06Screen\x01\0\x01\navailWidth\x01\x01\x05self_\navailWidth\0\0\0\x1C__widl_f_avail_height_Screen\x01\0\0\x01\x06Screen\x01\0\x01\x0BavailHeight\x01\x01\x05self_\x0BavailHeight\0\0\0\x15__widl_f_width_Screen\x01\0\0\x01\x06Screen\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0\x16__widl_f_height_Screen\x01\0\0\x01\x06Screen\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0\x1B__widl_f_color_depth_Screen\x01\0\0\x01\x06Screen\x01\0\x01\ncolorDepth\x01\x01\x05self_\ncolorDepth\0\0\0\x1B__widl_f_pixel_depth_Screen\x01\0\0\x01\x06Screen\x01\0\x01\npixelDepth\x01\x01\x05self_\npixelDepth\0\0\0\x13__widl_f_top_Screen\x01\0\0\x01\x06Screen\x01\0\x01\x03top\x01\x01\x05self_\x03top\0\0\0\x14__widl_f_left_Screen\x01\0\0\x01\x06Screen\x01\0\x01\x04left\x01\x01\x05self_\x04left\0\0\0\x19__widl_f_avail_top_Screen\x01\0\0\x01\x06Screen\x01\0\x01\x08availTop\x01\x01\x05self_\x08availTop\0\0\0\x1A__widl_f_avail_left_Screen\x01\0\0\x01\x06Screen\x01\0\x01\tavailLeft\x01\x01\x05self_\tavailLeft\0\0\0\x1B__widl_f_orientation_Screen\0\0\0\x01\x06Screen\x01\0\x01\x0Borientation\x01\x01\x05self_\x0Borientation\0\0\0\x1B__widl_f_color_gamut_Screen\0\0\0\x01\x06Screen\x01\0\x01\ncolorGamut\x01\x01\x05self_\ncolorGamut\0\0\0\x19__widl_f_luminance_Screen\0\0\0\x01\x06Screen\x01\0\x01\tluminance\x01\x01\x05self_\tluminance\0\0\0\x18__widl_f_onchange_Screen\0\0\0\x01\x06Screen\x01\0\x01\x08onchange\x01\x01\x05self_\x08onchange\0\0\0\x1C__widl_f_set_onchange_Screen\0\0\0\x01\x06Screen\x01\0\x02\x08onchange\x01\x02\x05self_\x08onchange\x08onchange\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
