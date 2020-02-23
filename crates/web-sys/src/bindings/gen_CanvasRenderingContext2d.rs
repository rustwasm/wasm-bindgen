use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CanvasRenderingContext2D` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CanvasRenderingContext2d {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CanvasRenderingContext2d: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CanvasRenderingContext2d {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(24u32);
            inform(67u32);
            inform(97u32);
            inform(110u32);
            inform(118u32);
            inform(97u32);
            inform(115u32);
            inform(82u32);
            inform(101u32);
            inform(110u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
            inform(105u32);
            inform(110u32);
            inform(103u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(50u32);
            inform(68u32);
        }
    }
    impl core::ops::Deref for CanvasRenderingContext2d {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for CanvasRenderingContext2d {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CanvasRenderingContext2d {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CanvasRenderingContext2d {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CanvasRenderingContext2d {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CanvasRenderingContext2d {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CanvasRenderingContext2d {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CanvasRenderingContext2d {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CanvasRenderingContext2d {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CanvasRenderingContext2d>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CanvasRenderingContext2d {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CanvasRenderingContext2d {
        #[inline]
        fn from(obj: JsValue) -> CanvasRenderingContext2d {
            CanvasRenderingContext2d { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CanvasRenderingContext2d {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CanvasRenderingContext2d> for CanvasRenderingContext2d {
        #[inline]
        fn as_ref(&self) -> &CanvasRenderingContext2d {
            self
        }
    }
    impl From<CanvasRenderingContext2d> for JsValue {
        #[inline]
        fn from(obj: CanvasRenderingContext2d) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CanvasRenderingContext2d {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CanvasRenderingContext2D(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CanvasRenderingContext2D(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CanvasRenderingContext2D(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CanvasRenderingContext2d { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CanvasRenderingContext2d) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CanvasRenderingContext2d> for ::js_sys::Object {
    #[inline]
    fn from(obj: CanvasRenderingContext2d) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CanvasRenderingContext2d {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_window_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&Window as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `drawWindow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*"]
    #[allow(clippy::all)]
    pub fn draw_window(
        &self,
        window: &Window,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        bg_color: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_window_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                window: <&Window as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_window_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            window: <&Window as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(window);
            drop(x);
            drop(y);
            drop(w);
            drop(h);
            drop(bg_color);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let window = <&Window as wasm_bindgen::convert::IntoWasmAbi>::into_abi(window);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let w = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                let h = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(h);
                let bg_color = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bg_color);
                __widl_f_draw_window_CanvasRenderingContext2D(self_, window, x, y, w, h, bg_color)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_window_with_flags_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&Window as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `drawWindow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*"]
    #[allow(clippy::all)]
    pub fn draw_window_with_flags(
        &self,
        window: &Window,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        bg_color: &str,
        flags: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_window_with_flags_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                window: <&Window as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                flags: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_window_with_flags_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            window: <&Window as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            flags: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(window);
            drop(x);
            drop(y);
            drop(w);
            drop(h);
            drop(bg_color);
            drop(flags);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let window = <&Window as wasm_bindgen::convert::IntoWasmAbi>::into_abi(window);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let w = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                let h = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(h);
                let bg_color = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bg_color);
                let flags = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(flags);
                __widl_f_draw_window_with_flags_CanvasRenderingContext2D(
                    self_, window, x, y, w, h, bg_color, flags,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_canvas_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <Option<HtmlCanvasElement> as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `canvas` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/canvas)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn canvas(&self) -> Option<HtmlCanvasElement> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_canvas_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlCanvasElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_canvas_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<HtmlCanvasElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_canvas_CanvasRenderingContext2D(self_)
            };
            <Option<HtmlCanvasElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_global_alpha_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `globalAlpha` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn global_alpha(&self) -> f64 {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_global_alpha_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_global_alpha_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_global_alpha_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_global_alpha_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `globalAlpha` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_global_alpha(&self, global_alpha: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_global_alpha_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                global_alpha: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_global_alpha_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            global_alpha: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(global_alpha);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let global_alpha =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(global_alpha);
                __widl_f_set_global_alpha_CanvasRenderingContext2D(self_, global_alpha)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_global_composite_operation_CanvasRenderingContext2D()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `globalCompositeOperation` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn global_composite_operation(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_global_composite_operation_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_global_composite_operation_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_global_composite_operation_CanvasRenderingContext2D(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_global_composite_operation_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `globalCompositeOperation` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_global_composite_operation(
        &self,
        global_composite_operation: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_global_composite_operation_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                global_composite_operation: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_global_composite_operation_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            global_composite_operation: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(global_composite_operation);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let global_composite_operation =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        global_composite_operation,
                    );
                __widl_f_set_global_composite_operation_CanvasRenderingContext2D(
                    self_,
                    global_composite_operation,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_html_image_element_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&HtmlImageElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_html_image_element(
        &self,
        image: &HtmlImageElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_html_image_element_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_html_image_element_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                __widl_f_draw_image_with_html_image_element_CanvasRenderingContext2D(
                    self_, image, dx, dy,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "SvgImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_svg_image_element_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&SvgImageElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "SvgImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_svg_image_element(
        &self,
        image: &SvgImageElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "SvgImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_svg_image_element_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&SvgImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_svg_image_element_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&SvgImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&SvgImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                __widl_f_draw_image_with_svg_image_element_CanvasRenderingContext2D(
                    self_, image, dx, dy,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_html_canvas_element_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_html_canvas_element(
        &self,
        image: &HtmlCanvasElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_html_canvas_element_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_html_canvas_element_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                __widl_f_draw_image_with_html_canvas_element_CanvasRenderingContext2D(
                    self_, image, dx, dy,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlVideoElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_html_video_element_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&HtmlVideoElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlVideoElement",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_html_video_element(
        &self,
        image: &HtmlVideoElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlVideoElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_html_video_element_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_html_video_element_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                __widl_f_draw_image_with_html_video_element_CanvasRenderingContext2D(
                    self_, image, dx, dy,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageBitmap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_image_bitmap_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&ImageBitmap as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageBitmap",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_image_bitmap(
        &self,
        image: &ImageBitmap,
        dx: f64,
        dy: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageBitmap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_image_bitmap_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_image_bitmap_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                __widl_f_draw_image_with_image_bitmap_CanvasRenderingContext2D(self_, image, dx, dy)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_html_image_element_and_dw_and_dh_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&HtmlImageElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_html_image_element_and_dw_and_dh(
        &self,
        image: &HtmlImageElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_html_image_element_and_dw_and_dh_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_html_image_element_and_dw_and_dh_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
            drop(dx);
            drop(dy);
            drop(dw);
            drop(dh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                let dw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dw);
                let dh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dh);
                __widl_f_draw_image_with_html_image_element_and_dw_and_dh_CanvasRenderingContext2D(
                    self_, image, dx, dy, dw, dh,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "SvgImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_svg_image_element_and_dw_and_dh_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&SvgImageElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "SvgImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_svg_image_element_and_dw_and_dh(
        &self,
        image: &SvgImageElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "SvgImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_svg_image_element_and_dw_and_dh_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&SvgImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_svg_image_element_and_dw_and_dh_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&SvgImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
            drop(dx);
            drop(dy);
            drop(dw);
            drop(dh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&SvgImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                let dw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dw);
                let dh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dh);
                __widl_f_draw_image_with_svg_image_element_and_dw_and_dh_CanvasRenderingContext2D(
                    self_, image, dx, dy, dw, dh,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_html_canvas_element_and_dw_and_dh_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_html_canvas_element_and_dw_and_dh(
        &self,
        image: &HtmlCanvasElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_html_canvas_element_and_dw_and_dh_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_html_canvas_element_and_dw_and_dh_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
            drop(dx);
            drop(dy);
            drop(dw);
            drop(dh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                let dw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dw);
                let dh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dh);
                __widl_f_draw_image_with_html_canvas_element_and_dw_and_dh_CanvasRenderingContext2D(
                    self_, image, dx, dy, dw, dh,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlVideoElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_html_video_element_and_dw_and_dh_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&HtmlVideoElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlVideoElement",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_html_video_element_and_dw_and_dh(
        &self,
        image: &HtmlVideoElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlVideoElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_html_video_element_and_dw_and_dh_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_html_video_element_and_dw_and_dh_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
            drop(dx);
            drop(dy);
            drop(dw);
            drop(dh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                let dw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dw);
                let dh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dh);
                __widl_f_draw_image_with_html_video_element_and_dw_and_dh_CanvasRenderingContext2D(
                    self_, image, dx, dy, dw, dh,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageBitmap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_image_bitmap_and_dw_and_dh_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&ImageBitmap as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageBitmap",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_image_bitmap_and_dw_and_dh(
        &self,
        image: &ImageBitmap,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageBitmap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_image_bitmap_and_dw_and_dh_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_image_bitmap_and_dw_and_dh_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
            drop(dx);
            drop(dy);
            drop(dw);
            drop(dh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                let dw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dw);
                let dh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dh);
                __widl_f_draw_image_with_image_bitmap_and_dw_and_dh_CanvasRenderingContext2D(
                    self_, image, dx, dy, dw, dh,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&HtmlImageElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        &self,
        image: &HtmlImageElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
            drop(sx);
            drop(sy);
            drop(sw);
            drop(sh);
            drop(dx);
            drop(dy);
            drop(dw);
            drop(dh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let sx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sx);
                let sy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sy);
                let sw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                let sh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sh);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                let dw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dw);
                let dh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dh);
                __widl_f_draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D ( self_ , image , sx , sy , sw , sh , dx , dy , dw , dh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "SvgImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_svg_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&SvgImageElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "SvgImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_svg_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        &self,
        image: &SvgImageElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "SvgImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_svg_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&SvgImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_svg_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&SvgImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
            drop(sx);
            drop(sy);
            drop(sw);
            drop(sh);
            drop(dx);
            drop(dy);
            drop(dw);
            drop(dh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&SvgImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let sx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sx);
                let sy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sy);
                let sw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                let sh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sh);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                let dw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dw);
                let dh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dh);
                __widl_f_draw_image_with_svg_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D ( self_ , image , sx , sy , sw , sh , dx , dy , dw , dh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        &self,
        image: &HtmlCanvasElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
            drop(sx);
            drop(sy);
            drop(sw);
            drop(sh);
            drop(dx);
            drop(dy);
            drop(dw);
            drop(dh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let sx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sx);
                let sy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sy);
                let sw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                let sh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sh);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                let dw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dw);
                let dh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dh);
                __widl_f_draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D ( self_ , image , sx , sy , sw , sh , dx , dy , dw , dh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlVideoElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&HtmlVideoElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlVideoElement",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        &self,
        image: &HtmlVideoElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HtmlVideoElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
            drop(sx);
            drop(sy);
            drop(sw);
            drop(sh);
            drop(dx);
            drop(dy);
            drop(dw);
            drop(dh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let sx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sx);
                let sy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sy);
                let sw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                let sh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sh);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                let dw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dw);
                let dh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dh);
                __widl_f_draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D ( self_ , image , sx , sy , sw , sh , dx , dy , dw , dh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageBitmap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&ImageBitmap as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageBitmap",))]
    #[allow(bad_style)]
    #[doc = "The `drawImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*"]
    #[allow(clippy::all)]
    pub fn draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        &self,
        image: &ImageBitmap,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageBitmap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
            drop(sx);
            drop(sy);
            drop(sw);
            drop(sh);
            drop(dx);
            drop(dy);
            drop(dw);
            drop(dh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let sx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sx);
                let sy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sy);
                let sw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                let sh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sh);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                let dw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dw);
                let dh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dh);
                __widl_f_draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D ( self_ , image , sx , sy , sw , sh , dx , dy , dw , dh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_begin_path_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `beginPath()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/beginPath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn begin_path(&self) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_begin_path_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_begin_path_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_begin_path_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clip_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `clip()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn clip(&self) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clip_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clip_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_clip_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "CanvasWindingRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clip_with_canvas_winding_rule_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <CanvasWindingRule as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "CanvasWindingRule",))]
    #[allow(bad_style)]
    #[doc = "The `clip()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*"]
    #[allow(clippy::all)]
    pub fn clip_with_canvas_winding_rule(&self, winding: CanvasWindingRule) {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "CanvasWindingRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clip_with_canvas_winding_rule_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                winding: <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clip_with_canvas_winding_rule_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            winding: <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(winding);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let winding =
                    <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(winding);
                __widl_f_clip_with_canvas_winding_rule_CanvasRenderingContext2D(self_, winding)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clip_with_path_2d_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&Path2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `clip()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    #[allow(clippy::all)]
    pub fn clip_with_path_2d(&self, path: &Path2d) {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clip_with_path_2d_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clip_with_path_2d_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                __widl_f_clip_with_path_2d_CanvasRenderingContext2D(self_, path)
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "CanvasRenderingContext2d",
    feature = "CanvasWindingRule",
    feature = "Path2d",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clip_with_path_2d_and_winding_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&Path2d as WasmDescribe>::describe();
    <CanvasWindingRule as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(
        feature = "CanvasRenderingContext2d",
        feature = "CanvasWindingRule",
        feature = "Path2d",
    ))]
    #[allow(bad_style)]
    #[doc = "The `clip()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*"]
    #[allow(clippy::all)]
    pub fn clip_with_path_2d_and_winding(&self, path: &Path2d, winding: CanvasWindingRule) {
        #[cfg(all(
            feature = "CanvasRenderingContext2d",
            feature = "CanvasWindingRule",
            feature = "Path2d",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clip_with_path_2d_and_winding_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                winding: <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clip_with_path_2d_and_winding_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            winding: <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(winding);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let winding =
                    <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(winding);
                __widl_f_clip_with_path_2d_and_winding_CanvasRenderingContext2D(
                    self_, path, winding,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fill_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `fill()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn fill(&self) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fill_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fill_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_fill_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "CanvasWindingRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fill_with_canvas_winding_rule_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <CanvasWindingRule as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "CanvasWindingRule",))]
    #[allow(bad_style)]
    #[doc = "The `fill()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*"]
    #[allow(clippy::all)]
    pub fn fill_with_canvas_winding_rule(&self, winding: CanvasWindingRule) {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "CanvasWindingRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fill_with_canvas_winding_rule_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                winding: <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fill_with_canvas_winding_rule_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            winding: <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(winding);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let winding =
                    <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(winding);
                __widl_f_fill_with_canvas_winding_rule_CanvasRenderingContext2D(self_, winding)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fill_with_path_2d_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&Path2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `fill()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    #[allow(clippy::all)]
    pub fn fill_with_path_2d(&self, path: &Path2d) {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fill_with_path_2d_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fill_with_path_2d_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                __widl_f_fill_with_path_2d_CanvasRenderingContext2D(self_, path)
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "CanvasRenderingContext2d",
    feature = "CanvasWindingRule",
    feature = "Path2d",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fill_with_path_2d_and_winding_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&Path2d as WasmDescribe>::describe();
    <CanvasWindingRule as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(
        feature = "CanvasRenderingContext2d",
        feature = "CanvasWindingRule",
        feature = "Path2d",
    ))]
    #[allow(bad_style)]
    #[doc = "The `fill()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*"]
    #[allow(clippy::all)]
    pub fn fill_with_path_2d_and_winding(&self, path: &Path2d, winding: CanvasWindingRule) {
        #[cfg(all(
            feature = "CanvasRenderingContext2d",
            feature = "CanvasWindingRule",
            feature = "Path2d",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fill_with_path_2d_and_winding_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                winding: <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fill_with_path_2d_and_winding_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            winding: <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(winding);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let winding =
                    <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(winding);
                __widl_f_fill_with_path_2d_and_winding_CanvasRenderingContext2D(
                    self_, path, winding,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_point_in_path_with_f64_CanvasRenderingContext2D()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `isPointInPath()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn is_point_in_path_with_f64(&self, x: f64, y: f64) -> bool {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_point_in_path_with_f64_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_point_in_path_with_f64_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_is_point_in_path_with_f64_CanvasRenderingContext2D(self_, x, y)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "CanvasWindingRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_point_in_path_with_f64_and_canvas_winding_rule_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <CanvasWindingRule as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "CanvasWindingRule",))]
    #[allow(bad_style)]
    #[doc = "The `isPointInPath()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*"]
    #[allow(clippy::all)]
    pub fn is_point_in_path_with_f64_and_canvas_winding_rule(
        &self,
        x: f64,
        y: f64,
        winding: CanvasWindingRule,
    ) -> bool {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "CanvasWindingRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_point_in_path_with_f64_and_canvas_winding_rule_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                winding: <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_point_in_path_with_f64_and_canvas_winding_rule_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            winding: <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x);
            drop(y);
            drop(winding);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let winding =
                    <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(winding);
                __widl_f_is_point_in_path_with_f64_and_canvas_winding_rule_CanvasRenderingContext2D(
                    self_, x, y, winding,
                )
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_point_in_path_with_path_2d_and_f64_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&Path2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `isPointInPath()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    #[allow(clippy::all)]
    pub fn is_point_in_path_with_path_2d_and_f64(&self, path: &Path2d, x: f64, y: f64) -> bool {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_point_in_path_with_path_2d_and_f64_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_point_in_path_with_path_2d_and_f64_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(path);
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_is_point_in_path_with_path_2d_and_f64_CanvasRenderingContext2D(
                    self_, path, x, y,
                )
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "CanvasRenderingContext2d",
    feature = "CanvasWindingRule",
    feature = "Path2d",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_point_in_path_with_path_2d_and_f64_and_winding_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&Path2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <CanvasWindingRule as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(
        feature = "CanvasRenderingContext2d",
        feature = "CanvasWindingRule",
        feature = "Path2d",
    ))]
    #[allow(bad_style)]
    #[doc = "The `isPointInPath()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*"]
    #[allow(clippy::all)]
    pub fn is_point_in_path_with_path_2d_and_f64_and_winding(
        &self,
        path: &Path2d,
        x: f64,
        y: f64,
        winding: CanvasWindingRule,
    ) -> bool {
        #[cfg(all(
            feature = "CanvasRenderingContext2d",
            feature = "CanvasWindingRule",
            feature = "Path2d",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_point_in_path_with_path_2d_and_f64_and_winding_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                winding: <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_point_in_path_with_path_2d_and_f64_and_winding_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            winding: <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(path);
            drop(x);
            drop(y);
            drop(winding);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let winding =
                    <CanvasWindingRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(winding);
                __widl_f_is_point_in_path_with_path_2d_and_f64_and_winding_CanvasRenderingContext2D(
                    self_, path, x, y, winding,
                )
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_point_in_stroke_with_x_and_y_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `isPointInStroke()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn is_point_in_stroke_with_x_and_y(&self, x: f64, y: f64) -> bool {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_point_in_stroke_with_x_and_y_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_point_in_stroke_with_x_and_y_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_is_point_in_stroke_with_x_and_y_CanvasRenderingContext2D(self_, x, y)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_point_in_stroke_with_path_and_x_and_y_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&Path2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `isPointInStroke()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    #[allow(clippy::all)]
    pub fn is_point_in_stroke_with_path_and_x_and_y(&self, path: &Path2d, x: f64, y: f64) -> bool {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_point_in_stroke_with_path_and_x_and_y_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_point_in_stroke_with_path_and_x_and_y_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(path);
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_is_point_in_stroke_with_path_and_x_and_y_CanvasRenderingContext2D(
                    self_, path, x, y,
                )
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stroke_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `stroke()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn stroke(&self) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stroke_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stroke_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_stroke_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stroke_with_path_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&Path2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `stroke()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    #[allow(clippy::all)]
    pub fn stroke_with_path(&self, path: &Path2d) {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stroke_with_path_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stroke_with_path_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                __widl_f_stroke_with_path_CanvasRenderingContext2D(self_, path)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasGradient", feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_linear_gradient_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <CanvasGradient as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasGradient", feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `createLinearGradient()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createLinearGradient)\n\n*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn create_linear_gradient(&self, x0: f64, y0: f64, x1: f64, y1: f64) -> CanvasGradient {
        #[cfg(all(feature = "CanvasGradient", feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_linear_gradient_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x0: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y0: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CanvasGradient as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_linear_gradient_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x0: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y0: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CanvasGradient as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x0);
            drop(y0);
            drop(x1);
            drop(y1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x0 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x0);
                let y0 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y0);
                let x1 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x1);
                let y1 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y1);
                __widl_f_create_linear_gradient_CanvasRenderingContext2D(self_, x0, y0, x1, y1)
            };
            <CanvasGradient as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "CanvasPattern",
    feature = "CanvasRenderingContext2d",
    feature = "HtmlImageElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_pattern_with_html_image_element_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&HtmlImageElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<CanvasPattern> as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(
        feature = "CanvasPattern",
        feature = "CanvasRenderingContext2d",
        feature = "HtmlImageElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createPattern()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)\n\n*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn create_pattern_with_html_image_element(
        &self,
        image: &HtmlImageElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "CanvasPattern",
            feature = "CanvasRenderingContext2d",
            feature = "HtmlImageElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_pattern_with_html_image_element_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                repetition: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_pattern_with_html_image_element_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            repetition: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(image);
            drop(repetition);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let repetition = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(repetition);
                __widl_f_create_pattern_with_html_image_element_CanvasRenderingContext2D(
                    self_, image, repetition,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "CanvasPattern",
    feature = "CanvasRenderingContext2d",
    feature = "SvgImageElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_pattern_with_svg_image_element_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&SvgImageElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<CanvasPattern> as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(
        feature = "CanvasPattern",
        feature = "CanvasRenderingContext2d",
        feature = "SvgImageElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createPattern()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)\n\n*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `SvgImageElement`*"]
    #[allow(clippy::all)]
    pub fn create_pattern_with_svg_image_element(
        &self,
        image: &SvgImageElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "CanvasPattern",
            feature = "CanvasRenderingContext2d",
            feature = "SvgImageElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_pattern_with_svg_image_element_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&SvgImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                repetition: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_pattern_with_svg_image_element_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&SvgImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            repetition: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(image);
            drop(repetition);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&SvgImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let repetition = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(repetition);
                __widl_f_create_pattern_with_svg_image_element_CanvasRenderingContext2D(
                    self_, image, repetition,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "CanvasPattern",
    feature = "CanvasRenderingContext2d",
    feature = "HtmlCanvasElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_pattern_with_html_canvas_element_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<CanvasPattern> as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(
        feature = "CanvasPattern",
        feature = "CanvasRenderingContext2d",
        feature = "HtmlCanvasElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createPattern()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)\n\n*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn create_pattern_with_html_canvas_element(
        &self,
        image: &HtmlCanvasElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "CanvasPattern",
            feature = "CanvasRenderingContext2d",
            feature = "HtmlCanvasElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_pattern_with_html_canvas_element_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                repetition: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_pattern_with_html_canvas_element_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            repetition: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(image);
            drop(repetition);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let repetition = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(repetition);
                __widl_f_create_pattern_with_html_canvas_element_CanvasRenderingContext2D(
                    self_, image, repetition,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "CanvasPattern",
    feature = "CanvasRenderingContext2d",
    feature = "HtmlVideoElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_pattern_with_html_video_element_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&HtmlVideoElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<CanvasPattern> as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(
        feature = "CanvasPattern",
        feature = "CanvasRenderingContext2d",
        feature = "HtmlVideoElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createPattern()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)\n\n*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
    #[allow(clippy::all)]
    pub fn create_pattern_with_html_video_element(
        &self,
        image: &HtmlVideoElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "CanvasPattern",
            feature = "CanvasRenderingContext2d",
            feature = "HtmlVideoElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_pattern_with_html_video_element_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                repetition: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_pattern_with_html_video_element_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            repetition: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(image);
            drop(repetition);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let repetition = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(repetition);
                __widl_f_create_pattern_with_html_video_element_CanvasRenderingContext2D(
                    self_, image, repetition,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "CanvasPattern",
    feature = "CanvasRenderingContext2d",
    feature = "ImageBitmap",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_pattern_with_image_bitmap_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&ImageBitmap as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<CanvasPattern> as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(
        feature = "CanvasPattern",
        feature = "CanvasRenderingContext2d",
        feature = "ImageBitmap",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createPattern()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)\n\n*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `ImageBitmap`*"]
    #[allow(clippy::all)]
    pub fn create_pattern_with_image_bitmap(
        &self,
        image: &ImageBitmap,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "CanvasPattern",
            feature = "CanvasRenderingContext2d",
            feature = "ImageBitmap",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_pattern_with_image_bitmap_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                repetition: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_pattern_with_image_bitmap_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            repetition: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(image);
            drop(repetition);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let repetition = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(repetition);
                __widl_f_create_pattern_with_image_bitmap_CanvasRenderingContext2D(
                    self_, image, repetition,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<CanvasPattern> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CanvasGradient", feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_radial_gradient_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <CanvasGradient as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasGradient", feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `createRadialGradient()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createRadialGradient)\n\n*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn create_radial_gradient(
        &self,
        x0: f64,
        y0: f64,
        r0: f64,
        x1: f64,
        y1: f64,
        r1: f64,
    ) -> Result<CanvasGradient, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasGradient", feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_radial_gradient_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x0: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y0: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                r0: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                r1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CanvasGradient as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_radial_gradient_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x0: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y0: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            r0: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            r1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CanvasGradient as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x0);
            drop(y0);
            drop(r0);
            drop(x1);
            drop(y1);
            drop(r1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x0 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x0);
                let y0 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y0);
                let r0 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(r0);
                let x1 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x1);
                let y1 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y1);
                let r1 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(r1);
                __widl_f_create_radial_gradient_CanvasRenderingContext2D(
                    self_, x0, y0, r0, x1, y1, r1,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<CanvasGradient as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stroke_style_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `strokeStyle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn stroke_style(&self) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stroke_style_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stroke_style_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_stroke_style_CanvasRenderingContext2D(self_)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_stroke_style_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `strokeStyle` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_stroke_style(&self, stroke_style: &::wasm_bindgen::JsValue) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_stroke_style_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stroke_style: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_stroke_style_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stroke_style: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(stroke_style);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let stroke_style =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        stroke_style,
                    );
                __widl_f_set_stroke_style_CanvasRenderingContext2D(self_, stroke_style)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fill_style_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `fillStyle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn fill_style(&self) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fill_style_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fill_style_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_fill_style_CanvasRenderingContext2D(self_)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_fill_style_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `fillStyle` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_fill_style(&self, fill_style: &::wasm_bindgen::JsValue) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_fill_style_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                fill_style: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_fill_style_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            fill_style: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(fill_style);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let fill_style =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        fill_style,
                    );
                __widl_f_set_fill_style_CanvasRenderingContext2D(self_, fill_style)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_filter_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `filter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn filter(&self) -> String {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_filter_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_filter_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_filter_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_filter_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `filter` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_filter(&self, filter: &str) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_filter_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                filter: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_filter_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            filter: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(filter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let filter = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(filter);
                __widl_f_set_filter_CanvasRenderingContext2D(self_, filter)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_hit_region_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `addHitRegion()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/addHitRegion)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn add_hit_region(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_hit_region_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_hit_region_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_add_hit_region_CanvasRenderingContext2D(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "HitRegionOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_hit_region_with_options_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&HitRegionOptions as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HitRegionOptions",))]
    #[allow(bad_style)]
    #[doc = "The `addHitRegion()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/addHitRegion)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HitRegionOptions`*"]
    #[allow(clippy::all)]
    pub fn add_hit_region_with_options(
        &self,
        options: &HitRegionOptions,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "HitRegionOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_hit_region_with_options_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&HitRegionOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_hit_region_with_options_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&HitRegionOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let options =
                    <&HitRegionOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_add_hit_region_with_options_CanvasRenderingContext2D(self_, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_hit_regions_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `clearHitRegions()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearHitRegions)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn clear_hit_regions(&self) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_hit_regions_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_hit_regions_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_clear_hit_regions_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_hit_region_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `removeHitRegion()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/removeHitRegion)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn remove_hit_region(&self, id: &str) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_hit_region_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_hit_region_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(id);
                __widl_f_remove_hit_region_CanvasRenderingContext2D(self_, id)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_data_with_sw_and_sh_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <ImageData as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
    #[allow(bad_style)]
    #[doc = "The `createImageData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    #[allow(clippy::all)]
    pub fn create_image_data_with_sw_and_sh(
        &self,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_data_with_sw_and_sh_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_data_with_sw_and_sh_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(sw);
            drop(sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let sw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                let sh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sh);
                __widl_f_create_image_data_with_sw_and_sh_CanvasRenderingContext2D(self_, sw, sh)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageData as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_data_with_imagedata_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&ImageData as WasmDescribe>::describe();
    <ImageData as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
    #[allow(bad_style)]
    #[doc = "The `createImageData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    #[allow(clippy::all)]
    pub fn create_image_data_with_imagedata(
        &self,
        imagedata: &ImageData,
    ) -> Result<ImageData, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_data_with_imagedata_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                imagedata: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_data_with_imagedata_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            imagedata: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(imagedata);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let imagedata =
                    <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(imagedata);
                __widl_f_create_image_data_with_imagedata_CanvasRenderingContext2D(self_, imagedata)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageData as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_image_data_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <ImageData as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
    #[allow(bad_style)]
    #[doc = "The `getImageData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    #[allow(clippy::all)]
    pub fn get_image_data(
        &self,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_image_data_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_image_data_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(sx);
            drop(sy);
            drop(sw);
            drop(sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let sx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sx);
                let sy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sy);
                let sw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                let sh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sh);
                __widl_f_get_image_data_CanvasRenderingContext2D(self_, sx, sy, sw, sh)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageData as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_put_image_data_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&ImageData as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
    #[allow(bad_style)]
    #[doc = "The `putImageData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    #[allow(clippy::all)]
    pub fn put_image_data(
        &self,
        imagedata: &ImageData,
        dx: f64,
        dy: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_put_image_data_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                imagedata: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_put_image_data_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            imagedata: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(imagedata);
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let imagedata =
                    <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(imagedata);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                __widl_f_put_image_data_CanvasRenderingContext2D(self_, imagedata, dx, dy)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&ImageData as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
    #[allow(bad_style)]
    #[doc = "The `putImageData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    #[allow(clippy::all)]
    pub fn put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(
        &self,
        imagedata: &ImageData,
        dx: f64,
        dy: f64,
        dirty_x: f64,
        dirty_y: f64,
        dirty_width: f64,
        dirty_height: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "ImageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                imagedata: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dirty_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dirty_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dirty_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dirty_height: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            imagedata: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dirty_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dirty_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dirty_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dirty_height: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(imagedata);
            drop(dx);
            drop(dy);
            drop(dirty_x);
            drop(dirty_y);
            drop(dirty_width);
            drop(dirty_height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let imagedata =
                    <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(imagedata);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                let dirty_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dirty_x);
                let dirty_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dirty_y);
                let dirty_width =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dirty_width);
                let dirty_height =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dirty_height);
                __widl_f_put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height_CanvasRenderingContext2D ( self_ , imagedata , dx , dy , dirty_x , dirty_y , dirty_width , dirty_height )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_image_smoothing_enabled_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `imageSmoothingEnabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn image_smoothing_enabled(&self) -> bool {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_image_smoothing_enabled_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_image_smoothing_enabled_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_image_smoothing_enabled_CanvasRenderingContext2D(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_image_smoothing_enabled_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `imageSmoothingEnabled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_image_smoothing_enabled(&self, image_smoothing_enabled: bool) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_image_smoothing_enabled_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image_smoothing_enabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_image_smoothing_enabled_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image_smoothing_enabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image_smoothing_enabled);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image_smoothing_enabled =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image_smoothing_enabled);
                __widl_f_set_image_smoothing_enabled_CanvasRenderingContext2D(
                    self_,
                    image_smoothing_enabled,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_line_dash_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `getLineDash()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getLineDash)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn get_line_dash(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_line_dash_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_line_dash_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_get_line_dash_CanvasRenderingContext2D(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_line_dash_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `setLineDash()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setLineDash)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_line_dash(
        &self,
        segments: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_line_dash_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                segments: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_line_dash_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            segments: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(segments);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let segments =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        segments,
                    );
                __widl_f_set_line_dash_CanvasRenderingContext2D(self_, segments)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_width_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `lineWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn line_width(&self) -> f64 {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_width_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_width_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_line_width_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_line_width_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `lineWidth` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_line_width(&self, line_width: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_line_width_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                line_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_line_width_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            line_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(line_width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let line_width = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(line_width);
                __widl_f_set_line_width_CanvasRenderingContext2D(self_, line_width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_cap_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `lineCap` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn line_cap(&self) -> String {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_cap_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_cap_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_line_cap_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_line_cap_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `lineCap` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_line_cap(&self, line_cap: &str) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_line_cap_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                line_cap: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_line_cap_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            line_cap: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(line_cap);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let line_cap = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(line_cap);
                __widl_f_set_line_cap_CanvasRenderingContext2D(self_, line_cap)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_join_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `lineJoin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn line_join(&self) -> String {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_join_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_join_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_line_join_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_line_join_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `lineJoin` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_line_join(&self, line_join: &str) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_line_join_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                line_join: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_line_join_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            line_join: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(line_join);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let line_join = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(line_join);
                __widl_f_set_line_join_CanvasRenderingContext2D(self_, line_join)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_miter_limit_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `miterLimit` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn miter_limit(&self) -> f64 {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_miter_limit_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_miter_limit_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_miter_limit_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_miter_limit_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `miterLimit` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_miter_limit(&self, miter_limit: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_miter_limit_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                miter_limit: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_miter_limit_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            miter_limit: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(miter_limit);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let miter_limit =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(miter_limit);
                __widl_f_set_miter_limit_CanvasRenderingContext2D(self_, miter_limit)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_dash_offset_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `lineDashOffset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn line_dash_offset(&self) -> f64 {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_dash_offset_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_dash_offset_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_line_dash_offset_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_line_dash_offset_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `lineDashOffset` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_line_dash_offset(&self, line_dash_offset: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_line_dash_offset_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                line_dash_offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_line_dash_offset_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            line_dash_offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(line_dash_offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let line_dash_offset =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(line_dash_offset);
                __widl_f_set_line_dash_offset_CanvasRenderingContext2D(self_, line_dash_offset)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_arc_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `arc()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn arc(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_arc_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_arc_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(radius);
            drop(start_angle);
            drop(end_angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let radius = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius);
                let start_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_angle);
                let end_angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_angle);
                __widl_f_arc_CanvasRenderingContext2D(self_, x, y, radius, start_angle, end_angle)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_arc_with_anticlockwise_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `arc()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn arc_with_anticlockwise(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_arc_with_anticlockwise_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                anticlockwise: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_arc_with_anticlockwise_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            anticlockwise: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(radius);
            drop(start_angle);
            drop(end_angle);
            drop(anticlockwise);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let radius = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius);
                let start_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_angle);
                let end_angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_angle);
                let anticlockwise =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(anticlockwise);
                __widl_f_arc_with_anticlockwise_CanvasRenderingContext2D(
                    self_,
                    x,
                    y,
                    radius,
                    start_angle,
                    end_angle,
                    anticlockwise,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_arc_to_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `arcTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arcTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn arc_to(
        &self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_arc_to_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x2: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y2: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_arc_to_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x2: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y2: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x1);
            drop(y1);
            drop(x2);
            drop(y2);
            drop(radius);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x1 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x1);
                let y1 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y1);
                let x2 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x2);
                let y2 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y2);
                let radius = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius);
                __widl_f_arc_to_CanvasRenderingContext2D(self_, x1, y1, x2, y2, radius)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bezier_curve_to_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `bezierCurveTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/bezierCurveTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bezier_curve_to_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cp1x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cp1y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cp2x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cp2y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bezier_curve_to_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cp1x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cp1y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cp2x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cp2y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cp1x);
            drop(cp1y);
            drop(cp2x);
            drop(cp2y);
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let cp1x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cp1x);
                let cp1y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cp1y);
                let cp2x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cp2x);
                let cp2y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cp2y);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_bezier_curve_to_CanvasRenderingContext2D(
                    self_, cp1x, cp1y, cp2x, cp2y, x, y,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_path_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `closePath()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/closePath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn close_path(&self) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_path_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_path_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_close_path_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ellipse_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `ellipse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn ellipse(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ellipse_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rotation: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ellipse_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rotation: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(radius_x);
            drop(radius_y);
            drop(rotation);
            drop(start_angle);
            drop(end_angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let radius_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius_x);
                let radius_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius_y);
                let rotation = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rotation);
                let start_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_angle);
                let end_angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_angle);
                __widl_f_ellipse_CanvasRenderingContext2D(
                    self_,
                    x,
                    y,
                    radius_x,
                    radius_y,
                    rotation,
                    start_angle,
                    end_angle,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ellipse_with_anticlockwise_CanvasRenderingContext2D()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `ellipse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn ellipse_with_anticlockwise(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ellipse_with_anticlockwise_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rotation: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                anticlockwise: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ellipse_with_anticlockwise_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rotation: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            anticlockwise: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(radius_x);
            drop(radius_y);
            drop(rotation);
            drop(start_angle);
            drop(end_angle);
            drop(anticlockwise);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let radius_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius_x);
                let radius_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius_y);
                let rotation = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rotation);
                let start_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_angle);
                let end_angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_angle);
                let anticlockwise =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(anticlockwise);
                __widl_f_ellipse_with_anticlockwise_CanvasRenderingContext2D(
                    self_,
                    x,
                    y,
                    radius_x,
                    radius_y,
                    rotation,
                    start_angle,
                    end_angle,
                    anticlockwise,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_to_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `lineTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn line_to(&self, x: f64, y: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_to_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_to_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_line_to_CanvasRenderingContext2D(self_, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_move_to_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `moveTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/moveTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn move_to(&self, x: f64, y: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_move_to_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_move_to_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_move_to_CanvasRenderingContext2D(self_, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_quadratic_curve_to_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `quadraticCurveTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/quadraticCurveTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_quadratic_curve_to_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cpx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cpy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_quadratic_curve_to_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cpx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cpy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cpx);
            drop(cpy);
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let cpx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cpx);
                let cpy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cpy);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_quadratic_curve_to_CanvasRenderingContext2D(self_, cpx, cpy, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rect_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `rect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rect)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rect_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rect_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(w);
            drop(h);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let w = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                let h = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(h);
                __widl_f_rect_CanvasRenderingContext2D(self_, x, y, w, h)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_rect_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `clearRect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearRect)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_rect_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_rect_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(w);
            drop(h);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let w = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                let h = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(h);
                __widl_f_clear_rect_CanvasRenderingContext2D(self_, x, y, w, h)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fill_rect_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `fillRect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fill_rect_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fill_rect_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(w);
            drop(h);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let w = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                let h = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(h);
                __widl_f_fill_rect_CanvasRenderingContext2D(self_, x, y, w, h)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stroke_rect_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `strokeRect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeRect)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stroke_rect_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stroke_rect_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(w);
            drop(h);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let w = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                let h = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(h);
                __widl_f_stroke_rect_CanvasRenderingContext2D(self_, x, y, w, h)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shadow_offset_x_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `shadowOffsetX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn shadow_offset_x(&self) -> f64 {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shadow_offset_x_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shadow_offset_x_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_shadow_offset_x_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_shadow_offset_x_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `shadowOffsetX` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_shadow_offset_x(&self, shadow_offset_x: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_shadow_offset_x_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shadow_offset_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_shadow_offset_x_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shadow_offset_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(shadow_offset_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let shadow_offset_x =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shadow_offset_x);
                __widl_f_set_shadow_offset_x_CanvasRenderingContext2D(self_, shadow_offset_x)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shadow_offset_y_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `shadowOffsetY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn shadow_offset_y(&self) -> f64 {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shadow_offset_y_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shadow_offset_y_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_shadow_offset_y_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_shadow_offset_y_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `shadowOffsetY` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_shadow_offset_y(&self, shadow_offset_y: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_shadow_offset_y_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shadow_offset_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_shadow_offset_y_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shadow_offset_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(shadow_offset_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let shadow_offset_y =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shadow_offset_y);
                __widl_f_set_shadow_offset_y_CanvasRenderingContext2D(self_, shadow_offset_y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shadow_blur_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `shadowBlur` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn shadow_blur(&self) -> f64 {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shadow_blur_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shadow_blur_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_shadow_blur_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_shadow_blur_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `shadowBlur` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_shadow_blur(&self, shadow_blur: f64) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_shadow_blur_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shadow_blur: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_shadow_blur_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shadow_blur: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(shadow_blur);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let shadow_blur =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shadow_blur);
                __widl_f_set_shadow_blur_CanvasRenderingContext2D(self_, shadow_blur)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shadow_color_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `shadowColor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn shadow_color(&self) -> String {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shadow_color_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shadow_color_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_shadow_color_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_shadow_color_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `shadowColor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_shadow_color(&self, shadow_color: &str) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_shadow_color_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shadow_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_shadow_color_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shadow_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(shadow_color);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let shadow_color =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shadow_color);
                __widl_f_set_shadow_color_CanvasRenderingContext2D(self_, shadow_color)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_restore_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `restore()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/restore)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn restore(&self) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_restore_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_restore_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_restore_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_save_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `save()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/save)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn save(&self) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_save_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_save_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_save_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fill_text_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `fillText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn fill_text(&self, text: &str, x: f64, y: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fill_text_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fill_text_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_fill_text_CanvasRenderingContext2D(self_, text, x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fill_text_with_max_width_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `fillText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn fill_text_with_max_width(
        &self,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fill_text_with_max_width_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fill_text_with_max_width_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
            drop(x);
            drop(y);
            drop(max_width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let max_width = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max_width);
                __widl_f_fill_text_with_max_width_CanvasRenderingContext2D(
                    self_, text, x, y, max_width,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "TextMetrics",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_measure_text_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <TextMetrics as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "TextMetrics",))]
    #[allow(bad_style)]
    #[doc = "The `measureText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/measureText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `TextMetrics`*"]
    #[allow(clippy::all)]
    pub fn measure_text(&self, text: &str) -> Result<TextMetrics, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "TextMetrics",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_measure_text_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TextMetrics as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_measure_text_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TextMetrics as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_measure_text_CanvasRenderingContext2D(self_, text)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TextMetrics as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stroke_text_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `strokeText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn stroke_text(&self, text: &str, x: f64, y: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stroke_text_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stroke_text_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_stroke_text_CanvasRenderingContext2D(self_, text, x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stroke_text_with_max_width_CanvasRenderingContext2D()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `strokeText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn stroke_text_with_max_width(
        &self,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stroke_text_with_max_width_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stroke_text_with_max_width_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
            drop(x);
            drop(y);
            drop(max_width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let max_width = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max_width);
                __widl_f_stroke_text_with_max_width_CanvasRenderingContext2D(
                    self_, text, x, y, max_width,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_font_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `font` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn font(&self) -> String {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_font_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_font_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_font_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_font_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `font` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_font(&self, font: &str) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_font_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                font: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_font_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            font: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let font = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(font);
                __widl_f_set_font_CanvasRenderingContext2D(self_, font)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_align_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `textAlign` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn text_align(&self) -> String {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_align_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_align_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_text_align_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_text_align_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `textAlign` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_text_align(&self, text_align: &str) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_text_align_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_text_align_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_align);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text_align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_align);
                __widl_f_set_text_align_CanvasRenderingContext2D(self_, text_align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_baseline_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `textBaseline` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn text_baseline(&self) -> String {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_baseline_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_baseline_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_text_baseline_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_text_baseline_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `textBaseline` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_text_baseline(&self, text_baseline: &str) {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_text_baseline_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_baseline: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_text_baseline_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_baseline: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_baseline);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text_baseline =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_baseline);
                __widl_f_set_text_baseline_CanvasRenderingContext2D(self_, text_baseline)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_transform_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `getTransform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getTransform)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn get_transform(&self) -> Result<DomMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_transform_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_transform_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_get_transform_CanvasRenderingContext2D(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reset_transform_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `resetTransform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/resetTransform)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn reset_transform(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reset_transform_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reset_transform_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_reset_transform_CanvasRenderingContext2D(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `rotate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rotate)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn rotate(&self, angle: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_rotate_CanvasRenderingContext2D(self_, angle)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `scale()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/scale)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn scale(&self, x: f64, y: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_scale_CanvasRenderingContext2D(self_, x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_transform_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `setTransform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_transform(
        &self,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_transform_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                b: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                c: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                d: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                e: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                f: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_transform_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            b: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            c: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            d: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            e: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            f: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a);
            drop(b);
            drop(c);
            drop(d);
            drop(e);
            drop(f);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let a = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a);
                let b = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(b);
                let c = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(c);
                let d = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(d);
                let e = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(e);
                let f = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(f);
                __widl_f_set_transform_CanvasRenderingContext2D(self_, a, b, c, d, e, f)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transform_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `transform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/transform)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn transform(
        &self,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transform_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                b: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                c: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                d: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                e: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                f: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transform_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            b: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            c: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            d: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            e: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            f: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a);
            drop(b);
            drop(c);
            drop(d);
            drop(e);
            drop(f);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let a = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a);
                let b = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(b);
                let c = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(c);
                let d = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(d);
                let e = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(e);
                let f = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(f);
                __widl_f_transform_CanvasRenderingContext2D(self_, a, b, c, d, e, f)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_translate_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d",))]
    #[allow(bad_style)]
    #[doc = "The `translate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/translate)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn translate(&self, x: f64, y: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_translate_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_translate_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_translate_CanvasRenderingContext2D(self_, x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_custom_focus_ring_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `drawCustomFocusRing()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawCustomFocusRing)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Element`*"]
    #[allow(clippy::all)]
    pub fn draw_custom_focus_ring(&self, element: &Element) -> bool {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_custom_focus_ring_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_custom_focus_ring_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(element);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let element = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                __widl_f_draw_custom_focus_ring_CanvasRenderingContext2D(self_, element)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_focus_if_needed_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `drawFocusIfNeeded()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawFocusIfNeeded)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Element`*"]
    #[allow(clippy::all)]
    pub fn draw_focus_if_needed(&self, element: &Element) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_focus_if_needed_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_focus_if_needed_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(element);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let element = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                __widl_f_draw_focus_if_needed_CanvasRenderingContext2D(self_, element)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
impl CanvasRenderingContext2d {
    pub const DRAWWINDOW_DRAW_CARET: u32 = 1u64 as u32;
}
impl CanvasRenderingContext2d {
    pub const DRAWWINDOW_DO_NOT_FLUSH: u32 = 2u64 as u32;
}
impl CanvasRenderingContext2d {
    pub const DRAWWINDOW_DRAW_VIEW: u32 = 4u64 as u32;
}
impl CanvasRenderingContext2d {
    pub const DRAWWINDOW_USE_WIDGET_LAYERS: u32 = 8u64 as u32;
}
impl CanvasRenderingContext2d {
    pub const DRAWWINDOW_ASYNC_DECODE_IMAGES: u32 = 16u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_76582d541621b292: [u8; 15195usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x19;\0\0\0\0x\0\0\x02\x18CanvasRenderingContext2D*__widl_instanceof_CanvasRenderingContext2D\0\0\0\0-__widl_f_draw_window_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x07\x05self_\x06window\x01x\x01y\x01w\x01h\x08bg_color\ndrawWindow\0\0\08__widl_f_draw_window_with_flags_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x08\x05self_\x06window\x01x\x01y\x01w\x01h\x08bg_color\x05flags\ndrawWindow\0\0\0(__widl_f_canvas_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x06canvas\x01\x01\x05self_\x06canvas\0\0\0.__widl_f_global_alpha_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x0BglobalAlpha\x01\x01\x05self_\x0BglobalAlpha\0\0\02__widl_f_set_global_alpha_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x0BglobalAlpha\x01\x02\x05self_\x0Cglobal_alpha\x0BglobalAlpha\0\0\0<__widl_f_global_composite_operation_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x18globalCompositeOperation\x01\x01\x05self_\x18globalCompositeOperation\0\0\0@__widl_f_set_global_composite_operation_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x18globalCompositeOperation\x01\x02\x05self_\x1Aglobal_composite_operation\x18globalCompositeOperation\0\0\0D__widl_f_draw_image_with_html_image_element_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\x05image\x02dx\x02dy\tdrawImage\0\0\0C__widl_f_draw_image_with_svg_image_element_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\x05image\x02dx\x02dy\tdrawImage\0\0\0E__widl_f_draw_image_with_html_canvas_element_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\x05image\x02dx\x02dy\tdrawImage\0\0\0D__widl_f_draw_image_with_html_video_element_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\x05image\x02dx\x02dy\tdrawImage\0\0\0>__widl_f_draw_image_with_image_bitmap_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\x05image\x02dx\x02dy\tdrawImage\0\0\0R__widl_f_draw_image_with_html_image_element_and_dw_and_dh_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x06\x05self_\x05image\x02dx\x02dy\x02dw\x02dh\tdrawImage\0\0\0Q__widl_f_draw_image_with_svg_image_element_and_dw_and_dh_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x06\x05self_\x05image\x02dx\x02dy\x02dw\x02dh\tdrawImage\0\0\0S__widl_f_draw_image_with_html_canvas_element_and_dw_and_dh_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x06\x05self_\x05image\x02dx\x02dy\x02dw\x02dh\tdrawImage\0\0\0R__widl_f_draw_image_with_html_video_element_and_dw_and_dh_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x06\x05self_\x05image\x02dx\x02dy\x02dw\x02dh\tdrawImage\0\0\0L__widl_f_draw_image_with_image_bitmap_and_dw_and_dh_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x06\x05self_\x05image\x02dx\x02dy\x02dw\x02dh\tdrawImage\0\0\0n__widl_f_draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\n\x05self_\x05image\x02sx\x02sy\x02sw\x02sh\x02dx\x02dy\x02dw\x02dh\tdrawImage\0\0\0m__widl_f_draw_image_with_svg_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\n\x05self_\x05image\x02sx\x02sy\x02sw\x02sh\x02dx\x02dy\x02dw\x02dh\tdrawImage\0\0\0o__widl_f_draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\n\x05self_\x05image\x02sx\x02sy\x02sw\x02sh\x02dx\x02dy\x02dw\x02dh\tdrawImage\0\0\0n__widl_f_draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\n\x05self_\x05image\x02sx\x02sy\x02sw\x02sh\x02dx\x02dy\x02dw\x02dh\tdrawImage\0\0\0h__widl_f_draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\n\x05self_\x05image\x02sx\x02sy\x02sw\x02sh\x02dx\x02dy\x02dw\x02dh\tdrawImage\0\0\0,__widl_f_begin_path_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\tbeginPath\0\0\0&__widl_f_clip_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x04clip\0\0\0?__widl_f_clip_with_canvas_winding_rule_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x07winding\x04clip\0\0\03__widl_f_clip_with_path_2d_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x04path\x04clip\0\0\0?__widl_f_clip_with_path_2d_and_winding_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x04path\x07winding\x04clip\0\0\0&__widl_f_fill_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x04fill\0\0\0?__widl_f_fill_with_canvas_winding_rule_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x07winding\x04fill\0\0\03__widl_f_fill_with_path_2d_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x04path\x04fill\0\0\0?__widl_f_fill_with_path_2d_and_winding_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x04path\x07winding\x04fill\0\0\0;__widl_f_is_point_in_path_with_f64_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x01x\x01y\risPointInPath\0\0\0S__widl_f_is_point_in_path_with_f64_and_canvas_winding_rule_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\x01x\x01y\x07winding\risPointInPath\0\0\0G__widl_f_is_point_in_path_with_path_2d_and_f64_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\x04path\x01x\x01y\risPointInPath\0\0\0S__widl_f_is_point_in_path_with_path_2d_and_f64_and_winding_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x04path\x01x\x01y\x07winding\risPointInPath\0\0\0A__widl_f_is_point_in_stroke_with_x_and_y_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x01x\x01y\x0FisPointInStroke\0\0\0J__widl_f_is_point_in_stroke_with_path_and_x_and_y_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\x04path\x01x\x01y\x0FisPointInStroke\0\0\0(__widl_f_stroke_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x06stroke\0\0\02__widl_f_stroke_with_path_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x04path\x06stroke\0\0\08__widl_f_create_linear_gradient_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x02x0\x02y0\x02x1\x02y1\x14createLinearGradient\0\0\0H__widl_f_create_pattern_with_html_image_element_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x05image\nrepetition\rcreatePattern\0\0\0G__widl_f_create_pattern_with_svg_image_element_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x05image\nrepetition\rcreatePattern\0\0\0I__widl_f_create_pattern_with_html_canvas_element_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x05image\nrepetition\rcreatePattern\0\0\0H__widl_f_create_pattern_with_html_video_element_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x05image\nrepetition\rcreatePattern\0\0\0B__widl_f_create_pattern_with_image_bitmap_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x05image\nrepetition\rcreatePattern\0\0\08__widl_f_create_radial_gradient_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x07\x05self_\x02x0\x02y0\x02r0\x02x1\x02y1\x02r1\x14createRadialGradient\0\0\0.__widl_f_stroke_style_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x0BstrokeStyle\x01\x01\x05self_\x0BstrokeStyle\0\0\02__widl_f_set_stroke_style_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x0BstrokeStyle\x01\x02\x05self_\x0Cstroke_style\x0BstrokeStyle\0\0\0,__widl_f_fill_style_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\tfillStyle\x01\x01\x05self_\tfillStyle\0\0\00__widl_f_set_fill_style_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\tfillStyle\x01\x02\x05self_\nfill_style\tfillStyle\0\0\0(__widl_f_filter_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x06filter\x01\x01\x05self_\x06filter\0\0\0,__widl_f_set_filter_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x06filter\x01\x02\x05self_\x06filter\x06filter\0\0\00__widl_f_add_hit_region_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x0CaddHitRegion\0\0\0=__widl_f_add_hit_region_with_options_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x07options\x0CaddHitRegion\0\0\03__widl_f_clear_hit_regions_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x0FclearHitRegions\0\0\03__widl_f_remove_hit_region_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x02id\x0FremoveHitRegion\0\0\0B__widl_f_create_image_data_with_sw_and_sh_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x02sw\x02sh\x0FcreateImageData\0\0\0B__widl_f_create_image_data_with_imagedata_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\timagedata\x0FcreateImageData\0\0\00__widl_f_get_image_data_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x02sx\x02sy\x02sw\x02sh\x0CgetImageData\0\0\00__widl_f_put_image_data_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\timagedata\x02dx\x02dy\x0CputImageData\0\0\0j__widl_f_put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x08\x05self_\timagedata\x02dx\x02dy\x07dirty_x\x07dirty_y\x0Bdirty_width\x0Cdirty_height\x0CputImageData\0\0\09__widl_f_image_smoothing_enabled_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x15imageSmoothingEnabled\x01\x01\x05self_\x15imageSmoothingEnabled\0\0\0=__widl_f_set_image_smoothing_enabled_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x15imageSmoothingEnabled\x01\x02\x05self_\x17image_smoothing_enabled\x15imageSmoothingEnabled\0\0\0/__widl_f_get_line_dash_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x0BgetLineDash\0\0\0/__widl_f_set_line_dash_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x08segments\x0BsetLineDash\0\0\0,__widl_f_line_width_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\tlineWidth\x01\x01\x05self_\tlineWidth\0\0\00__widl_f_set_line_width_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\tlineWidth\x01\x02\x05self_\nline_width\tlineWidth\0\0\0*__widl_f_line_cap_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x07lineCap\x01\x01\x05self_\x07lineCap\0\0\0.__widl_f_set_line_cap_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x07lineCap\x01\x02\x05self_\x08line_cap\x07lineCap\0\0\0+__widl_f_line_join_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x08lineJoin\x01\x01\x05self_\x08lineJoin\0\0\0/__widl_f_set_line_join_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x08lineJoin\x01\x02\x05self_\tline_join\x08lineJoin\0\0\0-__widl_f_miter_limit_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\nmiterLimit\x01\x01\x05self_\nmiterLimit\0\0\01__widl_f_set_miter_limit_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\nmiterLimit\x01\x02\x05self_\x0Bmiter_limit\nmiterLimit\0\0\02__widl_f_line_dash_offset_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x0ElineDashOffset\x01\x01\x05self_\x0ElineDashOffset\0\0\06__widl_f_set_line_dash_offset_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x0ElineDashOffset\x01\x02\x05self_\x10line_dash_offset\x0ElineDashOffset\0\0\0%__widl_f_arc_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x06\x05self_\x01x\x01y\x06radius\x0Bstart_angle\tend_angle\x03arc\0\0\08__widl_f_arc_with_anticlockwise_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x07\x05self_\x01x\x01y\x06radius\x0Bstart_angle\tend_angle\ranticlockwise\x03arc\0\0\0(__widl_f_arc_to_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x06\x05self_\x02x1\x02y1\x02x2\x02y2\x06radius\x05arcTo\0\0\01__widl_f_bezier_curve_to_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x07\x05self_\x04cp1x\x04cp1y\x04cp2x\x04cp2y\x01x\x01y\rbezierCurveTo\0\0\0,__widl_f_close_path_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\tclosePath\0\0\0)__widl_f_ellipse_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x08\x05self_\x01x\x01y\x08radius_x\x08radius_y\x08rotation\x0Bstart_angle\tend_angle\x07ellipse\0\0\0<__widl_f_ellipse_with_anticlockwise_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\t\x05self_\x01x\x01y\x08radius_x\x08radius_y\x08rotation\x0Bstart_angle\tend_angle\ranticlockwise\x07ellipse\0\0\0)__widl_f_line_to_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x01x\x01y\x06lineTo\0\0\0)__widl_f_move_to_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x01x\x01y\x06moveTo\0\0\04__widl_f_quadratic_curve_to_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x03cpx\x03cpy\x01x\x01y\x10quadraticCurveTo\0\0\0&__widl_f_rect_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x01x\x01y\x01w\x01h\x04rect\0\0\0,__widl_f_clear_rect_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x01x\x01y\x01w\x01h\tclearRect\0\0\0+__widl_f_fill_rect_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x01x\x01y\x01w\x01h\x08fillRect\0\0\0-__widl_f_stroke_rect_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x01x\x01y\x01w\x01h\nstrokeRect\0\0\01__widl_f_shadow_offset_x_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\rshadowOffsetX\x01\x01\x05self_\rshadowOffsetX\0\0\05__widl_f_set_shadow_offset_x_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\rshadowOffsetX\x01\x02\x05self_\x0Fshadow_offset_x\rshadowOffsetX\0\0\01__widl_f_shadow_offset_y_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\rshadowOffsetY\x01\x01\x05self_\rshadowOffsetY\0\0\05__widl_f_set_shadow_offset_y_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\rshadowOffsetY\x01\x02\x05self_\x0Fshadow_offset_y\rshadowOffsetY\0\0\0-__widl_f_shadow_blur_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\nshadowBlur\x01\x01\x05self_\nshadowBlur\0\0\01__widl_f_set_shadow_blur_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\nshadowBlur\x01\x02\x05self_\x0Bshadow_blur\nshadowBlur\0\0\0.__widl_f_shadow_color_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x0BshadowColor\x01\x01\x05self_\x0BshadowColor\0\0\02__widl_f_set_shadow_color_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x0BshadowColor\x01\x02\x05self_\x0Cshadow_color\x0BshadowColor\0\0\0)__widl_f_restore_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x07restore\0\0\0&__widl_f_save_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x04save\0\0\0+__widl_f_fill_text_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\x04text\x01x\x01y\x08fillText\0\0\0:__widl_f_fill_text_with_max_width_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x04text\x01x\x01y\tmax_width\x08fillText\0\0\0.__widl_f_measure_text_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x04text\x0BmeasureText\0\0\0-__widl_f_stroke_text_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\x04text\x01x\x01y\nstrokeText\0\0\0<__widl_f_stroke_text_with_max_width_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x04text\x01x\x01y\tmax_width\nstrokeText\0\0\0&__widl_f_font_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x04font\x01\x01\x05self_\x04font\0\0\0*__widl_f_set_font_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x04font\x01\x02\x05self_\x04font\x04font\0\0\0,__widl_f_text_align_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\ttextAlign\x01\x01\x05self_\ttextAlign\0\0\00__widl_f_set_text_align_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\ttextAlign\x01\x02\x05self_\ntext_align\ttextAlign\0\0\0/__widl_f_text_baseline_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x0CtextBaseline\x01\x01\x05self_\x0CtextBaseline\0\0\03__widl_f_set_text_baseline_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x0CtextBaseline\x01\x02\x05self_\rtext_baseline\x0CtextBaseline\0\0\0/__widl_f_get_transform_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x0CgetTransform\0\0\01__widl_f_reset_transform_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x0EresetTransform\0\0\0(__widl_f_rotate_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x05angle\x06rotate\0\0\0'__widl_f_scale_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x01x\x01y\x05scale\0\0\0/__widl_f_set_transform_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x07\x05self_\x01a\x01b\x01c\x01d\x01e\x01f\x0CsetTransform\0\0\0+__widl_f_transform_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x07\x05self_\x01a\x01b\x01c\x01d\x01e\x01f\ttransform\0\0\0+__widl_f_translate_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x01x\x01y\ttranslate\0\0\08__widl_f_draw_custom_focus_ring_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x07element\x13drawCustomFocusRing\0\0\06__widl_f_draw_focus_if_needed_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x07element\x11drawFocusIfNeeded\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
