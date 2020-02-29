use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IDBFileMetadataParameters ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IdbFileMetadataParameters` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*
    pub type IdbFileMetadataParameters;

}

impl IdbFileMetadataParameters {
    ///Construct a new `IdbFileMetadataParameters`.
    ///
    ///*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `lastModified` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*

    pub fn last_modified(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("lastModified"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `size` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*

    pub fn size(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("size"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
