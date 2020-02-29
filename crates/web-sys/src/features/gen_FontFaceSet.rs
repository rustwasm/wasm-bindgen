use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = FontFaceSet , typescript_type = "FontFaceSet" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FontFaceSet` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub type FontFaceSet;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFaceSet" , js_name = size ) ]
    ///Getter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/size)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn size(this: &FontFaceSet) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFaceSet" , js_name = onloading ) ]
    ///Getter for the `onloading` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloading)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn onloading(this: &FontFaceSet) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FontFaceSet" , js_name = onloading ) ]
    ///Setter for the `onloading` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloading)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn set_onloading(this: &FontFaceSet, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFaceSet" , js_name = onloadingdone ) ]
    ///Getter for the `onloadingdone` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingdone)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn onloadingdone(this: &FontFaceSet) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FontFaceSet" , js_name = onloadingdone ) ]
    ///Setter for the `onloadingdone` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingdone)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn set_onloadingdone(this: &FontFaceSet, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFaceSet" , js_name = onloadingerror ) ]
    ///Getter for the `onloadingerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingerror)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn onloadingerror(this: &FontFaceSet) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FontFaceSet" , js_name = onloadingerror ) ]
    ///Setter for the `onloadingerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingerror)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn set_onloadingerror(this: &FontFaceSet, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "FontFaceSet" , js_name = ready ) ]
    ///Getter for the `ready` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/ready)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn ready(this: &FontFaceSet) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "FontFaceSetLoadStatus")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFaceSet" , js_name = status ) ]
    ///Getter for the `status` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/status)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`, `FontFaceSetLoadStatus`*
    pub fn status(this: &FontFaceSet) -> FontFaceSetLoadStatus;

    #[cfg(feature = "FontFace")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FontFaceSet" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/add)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`, `FontFaceSet`*
    pub fn add(this: &FontFaceSet, font: &FontFace) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "FontFaceSet" , js_name = check ) ]
    ///The `check()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/check)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn check(this: &FontFaceSet, font: &str) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "FontFaceSet" , js_name = check ) ]
    ///The `check()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/check)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn check_with_text(this: &FontFaceSet, font: &str, text: &str) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "FontFaceSet" , js_name = clear ) ]
    ///The `clear()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/clear)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn clear(this: &FontFaceSet);

    #[cfg(feature = "FontFace")]
    # [ wasm_bindgen ( method , structural , js_class = "FontFaceSet" , js_name = delete ) ]
    ///The `delete()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/delete)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`, `FontFaceSet`*
    pub fn delete(this: &FontFaceSet, font: &FontFace) -> bool;

    #[cfg(feature = "FontFaceSetIterator")]
    # [ wasm_bindgen ( method , structural , js_class = "FontFaceSet" , js_name = entries ) ]
    ///The `entries()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/entries)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`, `FontFaceSetIterator`*
    pub fn entries(this: &FontFaceSet) -> FontFaceSetIterator;

    # [ wasm_bindgen ( catch , method , structural , js_class = "FontFaceSet" , js_name = forEach ) ]
    ///The `forEach()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/forEach)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn for_each(this: &FontFaceSet, cb: &::js_sys::Function) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "FontFaceSet" , js_name = forEach ) ]
    ///The `forEach()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/forEach)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn for_each_with_this_arg(
        this: &FontFaceSet,
        cb: &::js_sys::Function,
        this_arg: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "FontFace")]
    # [ wasm_bindgen ( method , structural , js_class = "FontFaceSet" , js_name = has ) ]
    ///The `has()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/has)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`, `FontFaceSet`*
    pub fn has(this: &FontFaceSet, font: &FontFace) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "FontFaceSet" , js_name = load ) ]
    ///The `load()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/load)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn load(this: &FontFaceSet, font: &str) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "FontFaceSet" , js_name = load ) ]
    ///The `load()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/load)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`*
    pub fn load_with_text(this: &FontFaceSet, font: &str, text: &str) -> ::js_sys::Promise;

    #[cfg(feature = "FontFaceSetIterator")]
    # [ wasm_bindgen ( method , structural , js_class = "FontFaceSet" , js_name = values ) ]
    ///The `values()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/values)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSet`, `FontFaceSetIterator`*
    pub fn values(this: &FontFaceSet) -> FontFaceSetIterator;

}
