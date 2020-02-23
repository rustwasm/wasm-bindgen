use super::*;
use js_sys::Object;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `IDBObjectStoreParameters` dictionary\n\n\n*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
pub struct IdbObjectStoreParameters {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl IdbObjectStoreParameters {
    #[doc = "Construct a new `IDBObjectStoreParameters`\n\n\n*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    pub fn new() -> IdbObjectStoreParameters {
        let mut _ret = IdbObjectStoreParameters {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `autoIncrement` field of this object\n\n\n*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    pub fn auto_increment(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("autoIncrement"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `keyPath` field of this object\n\n\n*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    pub fn key_path(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("keyPath"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
#[allow(bad_style)]
#[allow(clippy::all)]
const _CONST_IdbObjectStoreParameters: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<IdbObjectStoreParameters> for JsValue {
        #[inline]
        fn from(val: IdbObjectStoreParameters) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for IdbObjectStoreParameters {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for IdbObjectStoreParameters {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for IdbObjectStoreParameters {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbObjectStoreParameters {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for IdbObjectStoreParameters {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            IdbObjectStoreParameters {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for IdbObjectStoreParameters {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbObjectStoreParameters {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for IdbObjectStoreParameters {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for IdbObjectStoreParameters {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<IdbObjectStoreParameters>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(IdbObjectStoreParameters {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for IdbObjectStoreParameters {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbObjectStoreParameters {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbObjectStoreParameters) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_79a9c53590a460d3: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
