use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLOptionsCollection` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection)\n\n*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlOptionsCollection {
    obj: HtmlCollection,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlOptionsCollection: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlOptionsCollection {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(79u32);
            inform(112u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(115u32);
            inform(67u32);
            inform(111u32);
            inform(108u32);
            inform(108u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for HtmlOptionsCollection {
        type Target = HtmlCollection;
        #[inline]
        fn deref(&self) -> &HtmlCollection {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlOptionsCollection {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlOptionsCollection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlOptionsCollection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlOptionsCollection {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlOptionsCollection {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlOptionsCollection {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlOptionsCollection {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlOptionsCollection {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlOptionsCollection>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlOptionsCollection {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlOptionsCollection {
        #[inline]
        fn from(obj: JsValue) -> HtmlOptionsCollection {
            HtmlOptionsCollection { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlOptionsCollection {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlOptionsCollection> for HtmlOptionsCollection {
        #[inline]
        fn as_ref(&self) -> &HtmlOptionsCollection {
            self
        }
    }
    impl From<HtmlOptionsCollection> for JsValue {
        #[inline]
        fn from(obj: HtmlOptionsCollection) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlOptionsCollection {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLOptionsCollection(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLOptionsCollection(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLOptionsCollection(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlOptionsCollection { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlOptionsCollection) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlOptionsCollection> for HtmlCollection {
    #[inline]
    fn from(obj: HtmlOptionsCollection) -> HtmlCollection {
        use wasm_bindgen::JsCast;
        HtmlCollection::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlCollection> for HtmlOptionsCollection {
    #[inline]
    fn as_ref(&self) -> &HtmlCollection {
        use wasm_bindgen::JsCast;
        HtmlCollection::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlOptionsCollection> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlOptionsCollection) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlOptionsCollection {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlOptionElement", feature = "HtmlOptionsCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_html_option_element_HTMLOptionsCollection()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOptionsCollection as WasmDescribe>::describe();
    <&HtmlOptionElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionsCollection {
    #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlOptionsCollection",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlOptionsCollection`*"]
    #[allow(clippy::all)]
    pub fn add_with_html_option_element(
        &self,
        element: &HtmlOptionElement,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlOptionsCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_html_option_element_HTMLOptionsCollection(
                self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_html_option_element_HTMLOptionsCollection(
            self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element =
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                __widl_f_add_with_html_option_element_HTMLOptionsCollection(self_, element)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlOptGroupElement", feature = "HtmlOptionsCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_html_opt_group_element_HTMLOptionsCollection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOptionsCollection as WasmDescribe>::describe();
    <&HtmlOptGroupElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionsCollection {
    #[cfg(all(feature = "HtmlOptGroupElement", feature = "HtmlOptionsCollection",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)\n\n*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlOptionsCollection`*"]
    #[allow(clippy::all)]
    pub fn add_with_html_opt_group_element(
        &self,
        element: &HtmlOptGroupElement,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptGroupElement", feature = "HtmlOptionsCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_html_opt_group_element_HTMLOptionsCollection(
                self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_html_opt_group_element_HTMLOptionsCollection(
            self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element =
                    <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                __widl_f_add_with_html_opt_group_element_HTMLOptionsCollection(self_, element)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(
    feature = "HtmlElement",
    feature = "HtmlOptionElement",
    feature = "HtmlOptionsCollection",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_html_option_element_and_opt_html_element_HTMLOptionsCollection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlOptionsCollection as WasmDescribe>::describe();
    <&HtmlOptionElement as WasmDescribe>::describe();
    <Option<&HtmlElement> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionsCollection {
    #[cfg(all(
        feature = "HtmlElement",
        feature = "HtmlOptionElement",
        feature = "HtmlOptionsCollection",
    ))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlOptionElement`, `HtmlOptionsCollection`*"]
    #[allow(clippy::all)]
    pub fn add_with_html_option_element_and_opt_html_element(
        &self,
        element: &HtmlOptionElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "HtmlElement",
            feature = "HtmlOptionElement",
            feature = "HtmlOptionsCollection",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_html_option_element_and_opt_html_element_HTMLOptionsCollection(
                self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                before: <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_html_option_element_and_opt_html_element_HTMLOptionsCollection(
            self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            before: <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(element);
            drop(before);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element =
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                let before =
                    <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(before);
                __widl_f_add_with_html_option_element_and_opt_html_element_HTMLOptionsCollection(
                    self_, element, before,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(
    feature = "HtmlElement",
    feature = "HtmlOptGroupElement",
    feature = "HtmlOptionsCollection",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_html_opt_group_element_and_opt_html_element_HTMLOptionsCollection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlOptionsCollection as WasmDescribe>::describe();
    <&HtmlOptGroupElement as WasmDescribe>::describe();
    <Option<&HtmlElement> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionsCollection {
    #[cfg(all(
        feature = "HtmlElement",
        feature = "HtmlOptGroupElement",
        feature = "HtmlOptionsCollection",
    ))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlOptGroupElement`, `HtmlOptionsCollection`*"]
    #[allow(clippy::all)]
    pub fn add_with_html_opt_group_element_and_opt_html_element(
        &self,
        element: &HtmlOptGroupElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "HtmlElement",
            feature = "HtmlOptGroupElement",
            feature = "HtmlOptionsCollection",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_html_opt_group_element_and_opt_html_element_HTMLOptionsCollection(
                self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                before: <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_html_opt_group_element_and_opt_html_element_HTMLOptionsCollection(
            self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            before: <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(element);
            drop(before);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element =
                    <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                let before =
                    <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(before);
                __widl_f_add_with_html_opt_group_element_and_opt_html_element_HTMLOptionsCollection(
                    self_, element, before,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement", feature = "HtmlOptionsCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_html_option_element_and_opt_i32_HTMLOptionsCollection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlOptionsCollection as WasmDescribe>::describe();
    <&HtmlOptionElement as WasmDescribe>::describe();
    <Option<i32> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionsCollection {
    #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlOptionsCollection",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlOptionsCollection`*"]
    #[allow(clippy::all)]
    pub fn add_with_html_option_element_and_opt_i32(
        &self,
        element: &HtmlOptionElement,
        before: Option<i32>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlOptionsCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_html_option_element_and_opt_i32_HTMLOptionsCollection(
                self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                before: <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_html_option_element_and_opt_i32_HTMLOptionsCollection(
            self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            before: <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(element);
            drop(before);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element =
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                let before = <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(before);
                __widl_f_add_with_html_option_element_and_opt_i32_HTMLOptionsCollection(
                    self_, element, before,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlOptGroupElement", feature = "HtmlOptionsCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_html_opt_group_element_and_opt_i32_HTMLOptionsCollection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlOptionsCollection as WasmDescribe>::describe();
    <&HtmlOptGroupElement as WasmDescribe>::describe();
    <Option<i32> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionsCollection {
    #[cfg(all(feature = "HtmlOptGroupElement", feature = "HtmlOptionsCollection",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)\n\n*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlOptionsCollection`*"]
    #[allow(clippy::all)]
    pub fn add_with_html_opt_group_element_and_opt_i32(
        &self,
        element: &HtmlOptGroupElement,
        before: Option<i32>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptGroupElement", feature = "HtmlOptionsCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_html_opt_group_element_and_opt_i32_HTMLOptionsCollection(
                self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                before: <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_html_opt_group_element_and_opt_i32_HTMLOptionsCollection(
            self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            before: <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(element);
            drop(before);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element =
                    <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                let before = <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(before);
                __widl_f_add_with_html_opt_group_element_and_opt_i32_HTMLOptionsCollection(
                    self_, element, before,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlOptionsCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_HTMLOptionsCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOptionsCollection as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionsCollection {
    #[cfg(all(feature = "HtmlOptionsCollection",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/remove)\n\n*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
    #[allow(clippy::all)]
    pub fn remove(&self, index: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionsCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_HTMLOptionsCollection(
                self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_HTMLOptionsCollection(
            self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_remove_HTMLOptionsCollection(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement", feature = "HtmlOptionsCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_HTMLOptionsCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlOptionsCollection as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&HtmlOptionElement> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionsCollection {
    #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlOptionsCollection",))]
    #[allow(bad_style)]
    #[doc = "The indexing setter\n\n\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlOptionsCollection`*"]
    #[allow(clippy::all)]
    pub fn set(
        &self,
        index: u32,
        option: Option<&HtmlOptionElement>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlOptionsCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_HTMLOptionsCollection(
                self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                option: <Option<&HtmlOptionElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_HTMLOptionsCollection(
            self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            option: <Option<&HtmlOptionElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            drop(option);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                let option =
                    <Option<&HtmlOptionElement> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        option,
                    );
                __widl_f_set_HTMLOptionsCollection(self_, index, option)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlOptionsCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_HTMLOptionsCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOptionsCollection as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlOptionsCollection {
    #[cfg(all(feature = "HtmlOptionsCollection",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/length)\n\n*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "HtmlOptionsCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_HTMLOptionsCollection(
                self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_HTMLOptionsCollection(
            self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_HTMLOptionsCollection(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOptionsCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_length_HTMLOptionsCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOptionsCollection as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionsCollection {
    #[cfg(all(feature = "HtmlOptionsCollection",))]
    #[allow(bad_style)]
    #[doc = "The `length` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/length)\n\n*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
    #[allow(clippy::all)]
    pub fn set_length(&self, length: u32) {
        #[cfg(all(feature = "HtmlOptionsCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_length_HTMLOptionsCollection(
                self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_length_HTMLOptionsCollection(
            self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(length);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let length = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(length);
                __widl_f_set_length_HTMLOptionsCollection(self_, length)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlOptionsCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selected_index_HTMLOptionsCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOptionsCollection as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlOptionsCollection {
    #[cfg(all(feature = "HtmlOptionsCollection",))]
    #[allow(bad_style)]
    #[doc = "The `selectedIndex` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/selectedIndex)\n\n*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
    #[allow(clippy::all)]
    pub fn selected_index(&self) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionsCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selected_index_HTMLOptionsCollection(
                self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selected_index_HTMLOptionsCollection(
            self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selected_index_HTMLOptionsCollection(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlOptionsCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selected_index_HTMLOptionsCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOptionsCollection as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionsCollection {
    #[cfg(all(feature = "HtmlOptionsCollection",))]
    #[allow(bad_style)]
    #[doc = "The `selectedIndex` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/selectedIndex)\n\n*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
    #[allow(clippy::all)]
    pub fn set_selected_index(&self, selected_index: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionsCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selected_index_HTMLOptionsCollection(
                self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selected_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selected_index_HTMLOptionsCollection(
            self_: <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selected_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(selected_index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOptionsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selected_index =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selected_index);
                __widl_f_set_selected_index_HTMLOptionsCollection(self_, selected_index)
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
pub static __WASM_BINDGEN_GENERATED_5e394beb46604976: [u8; 1571usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE1\x05\0\0\0\0\r\0\0\x02\x15HTMLOptionsCollection'__widl_instanceof_HTMLOptionsCollection\0\0\0\0;__widl_f_add_with_html_option_element_HTMLOptionsCollection\x01\0\0\x01\x15HTMLOptionsCollection\x01\0\0\x01\x02\x05self_\x07element\x03add\0\0\0>__widl_f_add_with_html_opt_group_element_HTMLOptionsCollection\x01\0\0\x01\x15HTMLOptionsCollection\x01\0\0\x01\x02\x05self_\x07element\x03add\0\0\0P__widl_f_add_with_html_option_element_and_opt_html_element_HTMLOptionsCollection\x01\0\0\x01\x15HTMLOptionsCollection\x01\0\0\x01\x03\x05self_\x07element\x06before\x03add\0\0\0S__widl_f_add_with_html_opt_group_element_and_opt_html_element_HTMLOptionsCollection\x01\0\0\x01\x15HTMLOptionsCollection\x01\0\0\x01\x03\x05self_\x07element\x06before\x03add\0\0\0G__widl_f_add_with_html_option_element_and_opt_i32_HTMLOptionsCollection\x01\0\0\x01\x15HTMLOptionsCollection\x01\0\0\x01\x03\x05self_\x07element\x06before\x03add\0\0\0J__widl_f_add_with_html_opt_group_element_and_opt_i32_HTMLOptionsCollection\x01\0\0\x01\x15HTMLOptionsCollection\x01\0\0\x01\x03\x05self_\x07element\x06before\x03add\0\0\0%__widl_f_remove_HTMLOptionsCollection\x01\0\0\x01\x15HTMLOptionsCollection\x01\0\0\x01\x02\x05self_\x05index\x06remove\0\0\0\"__widl_f_set_HTMLOptionsCollection\x01\0\0\x01\x15HTMLOptionsCollection\x01\0\x04\x01\x03\x05self_\x05index\x06option\x03set\0\0\0%__widl_f_length_HTMLOptionsCollection\0\0\0\x01\x15HTMLOptionsCollection\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0)__widl_f_set_length_HTMLOptionsCollection\0\0\0\x01\x15HTMLOptionsCollection\x01\0\x02\x06length\x01\x02\x05self_\x06length\x06length\0\0\0-__widl_f_selected_index_HTMLOptionsCollection\x01\0\0\x01\x15HTMLOptionsCollection\x01\0\x01\rselectedIndex\x01\x01\x05self_\rselectedIndex\0\0\01__widl_f_set_selected_index_HTMLOptionsCollection\x01\0\0\x01\x15HTMLOptionsCollection\x01\0\x02\rselectedIndex\x01\x02\x05self_\x0Eselected_index\rselectedIndex\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
