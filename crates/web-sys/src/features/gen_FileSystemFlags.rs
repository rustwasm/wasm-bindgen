use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FileSystemFlags ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FileSystemFlags` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `FileSystemFlags`*
    pub type FileSystemFlags;

}

impl FileSystemFlags {
    ///Construct a new `FileSystemFlags`.
    ///
    ///*This API requires the following crate features to be activated: `FileSystemFlags`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `create` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `FileSystemFlags`*

    pub fn create(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("create"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `exclusive` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `FileSystemFlags`*

    pub fn exclusive(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("exclusive"),
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
