use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGNumberList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList)\n\n*This API requires the following crate features to be activated: `SvgNumberList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgNumberList {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgNumberList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgNumberList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(78u32);
            inform(117u32);
            inform(109u32);
            inform(98u32);
            inform(101u32);
            inform(114u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgNumberList {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgNumberList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgNumberList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgNumberList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgNumberList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgNumberList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgNumberList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgNumberList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgNumberList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgNumberList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgNumberList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgNumberList {
        #[inline]
        fn from(obj: JsValue) -> SvgNumberList {
            SvgNumberList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgNumberList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgNumberList> for SvgNumberList {
        #[inline]
        fn as_ref(&self) -> &SvgNumberList {
            self
        }
    }
    impl From<SvgNumberList> for JsValue {
        #[inline]
        fn from(obj: SvgNumberList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgNumberList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGNumberList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGNumberList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGNumberList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgNumberList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgNumberList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgNumberList> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgNumberList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgNumberList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_item_SVGNumberList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgNumberList as WasmDescribe>::describe();
    <&SvgNumber as WasmDescribe>::describe();
    <SvgNumber as WasmDescribe>::describe();
}
impl SvgNumberList {
    #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
    #[allow(bad_style)]
    #[doc = "The `appendItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/appendItem)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    #[allow(clippy::all)]
    pub fn append_item(&self, new_item: &SvgNumber) -> Result<SvgNumber, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_item_SVGNumberList(
                self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_item: <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_item_SVGNumberList(
            self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_item: <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(new_item);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let new_item =
                    <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_item);
                __widl_f_append_item_SVGNumberList(self_, new_item)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgNumberList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_SVGNumberList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgNumberList as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgNumberList {
    #[cfg(all(feature = "SvgNumberList",))]
    #[allow(bad_style)]
    #[doc = "The `clear()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/clear)\n\n*This API requires the following crate features to be activated: `SvgNumberList`*"]
    #[allow(clippy::all)]
    pub fn clear(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgNumberList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_SVGNumberList(
                self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_SVGNumberList(
            self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_SVGNumberList(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_item_SVGNumberList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgNumberList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SvgNumber as WasmDescribe>::describe();
}
impl SvgNumberList {
    #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
    #[allow(bad_style)]
    #[doc = "The `getItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/getItem)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    #[allow(clippy::all)]
    pub fn get_item(&self, index: u32) -> Result<SvgNumber, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_item_SVGNumberList(
                self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_item_SVGNumberList(
            self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_item_SVGNumberList(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_initialize_SVGNumberList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgNumberList as WasmDescribe>::describe();
    <&SvgNumber as WasmDescribe>::describe();
    <SvgNumber as WasmDescribe>::describe();
}
impl SvgNumberList {
    #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
    #[allow(bad_style)]
    #[doc = "The `initialize()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/initialize)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    #[allow(clippy::all)]
    pub fn initialize(&self, new_item: &SvgNumber) -> Result<SvgNumber, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_initialize_SVGNumberList(
                self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_item: <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_initialize_SVGNumberList(
            self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_item: <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(new_item);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let new_item =
                    <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_item);
                __widl_f_initialize_SVGNumberList(self_, new_item)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_item_before_SVGNumberList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SvgNumberList as WasmDescribe>::describe();
    <&SvgNumber as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SvgNumber as WasmDescribe>::describe();
}
impl SvgNumberList {
    #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
    #[allow(bad_style)]
    #[doc = "The `insertItemBefore()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/insertItemBefore)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    #[allow(clippy::all)]
    pub fn insert_item_before(
        &self,
        new_item: &SvgNumber,
        index: u32,
    ) -> Result<SvgNumber, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_item_before_SVGNumberList(
                self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_item: <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_item_before_SVGNumberList(
            self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_item: <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ = <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let new_item =
                    <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_item);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_insert_item_before_SVGNumberList(self_, new_item, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_item_SVGNumberList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgNumberList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SvgNumber as WasmDescribe>::describe();
}
impl SvgNumberList {
    #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
    #[allow(bad_style)]
    #[doc = "The `removeItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/removeItem)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    #[allow(clippy::all)]
    pub fn remove_item(&self, index: u32) -> Result<SvgNumber, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_item_SVGNumberList(
                self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_item_SVGNumberList(
            self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_remove_item_SVGNumberList(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_item_SVGNumberList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SvgNumberList as WasmDescribe>::describe();
    <&SvgNumber as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SvgNumber as WasmDescribe>::describe();
}
impl SvgNumberList {
    #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
    #[allow(bad_style)]
    #[doc = "The `replaceItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/replaceItem)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    #[allow(clippy::all)]
    pub fn replace_item(
        &self,
        new_item: &SvgNumber,
        index: u32,
    ) -> Result<SvgNumber, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_item_SVGNumberList(
                self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_item: <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_item_SVGNumberList(
            self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_item: <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ = <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let new_item =
                    <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_item);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_replace_item_SVGNumberList(self_, new_item, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_SVGNumberList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgNumberList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SvgNumber as WasmDescribe>::describe();
}
impl SvgNumberList {
    #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    #[allow(clippy::all)]
    pub fn get(&self, index: u32) -> Result<SvgNumber, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgNumber", feature = "SvgNumberList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_SVGNumberList(
                self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_SVGNumberList(
            self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_SVGNumberList(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgNumberList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_number_of_items_SVGNumberList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgNumberList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl SvgNumberList {
    #[cfg(all(feature = "SvgNumberList",))]
    #[allow(bad_style)]
    #[doc = "The `numberOfItems` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/numberOfItems)\n\n*This API requires the following crate features to be activated: `SvgNumberList`*"]
    #[allow(clippy::all)]
    pub fn number_of_items(&self) -> u32 {
        #[cfg(all(feature = "SvgNumberList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_number_of_items_SVGNumberList(
                self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_number_of_items_SVGNumberList(
            self_: <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgNumberList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_number_of_items_SVGNumberList(self_)
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
pub static __WASM_BINDGEN_GENERATED_3517f18e5b6135df: [u8; 926usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\\\x03\0\0\0\0\n\0\0\x02\rSVGNumberList\x1F__widl_instanceof_SVGNumberList\0\0\0\0\"__widl_f_append_item_SVGNumberList\x01\0\0\x01\rSVGNumberList\x01\0\0\x01\x02\x05self_\x08new_item\nappendItem\0\0\0\x1C__widl_f_clear_SVGNumberList\x01\0\0\x01\rSVGNumberList\x01\0\0\x01\x01\x05self_\x05clear\0\0\0\x1F__widl_f_get_item_SVGNumberList\x01\0\0\x01\rSVGNumberList\x01\0\0\x01\x02\x05self_\x05index\x07getItem\0\0\0!__widl_f_initialize_SVGNumberList\x01\0\0\x01\rSVGNumberList\x01\0\0\x01\x02\x05self_\x08new_item\ninitialize\0\0\0)__widl_f_insert_item_before_SVGNumberList\x01\0\0\x01\rSVGNumberList\x01\0\0\x01\x03\x05self_\x08new_item\x05index\x10insertItemBefore\0\0\0\"__widl_f_remove_item_SVGNumberList\x01\0\0\x01\rSVGNumberList\x01\0\0\x01\x02\x05self_\x05index\nremoveItem\0\0\0#__widl_f_replace_item_SVGNumberList\x01\0\0\x01\rSVGNumberList\x01\0\0\x01\x03\x05self_\x08new_item\x05index\x0BreplaceItem\0\0\0\x1A__widl_f_get_SVGNumberList\x01\0\0\x01\rSVGNumberList\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0&__widl_f_number_of_items_SVGNumberList\0\0\0\x01\rSVGNumberList\x01\0\x01\rnumberOfItems\x01\x01\x05self_\rnumberOfItems\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
