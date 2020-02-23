use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBKeyRange` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbKeyRange {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbKeyRange: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbKeyRange {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(75u32);
            inform(101u32);
            inform(121u32);
            inform(82u32);
            inform(97u32);
            inform(110u32);
            inform(103u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for IdbKeyRange {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbKeyRange {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbKeyRange {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbKeyRange {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbKeyRange {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbKeyRange {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbKeyRange {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbKeyRange {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbKeyRange {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbKeyRange>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbKeyRange {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbKeyRange {
        #[inline]
        fn from(obj: JsValue) -> IdbKeyRange {
            IdbKeyRange { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbKeyRange {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbKeyRange> for IdbKeyRange {
        #[inline]
        fn as_ref(&self) -> &IdbKeyRange {
            self
        }
    }
    impl From<IdbKeyRange> for JsValue {
        #[inline]
        fn from(obj: IdbKeyRange) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbKeyRange {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBKeyRange(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBKeyRange(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBKeyRange(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbKeyRange { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbKeyRange) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbKeyRange> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbKeyRange) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbKeyRange {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bound_IDBKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbKeyRange as WasmDescribe>::describe();
}
impl IdbKeyRange {
    #[cfg(all(feature = "IdbKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `bound()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    #[allow(clippy::all)]
    pub fn bound(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
    ) -> Result<IdbKeyRange, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bound_IDBKeyRange(
                lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bound_IDBKeyRange(
            lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(lower);
            drop(upper);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let lower =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        lower,
                    );
                let upper =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        upper,
                    );
                __widl_f_bound_IDBKeyRange(lower, upper)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bound_with_lower_open_IDBKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <IdbKeyRange as WasmDescribe>::describe();
}
impl IdbKeyRange {
    #[cfg(all(feature = "IdbKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `bound()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    #[allow(clippy::all)]
    pub fn bound_with_lower_open(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
        lower_open: bool,
    ) -> Result<IdbKeyRange, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bound_with_lower_open_IDBKeyRange(
                lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                lower_open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bound_with_lower_open_IDBKeyRange(
            lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            lower_open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(lower);
            drop(upper);
            drop(lower_open);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let lower =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        lower,
                    );
                let upper =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        upper,
                    );
                let lower_open = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(lower_open);
                __widl_f_bound_with_lower_open_IDBKeyRange(lower, upper, lower_open)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bound_with_lower_open_and_upper_open_IDBKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <IdbKeyRange as WasmDescribe>::describe();
}
impl IdbKeyRange {
    #[cfg(all(feature = "IdbKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `bound()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    #[allow(clippy::all)]
    pub fn bound_with_lower_open_and_upper_open(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
        lower_open: bool,
        upper_open: bool,
    ) -> Result<IdbKeyRange, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bound_with_lower_open_and_upper_open_IDBKeyRange(
                lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                lower_open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                upper_open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bound_with_lower_open_and_upper_open_IDBKeyRange(
            lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            lower_open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            upper_open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(lower);
            drop(upper);
            drop(lower_open);
            drop(upper_open);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let lower =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        lower,
                    );
                let upper =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        upper,
                    );
                let lower_open = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(lower_open);
                let upper_open = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(upper_open);
                __widl_f_bound_with_lower_open_and_upper_open_IDBKeyRange(
                    lower, upper, lower_open, upper_open,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_includes_IDBKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbKeyRange as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl IdbKeyRange {
    #[cfg(all(feature = "IdbKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `includes()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/includes)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    #[allow(clippy::all)]
    pub fn includes(&self, key: &::wasm_bindgen::JsValue) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_includes_IDBKeyRange(
                self_: <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_includes_IDBKeyRange(
            self_: <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_includes_IDBKeyRange(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lower_bound_IDBKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbKeyRange as WasmDescribe>::describe();
}
impl IdbKeyRange {
    #[cfg(all(feature = "IdbKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `lowerBound()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerBound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    #[allow(clippy::all)]
    pub fn lower_bound(
        lower: &::wasm_bindgen::JsValue,
    ) -> Result<IdbKeyRange, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lower_bound_IDBKeyRange(
                lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lower_bound_IDBKeyRange(
            lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(lower);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let lower =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        lower,
                    );
                __widl_f_lower_bound_IDBKeyRange(lower)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lower_bound_with_open_IDBKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <IdbKeyRange as WasmDescribe>::describe();
}
impl IdbKeyRange {
    #[cfg(all(feature = "IdbKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `lowerBound()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerBound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    #[allow(clippy::all)]
    pub fn lower_bound_with_open(
        lower: &::wasm_bindgen::JsValue,
        open: bool,
    ) -> Result<IdbKeyRange, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lower_bound_with_open_IDBKeyRange(
                lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lower_bound_with_open_IDBKeyRange(
            lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(lower);
            drop(open);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let lower =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        lower,
                    );
                let open = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(open);
                __widl_f_lower_bound_with_open_IDBKeyRange(lower, open)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_only_IDBKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbKeyRange as WasmDescribe>::describe();
}
impl IdbKeyRange {
    #[cfg(all(feature = "IdbKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `only()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/only)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    #[allow(clippy::all)]
    pub fn only(value: &::wasm_bindgen::JsValue) -> Result<IdbKeyRange, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_only_IDBKeyRange(
                value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_only_IDBKeyRange(
            value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let value =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        value,
                    );
                __widl_f_only_IDBKeyRange(value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_upper_bound_IDBKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbKeyRange as WasmDescribe>::describe();
}
impl IdbKeyRange {
    #[cfg(all(feature = "IdbKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `upperBound()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperBound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    #[allow(clippy::all)]
    pub fn upper_bound(
        upper: &::wasm_bindgen::JsValue,
    ) -> Result<IdbKeyRange, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_upper_bound_IDBKeyRange(
                upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_upper_bound_IDBKeyRange(
            upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(upper);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let upper =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        upper,
                    );
                __widl_f_upper_bound_IDBKeyRange(upper)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_upper_bound_with_open_IDBKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <IdbKeyRange as WasmDescribe>::describe();
}
impl IdbKeyRange {
    #[cfg(all(feature = "IdbKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `upperBound()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperBound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    #[allow(clippy::all)]
    pub fn upper_bound_with_open(
        upper: &::wasm_bindgen::JsValue,
        open: bool,
    ) -> Result<IdbKeyRange, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_upper_bound_with_open_IDBKeyRange(
                upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_upper_bound_with_open_IDBKeyRange(
            upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(upper);
            drop(open);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let upper =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        upper,
                    );
                let open = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(open);
                __widl_f_upper_bound_with_open_IDBKeyRange(upper, open)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbKeyRange as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lower_IDBKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbKeyRange as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl IdbKeyRange {
    #[cfg(all(feature = "IdbKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `lower` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lower)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    #[allow(clippy::all)]
    pub fn lower(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lower_IDBKeyRange(
                self_: <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lower_IDBKeyRange(
            self_: <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_lower_IDBKeyRange(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_upper_IDBKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbKeyRange as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl IdbKeyRange {
    #[cfg(all(feature = "IdbKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `upper` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upper)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    #[allow(clippy::all)]
    pub fn upper(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_upper_IDBKeyRange(
                self_: <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_upper_IDBKeyRange(
            self_: <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_upper_IDBKeyRange(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lower_open_IDBKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbKeyRange as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl IdbKeyRange {
    #[cfg(all(feature = "IdbKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `lowerOpen` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerOpen)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    #[allow(clippy::all)]
    pub fn lower_open(&self) -> bool {
        #[cfg(all(feature = "IdbKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lower_open_IDBKeyRange(
                self_: <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lower_open_IDBKeyRange(
            self_: <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_lower_open_IDBKeyRange(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_upper_open_IDBKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbKeyRange as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl IdbKeyRange {
    #[cfg(all(feature = "IdbKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `upperOpen` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperOpen)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    #[allow(clippy::all)]
    pub fn upper_open(&self) -> bool {
        #[cfg(all(feature = "IdbKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_upper_open_IDBKeyRange(
                self_: <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_upper_open_IDBKeyRange(
            self_: <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbKeyRange as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_upper_open_IDBKeyRange(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_22458933407aa815: [u8; 1200usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}n\x04\0\0\0\0\x0E\0\0\x02\x0BIDBKeyRange\x1D__widl_instanceof_IDBKeyRange\0\0\0\0\x1A__widl_f_bound_IDBKeyRange\x01\0\0\x01\x0BIDBKeyRange\x01\x01\0\x01\x02\x05lower\x05upper\x05bound\0\0\0*__widl_f_bound_with_lower_open_IDBKeyRange\x01\0\0\x01\x0BIDBKeyRange\x01\x01\0\x01\x03\x05lower\x05upper\nlower_open\x05bound\0\0\09__widl_f_bound_with_lower_open_and_upper_open_IDBKeyRange\x01\0\0\x01\x0BIDBKeyRange\x01\x01\0\x01\x04\x05lower\x05upper\nlower_open\nupper_open\x05bound\0\0\0\x1D__widl_f_includes_IDBKeyRange\x01\0\0\x01\x0BIDBKeyRange\x01\0\0\x01\x02\x05self_\x03key\x08includes\0\0\0 __widl_f_lower_bound_IDBKeyRange\x01\0\0\x01\x0BIDBKeyRange\x01\x01\0\x01\x01\x05lower\nlowerBound\0\0\0*__widl_f_lower_bound_with_open_IDBKeyRange\x01\0\0\x01\x0BIDBKeyRange\x01\x01\0\x01\x02\x05lower\x04open\nlowerBound\0\0\0\x19__widl_f_only_IDBKeyRange\x01\0\0\x01\x0BIDBKeyRange\x01\x01\0\x01\x01\x05value\x04only\0\0\0 __widl_f_upper_bound_IDBKeyRange\x01\0\0\x01\x0BIDBKeyRange\x01\x01\0\x01\x01\x05upper\nupperBound\0\0\0*__widl_f_upper_bound_with_open_IDBKeyRange\x01\0\0\x01\x0BIDBKeyRange\x01\x01\0\x01\x02\x05upper\x04open\nupperBound\0\0\0\x1A__widl_f_lower_IDBKeyRange\x01\0\0\x01\x0BIDBKeyRange\x01\0\x01\x05lower\x01\x01\x05self_\x05lower\0\0\0\x1A__widl_f_upper_IDBKeyRange\x01\0\0\x01\x0BIDBKeyRange\x01\0\x01\x05upper\x01\x01\x05self_\x05upper\0\0\0\x1F__widl_f_lower_open_IDBKeyRange\0\0\0\x01\x0BIDBKeyRange\x01\0\x01\tlowerOpen\x01\x01\x05self_\tlowerOpen\0\0\0\x1F__widl_f_upper_open_IDBKeyRange\0\0\0\x01\x0BIDBKeyRange\x01\0\x01\tupperOpen\x01\x01\x05self_\tupperOpen\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
