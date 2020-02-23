use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `History` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History)\n\n*This API requires the following crate features to be activated: `History`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct History {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_History: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for History {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(7u32);
            inform(72u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for History {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for History {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for History {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a History {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for History {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            History {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for History {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a History {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for History {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<History>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(History {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for History {
        #[inline]
        fn from(obj: JsValue) -> History {
            History { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for History {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<History> for History {
        #[inline]
        fn as_ref(&self) -> &History {
            self
        }
    }
    impl From<History> for JsValue {
        #[inline]
        fn from(obj: History) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for History {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_History(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_History(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_History(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            History { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const History) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<History> for ::js_sys::Object {
    #[inline]
    fn from(obj: History) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for History {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "History",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_back_History() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&History as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl History {
    #[cfg(all(feature = "History",))]
    #[allow(bad_style)]
    #[doc = "The `back()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/back)\n\n*This API requires the following crate features to be activated: `History`*"]
    #[allow(clippy::all)]
    pub fn back(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "History",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_back_History(
                self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_back_History(
            self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&History as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_back_History(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "History",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_forward_History() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&History as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl History {
    #[cfg(all(feature = "History",))]
    #[allow(bad_style)]
    #[doc = "The `forward()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/forward)\n\n*This API requires the following crate features to be activated: `History`*"]
    #[allow(clippy::all)]
    pub fn forward(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "History",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_forward_History(
                self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_forward_History(
            self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&History as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_forward_History(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "History",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_go_History() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&History as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl History {
    #[cfg(all(feature = "History",))]
    #[allow(bad_style)]
    #[doc = "The `go()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/go)\n\n*This API requires the following crate features to be activated: `History`*"]
    #[allow(clippy::all)]
    pub fn go(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "History",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_go_History(
                self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_go_History(
            self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&History as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_go_History(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "History",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_go_with_delta_History() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&History as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl History {
    #[cfg(all(feature = "History",))]
    #[allow(bad_style)]
    #[doc = "The `go()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/go)\n\n*This API requires the following crate features to be activated: `History`*"]
    #[allow(clippy::all)]
    pub fn go_with_delta(&self, delta: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "History",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_go_with_delta_History(
                self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                delta: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_go_with_delta_History(
            self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            delta: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(delta);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&History as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let delta = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(delta);
                __widl_f_go_with_delta_History(self_, delta)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "History",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_push_state_History() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&History as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl History {
    #[cfg(all(feature = "History",))]
    #[allow(bad_style)]
    #[doc = "The `pushState()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState)\n\n*This API requires the following crate features to be activated: `History`*"]
    #[allow(clippy::all)]
    pub fn push_state(
        &self,
        data: &::wasm_bindgen::JsValue,
        title: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "History",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_push_state_History(
                self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_push_state_History(
            self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            drop(title);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&History as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                __widl_f_push_state_History(self_, data, title)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "History",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_push_state_with_url_History() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&History as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl History {
    #[cfg(all(feature = "History",))]
    #[allow(bad_style)]
    #[doc = "The `pushState()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState)\n\n*This API requires the following crate features to be activated: `History`*"]
    #[allow(clippy::all)]
    pub fn push_state_with_url(
        &self,
        data: &::wasm_bindgen::JsValue,
        title: &str,
        url: Option<&str>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "History",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_push_state_with_url_History(
                self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_push_state_with_url_History(
            self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            drop(title);
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&History as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                let url = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_push_state_with_url_History(self_, data, title, url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "History",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_state_History() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&History as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl History {
    #[cfg(all(feature = "History",))]
    #[allow(bad_style)]
    #[doc = "The `replaceState()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState)\n\n*This API requires the following crate features to be activated: `History`*"]
    #[allow(clippy::all)]
    pub fn replace_state(
        &self,
        data: &::wasm_bindgen::JsValue,
        title: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "History",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_state_History(
                self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_state_History(
            self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            drop(title);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&History as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                __widl_f_replace_state_History(self_, data, title)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "History",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_state_with_url_History() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&History as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl History {
    #[cfg(all(feature = "History",))]
    #[allow(bad_style)]
    #[doc = "The `replaceState()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState)\n\n*This API requires the following crate features to be activated: `History`*"]
    #[allow(clippy::all)]
    pub fn replace_state_with_url(
        &self,
        data: &::wasm_bindgen::JsValue,
        title: &str,
        url: Option<&str>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "History",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_state_with_url_History(
                self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_state_with_url_History(
            self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            drop(title);
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&History as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                let url = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_replace_state_with_url_History(self_, data, title, url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "History",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_History() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&History as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl History {
    #[cfg(all(feature = "History",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/length)\n\n*This API requires the following crate features to be activated: `History`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> Result<u32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "History",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_History(
                self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_History(
            self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&History as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_History(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "History", feature = "ScrollRestoration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_restoration_History() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&History as WasmDescribe>::describe();
    <ScrollRestoration as WasmDescribe>::describe();
}
impl History {
    #[cfg(all(feature = "History", feature = "ScrollRestoration",))]
    #[allow(bad_style)]
    #[doc = "The `scrollRestoration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/scrollRestoration)\n\n*This API requires the following crate features to be activated: `History`, `ScrollRestoration`*"]
    #[allow(clippy::all)]
    pub fn scroll_restoration(&self) -> Result<ScrollRestoration, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "History", feature = "ScrollRestoration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_restoration_History(
                self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ScrollRestoration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_restoration_History(
            self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ScrollRestoration as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&History as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scroll_restoration_History(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ScrollRestoration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "History", feature = "ScrollRestoration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_scroll_restoration_History() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&History as WasmDescribe>::describe();
    <ScrollRestoration as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl History {
    #[cfg(all(feature = "History", feature = "ScrollRestoration",))]
    #[allow(bad_style)]
    #[doc = "The `scrollRestoration` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/scrollRestoration)\n\n*This API requires the following crate features to be activated: `History`, `ScrollRestoration`*"]
    #[allow(clippy::all)]
    pub fn set_scroll_restoration(
        &self,
        scroll_restoration: ScrollRestoration,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "History", feature = "ScrollRestoration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_scroll_restoration_History(
                self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scroll_restoration: <ScrollRestoration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_scroll_restoration_History(
            self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scroll_restoration: <ScrollRestoration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(scroll_restoration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&History as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scroll_restoration =
                    <ScrollRestoration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        scroll_restoration,
                    );
                __widl_f_set_scroll_restoration_History(self_, scroll_restoration)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "History",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_state_History() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&History as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl History {
    #[cfg(all(feature = "History",))]
    #[allow(bad_style)]
    #[doc = "The `state` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/state)\n\n*This API requires the following crate features to be activated: `History`*"]
    #[allow(clippy::all)]
    pub fn state(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "History",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_state_History(
                self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_state_History(
            self_: <&History as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&History as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_state_History(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_cd5c5db783cf5fb5: [u8; 1052usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xDA\x03\0\0\0\0\r\0\0\x02\x07History\x19__widl_instanceof_History\0\0\0\0\x15__widl_f_back_History\x01\0\0\x01\x07History\x01\0\0\x01\x01\x05self_\x04back\0\0\0\x18__widl_f_forward_History\x01\0\0\x01\x07History\x01\0\0\x01\x01\x05self_\x07forward\0\0\0\x13__widl_f_go_History\x01\0\0\x01\x07History\x01\0\0\x01\x01\x05self_\x02go\0\0\0\x1E__widl_f_go_with_delta_History\x01\0\0\x01\x07History\x01\0\0\x01\x02\x05self_\x05delta\x02go\0\0\0\x1B__widl_f_push_state_History\x01\0\0\x01\x07History\x01\0\0\x01\x03\x05self_\x04data\x05title\tpushState\0\0\0$__widl_f_push_state_with_url_History\x01\0\0\x01\x07History\x01\0\0\x01\x04\x05self_\x04data\x05title\x03url\tpushState\0\0\0\x1E__widl_f_replace_state_History\x01\0\0\x01\x07History\x01\0\0\x01\x03\x05self_\x04data\x05title\x0CreplaceState\0\0\0'__widl_f_replace_state_with_url_History\x01\0\0\x01\x07History\x01\0\0\x01\x04\x05self_\x04data\x05title\x03url\x0CreplaceState\0\0\0\x17__widl_f_length_History\x01\0\0\x01\x07History\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0#__widl_f_scroll_restoration_History\x01\0\0\x01\x07History\x01\0\x01\x11scrollRestoration\x01\x01\x05self_\x11scrollRestoration\0\0\0'__widl_f_set_scroll_restoration_History\x01\0\0\x01\x07History\x01\0\x02\x11scrollRestoration\x01\x02\x05self_\x12scroll_restoration\x11scrollRestoration\0\0\0\x16__widl_f_state_History\x01\0\0\x01\x07History\x01\0\x01\x05state\x01\x01\x05self_\x05state\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
