use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGTransformList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList)\n\n*This API requires the following crate features to be activated: `SvgTransformList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgTransformList {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgTransformList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgTransformList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(110u32);
            inform(115u32);
            inform(102u32);
            inform(111u32);
            inform(114u32);
            inform(109u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgTransformList {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgTransformList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgTransformList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgTransformList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgTransformList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgTransformList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgTransformList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgTransformList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgTransformList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgTransformList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgTransformList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgTransformList {
        #[inline]
        fn from(obj: JsValue) -> SvgTransformList {
            SvgTransformList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgTransformList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgTransformList> for SvgTransformList {
        #[inline]
        fn as_ref(&self) -> &SvgTransformList {
            self
        }
    }
    impl From<SvgTransformList> for JsValue {
        #[inline]
        fn from(obj: SvgTransformList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgTransformList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGTransformList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGTransformList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGTransformList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgTransformList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgTransformList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgTransformList> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgTransformList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgTransformList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_item_SVGTransformList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTransformList as WasmDescribe>::describe();
    <&SvgTransform as WasmDescribe>::describe();
    <SvgTransform as WasmDescribe>::describe();
}
impl SvgTransformList {
    #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
    #[allow(bad_style)]
    #[doc = "The `appendItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/appendItem)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    #[allow(clippy::all)]
    pub fn append_item(
        &self,
        new_item: &SvgTransform,
    ) -> Result<SvgTransform, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_item_SVGTransformList(
                self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_item: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_item_SVGTransformList(
            self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_item: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(new_item);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let new_item =
                    <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_item);
                __widl_f_append_item_SVGTransformList(self_, new_item)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgTransform as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SvgTransformList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_SVGTransformList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTransformList as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgTransformList {
    #[cfg(all(feature = "SvgTransformList",))]
    #[allow(bad_style)]
    #[doc = "The `clear()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/clear)\n\n*This API requires the following crate features to be activated: `SvgTransformList`*"]
    #[allow(clippy::all)]
    pub fn clear(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransformList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_SVGTransformList(
                self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_SVGTransformList(
            self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_SVGTransformList(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_consolidate_SVGTransformList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTransformList as WasmDescribe>::describe();
    <Option<SvgTransform> as WasmDescribe>::describe();
}
impl SvgTransformList {
    #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
    #[allow(bad_style)]
    #[doc = "The `consolidate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/consolidate)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    #[allow(clippy::all)]
    pub fn consolidate(&self) -> Result<Option<SvgTransform>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_consolidate_SVGTransformList(
                self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<SvgTransform> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_consolidate_SVGTransformList(
            self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<SvgTransform> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_consolidate_SVGTransformList(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<SvgTransform> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "SvgMatrix",
    feature = "SvgTransform",
    feature = "SvgTransformList",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_svg_transform_from_matrix_SVGTransformList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTransformList as WasmDescribe>::describe();
    <&SvgMatrix as WasmDescribe>::describe();
    <SvgTransform as WasmDescribe>::describe();
}
impl SvgTransformList {
    #[cfg(all(
        feature = "SvgMatrix",
        feature = "SvgTransform",
        feature = "SvgTransformList",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createSVGTransformFromMatrix()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/createSVGTransformFromMatrix)\n\n*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`, `SvgTransformList`*"]
    #[allow(clippy::all)]
    pub fn create_svg_transform_from_matrix(&self, matrix: &SvgMatrix) -> SvgTransform {
        #[cfg(all(
            feature = "SvgMatrix",
            feature = "SvgTransform",
            feature = "SvgTransformList",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_svg_transform_from_matrix_SVGTransformList(
                self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                matrix: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_svg_transform_from_matrix_SVGTransformList(
            self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            matrix: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(matrix);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let matrix = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(matrix);
                __widl_f_create_svg_transform_from_matrix_SVGTransformList(self_, matrix)
            };
            <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_item_SVGTransformList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTransformList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SvgTransform as WasmDescribe>::describe();
}
impl SvgTransformList {
    #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
    #[allow(bad_style)]
    #[doc = "The `getItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/getItem)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    #[allow(clippy::all)]
    pub fn get_item(&self, index: u32) -> Result<SvgTransform, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_item_SVGTransformList(
                self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_item_SVGTransformList(
            self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_item_SVGTransformList(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgTransform as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_initialize_SVGTransformList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTransformList as WasmDescribe>::describe();
    <&SvgTransform as WasmDescribe>::describe();
    <SvgTransform as WasmDescribe>::describe();
}
impl SvgTransformList {
    #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
    #[allow(bad_style)]
    #[doc = "The `initialize()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/initialize)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    #[allow(clippy::all)]
    pub fn initialize(
        &self,
        new_item: &SvgTransform,
    ) -> Result<SvgTransform, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_initialize_SVGTransformList(
                self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_item: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_initialize_SVGTransformList(
            self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_item: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(new_item);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let new_item =
                    <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_item);
                __widl_f_initialize_SVGTransformList(self_, new_item)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgTransform as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_item_before_SVGTransformList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SvgTransformList as WasmDescribe>::describe();
    <&SvgTransform as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SvgTransform as WasmDescribe>::describe();
}
impl SvgTransformList {
    #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
    #[allow(bad_style)]
    #[doc = "The `insertItemBefore()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/insertItemBefore)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    #[allow(clippy::all)]
    pub fn insert_item_before(
        &self,
        new_item: &SvgTransform,
        index: u32,
    ) -> Result<SvgTransform, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_item_before_SVGTransformList(
                self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_item: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_item_before_SVGTransformList(
            self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_item: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(new_item);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let new_item =
                    <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_item);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_insert_item_before_SVGTransformList(self_, new_item, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgTransform as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_item_SVGTransformList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTransformList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SvgTransform as WasmDescribe>::describe();
}
impl SvgTransformList {
    #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
    #[allow(bad_style)]
    #[doc = "The `removeItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/removeItem)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    #[allow(clippy::all)]
    pub fn remove_item(&self, index: u32) -> Result<SvgTransform, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_item_SVGTransformList(
                self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_item_SVGTransformList(
            self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_remove_item_SVGTransformList(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgTransform as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_item_SVGTransformList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SvgTransformList as WasmDescribe>::describe();
    <&SvgTransform as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SvgTransform as WasmDescribe>::describe();
}
impl SvgTransformList {
    #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
    #[allow(bad_style)]
    #[doc = "The `replaceItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/replaceItem)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    #[allow(clippy::all)]
    pub fn replace_item(
        &self,
        new_item: &SvgTransform,
        index: u32,
    ) -> Result<SvgTransform, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_item_SVGTransformList(
                self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_item: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_item_SVGTransformList(
            self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_item: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(new_item);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let new_item =
                    <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_item);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_replace_item_SVGTransformList(self_, new_item, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgTransform as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_SVGTransformList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTransformList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SvgTransform as WasmDescribe>::describe();
}
impl SvgTransformList {
    #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    #[allow(clippy::all)]
    pub fn get(&self, index: u32) -> Result<SvgTransform, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransform", feature = "SvgTransformList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_SVGTransformList(
                self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_SVGTransformList(
            self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_SVGTransformList(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgTransform as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SvgTransformList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_number_of_items_SVGTransformList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTransformList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl SvgTransformList {
    #[cfg(all(feature = "SvgTransformList",))]
    #[allow(bad_style)]
    #[doc = "The `numberOfItems` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/numberOfItems)\n\n*This API requires the following crate features to be activated: `SvgTransformList`*"]
    #[allow(clippy::all)]
    pub fn number_of_items(&self) -> u32 {
        #[cfg(all(feature = "SvgTransformList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_number_of_items_SVGTransformList(
                self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_number_of_items_SVGTransformList(
            self_: <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgTransformList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_number_of_items_SVGTransformList(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b1b00e13b3d48ad7: [u8; 1201usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}o\x04\0\0\0\0\x0C\0\0\x02\x10SVGTransformList\"__widl_instanceof_SVGTransformList\0\0\0\0%__widl_f_append_item_SVGTransformList\x01\0\0\x01\x10SVGTransformList\x01\0\0\x01\x02\x05self_\x08new_item\nappendItem\0\0\0\x1F__widl_f_clear_SVGTransformList\x01\0\0\x01\x10SVGTransformList\x01\0\0\x01\x01\x05self_\x05clear\0\0\0%__widl_f_consolidate_SVGTransformList\x01\0\0\x01\x10SVGTransformList\x01\0\0\x01\x01\x05self_\x0Bconsolidate\0\0\0:__widl_f_create_svg_transform_from_matrix_SVGTransformList\0\0\0\x01\x10SVGTransformList\x01\0\0\x01\x02\x05self_\x06matrix\x1CcreateSVGTransformFromMatrix\0\0\0\"__widl_f_get_item_SVGTransformList\x01\0\0\x01\x10SVGTransformList\x01\0\0\x01\x02\x05self_\x05index\x07getItem\0\0\0$__widl_f_initialize_SVGTransformList\x01\0\0\x01\x10SVGTransformList\x01\0\0\x01\x02\x05self_\x08new_item\ninitialize\0\0\0,__widl_f_insert_item_before_SVGTransformList\x01\0\0\x01\x10SVGTransformList\x01\0\0\x01\x03\x05self_\x08new_item\x05index\x10insertItemBefore\0\0\0%__widl_f_remove_item_SVGTransformList\x01\0\0\x01\x10SVGTransformList\x01\0\0\x01\x02\x05self_\x05index\nremoveItem\0\0\0&__widl_f_replace_item_SVGTransformList\x01\0\0\x01\x10SVGTransformList\x01\0\0\x01\x03\x05self_\x08new_item\x05index\x0BreplaceItem\0\0\0\x1D__widl_f_get_SVGTransformList\x01\0\0\x01\x10SVGTransformList\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0)__widl_f_number_of_items_SVGTransformList\0\0\0\x01\x10SVGTransformList\x01\0\x01\rnumberOfItems\x01\x01\x05self_\rnumberOfItems\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
