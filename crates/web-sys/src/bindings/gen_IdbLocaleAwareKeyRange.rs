use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBLocaleAwareKeyRange` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBLocaleAwareKeyRange)\n\n*This API requires the following crate features to be activated: `IdbLocaleAwareKeyRange`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbLocaleAwareKeyRange {
    obj: IdbKeyRange,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbLocaleAwareKeyRange: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbLocaleAwareKeyRange {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(76u32);
            inform(111u32);
            inform(99u32);
            inform(97u32);
            inform(108u32);
            inform(101u32);
            inform(65u32);
            inform(119u32);
            inform(97u32);
            inform(114u32);
            inform(101u32);
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
    impl core::ops::Deref for IdbLocaleAwareKeyRange {
        type Target = IdbKeyRange;
        #[inline]
        fn deref(&self) -> &IdbKeyRange {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbLocaleAwareKeyRange {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbLocaleAwareKeyRange {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbLocaleAwareKeyRange {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbLocaleAwareKeyRange {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbLocaleAwareKeyRange {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbLocaleAwareKeyRange {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbLocaleAwareKeyRange {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbLocaleAwareKeyRange {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbLocaleAwareKeyRange>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbLocaleAwareKeyRange {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbLocaleAwareKeyRange {
        #[inline]
        fn from(obj: JsValue) -> IdbLocaleAwareKeyRange {
            IdbLocaleAwareKeyRange { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbLocaleAwareKeyRange {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbLocaleAwareKeyRange> for IdbLocaleAwareKeyRange {
        #[inline]
        fn as_ref(&self) -> &IdbLocaleAwareKeyRange {
            self
        }
    }
    impl From<IdbLocaleAwareKeyRange> for JsValue {
        #[inline]
        fn from(obj: IdbLocaleAwareKeyRange) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbLocaleAwareKeyRange {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBLocaleAwareKeyRange(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBLocaleAwareKeyRange(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBLocaleAwareKeyRange(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbLocaleAwareKeyRange { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbLocaleAwareKeyRange) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbLocaleAwareKeyRange> for IdbKeyRange {
    #[inline]
    fn from(obj: IdbLocaleAwareKeyRange) -> IdbKeyRange {
        use wasm_bindgen::JsCast;
        IdbKeyRange::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<IdbKeyRange> for IdbLocaleAwareKeyRange {
    #[inline]
    fn as_ref(&self) -> &IdbKeyRange {
        use wasm_bindgen::JsCast;
        IdbKeyRange::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IdbLocaleAwareKeyRange> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbLocaleAwareKeyRange) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbLocaleAwareKeyRange {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbLocaleAwareKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bound_IDBLocaleAwareKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbLocaleAwareKeyRange as WasmDescribe>::describe();
}
impl IdbLocaleAwareKeyRange {
    #[cfg(all(feature = "IdbLocaleAwareKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `bound()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBLocaleAwareKeyRange/bound)\n\n*This API requires the following crate features to be activated: `IdbLocaleAwareKeyRange`*"]
    #[allow(clippy::all)]
    pub fn bound(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
    ) -> Result<IdbLocaleAwareKeyRange, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbLocaleAwareKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bound_IDBLocaleAwareKeyRange(
                lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbLocaleAwareKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bound_IDBLocaleAwareKeyRange(
            lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbLocaleAwareKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_bound_IDBLocaleAwareKeyRange(lower, upper)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbLocaleAwareKeyRange as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbLocaleAwareKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bound_with_lower_open_IDBLocaleAwareKeyRange() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <IdbLocaleAwareKeyRange as WasmDescribe>::describe();
}
impl IdbLocaleAwareKeyRange {
    #[cfg(all(feature = "IdbLocaleAwareKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `bound()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBLocaleAwareKeyRange/bound)\n\n*This API requires the following crate features to be activated: `IdbLocaleAwareKeyRange`*"]
    #[allow(clippy::all)]
    pub fn bound_with_lower_open(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
        lower_open: bool,
    ) -> Result<IdbLocaleAwareKeyRange, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbLocaleAwareKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bound_with_lower_open_IDBLocaleAwareKeyRange(
                lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                lower_open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbLocaleAwareKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bound_with_lower_open_IDBLocaleAwareKeyRange(
            lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            lower_open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbLocaleAwareKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_bound_with_lower_open_IDBLocaleAwareKeyRange(lower, upper, lower_open)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbLocaleAwareKeyRange as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbLocaleAwareKeyRange",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bound_with_lower_open_and_upper_open_IDBLocaleAwareKeyRange(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <IdbLocaleAwareKeyRange as WasmDescribe>::describe();
}
impl IdbLocaleAwareKeyRange {
    #[cfg(all(feature = "IdbLocaleAwareKeyRange",))]
    #[allow(bad_style)]
    #[doc = "The `bound()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBLocaleAwareKeyRange/bound)\n\n*This API requires the following crate features to be activated: `IdbLocaleAwareKeyRange`*"]
    #[allow(clippy::all)]
    pub fn bound_with_lower_open_and_upper_open(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
        lower_open: bool,
        upper_open: bool,
    ) -> Result<IdbLocaleAwareKeyRange, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbLocaleAwareKeyRange",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bound_with_lower_open_and_upper_open_IDBLocaleAwareKeyRange(
                lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                lower_open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                upper_open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbLocaleAwareKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bound_with_lower_open_and_upper_open_IDBLocaleAwareKeyRange(
            lower: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            upper: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            lower_open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            upper_open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbLocaleAwareKeyRange as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_bound_with_lower_open_and_upper_open_IDBLocaleAwareKeyRange(
                    lower, upper, lower_open, upper_open,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbLocaleAwareKeyRange as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b17f9558df1bb89e: [u8; 526usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCC\x01\0\0\0\0\x04\0\0\x02\x16IDBLocaleAwareKeyRange(__widl_instanceof_IDBLocaleAwareKeyRange\0\0\0\0%__widl_f_bound_IDBLocaleAwareKeyRange\x01\0\0\x01\x16IDBLocaleAwareKeyRange\x01\x01\0\x01\x02\x05lower\x05upper\x05bound\0\0\05__widl_f_bound_with_lower_open_IDBLocaleAwareKeyRange\x01\0\0\x01\x16IDBLocaleAwareKeyRange\x01\x01\0\x01\x03\x05lower\x05upper\nlower_open\x05bound\0\0\0D__widl_f_bound_with_lower_open_and_upper_open_IDBLocaleAwareKeyRange\x01\0\0\x01\x16IDBLocaleAwareKeyRange\x01\x01\0\x01\x04\x05lower\x05upper\nlower_open\nupper_open\x05bound\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
