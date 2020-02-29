use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = OfflineResourceList , typescript_type = "OfflineResourceList" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `OfflineResourceList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub type OfflineResourceList;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "OfflineResourceList" , js_name = status ) ]
    ///Getter for the `status` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/status)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn status(this: &OfflineResourceList) -> Result<u16, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineResourceList" , js_name = onchecking ) ]
    ///Getter for the `onchecking` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onchecking)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn onchecking(this: &OfflineResourceList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "OfflineResourceList" , js_name = onchecking ) ]
    ///Setter for the `onchecking` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onchecking)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn set_onchecking(this: &OfflineResourceList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineResourceList" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onerror)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn onerror(this: &OfflineResourceList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "OfflineResourceList" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onerror)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn set_onerror(this: &OfflineResourceList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineResourceList" , js_name = onnoupdate ) ]
    ///Getter for the `onnoupdate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onnoupdate)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn onnoupdate(this: &OfflineResourceList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "OfflineResourceList" , js_name = onnoupdate ) ]
    ///Setter for the `onnoupdate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onnoupdate)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn set_onnoupdate(this: &OfflineResourceList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineResourceList" , js_name = ondownloading ) ]
    ///Getter for the `ondownloading` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/ondownloading)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn ondownloading(this: &OfflineResourceList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "OfflineResourceList" , js_name = ondownloading ) ]
    ///Setter for the `ondownloading` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/ondownloading)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn set_ondownloading(this: &OfflineResourceList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineResourceList" , js_name = onprogress ) ]
    ///Getter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn onprogress(this: &OfflineResourceList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "OfflineResourceList" , js_name = onprogress ) ]
    ///Setter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn set_onprogress(this: &OfflineResourceList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineResourceList" , js_name = onupdateready ) ]
    ///Getter for the `onupdateready` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onupdateready)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn onupdateready(this: &OfflineResourceList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "OfflineResourceList" , js_name = onupdateready ) ]
    ///Setter for the `onupdateready` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onupdateready)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn set_onupdateready(this: &OfflineResourceList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineResourceList" , js_name = oncached ) ]
    ///Getter for the `oncached` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/oncached)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn oncached(this: &OfflineResourceList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "OfflineResourceList" , js_name = oncached ) ]
    ///Setter for the `oncached` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/oncached)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn set_oncached(this: &OfflineResourceList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineResourceList" , js_name = onobsolete ) ]
    ///Getter for the `onobsolete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onobsolete)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn onobsolete(this: &OfflineResourceList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "OfflineResourceList" , js_name = onobsolete ) ]
    ///Setter for the `onobsolete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onobsolete)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn set_onobsolete(this: &OfflineResourceList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineResourceList" , js_name = swapCache ) ]
    ///The `swapCache()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/swapCache)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn swap_cache(this: &OfflineResourceList) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineResourceList" , js_name = update ) ]
    ///The `update()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/update)
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*
    pub fn update(this: &OfflineResourceList) -> Result<(), JsValue>;

}

impl OfflineResourceList {
    ///The `OfflineResourceList.UNCACHED` const.
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*

    pub const UNCACHED: u16 = 0i64 as u16;

    ///The `OfflineResourceList.IDLE` const.
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*

    pub const IDLE: u16 = 1u64 as u16;

    ///The `OfflineResourceList.CHECKING` const.
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*

    pub const CHECKING: u16 = 2u64 as u16;

    ///The `OfflineResourceList.DOWNLOADING` const.
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*

    pub const DOWNLOADING: u16 = 3u64 as u16;

    ///The `OfflineResourceList.UPDATEREADY` const.
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*

    pub const UPDATEREADY: u16 = 4u64 as u16;

    ///The `OfflineResourceList.OBSOLETE` const.
    ///
    ///*This API requires the following crate features to be activated: `OfflineResourceList`*

    pub const OBSOLETE: u16 = 5u64 as u16;
}
