use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = RTCDTMFSender , typescript_name = RTCDTMFSender ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcdtmfSender` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender)
    ///
    ///*This API requires the following crate features to be activated: `RtcdtmfSender`*
    pub type RtcdtmfSender;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDTMFSender" , js_name = ontonechange ) ]
    ///Getter for the `ontonechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/ontonechange)
    ///
    ///*This API requires the following crate features to be activated: `RtcdtmfSender`*
    pub fn ontonechange(this: &RtcdtmfSender) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCDTMFSender" , js_name = ontonechange ) ]
    ///Setter for the `ontonechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/ontonechange)
    ///
    ///*This API requires the following crate features to be activated: `RtcdtmfSender`*
    pub fn set_ontonechange(this: &RtcdtmfSender, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDTMFSender" , js_name = toneBuffer ) ]
    ///Getter for the `toneBuffer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/toneBuffer)
    ///
    ///*This API requires the following crate features to be activated: `RtcdtmfSender`*
    pub fn tone_buffer(this: &RtcdtmfSender) -> String;

    # [ wasm_bindgen ( method , structural , js_class = "RTCDTMFSender" , js_name = insertDTMF ) ]
    ///The `insertDTMF()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)
    ///
    ///*This API requires the following crate features to be activated: `RtcdtmfSender`*
    pub fn insert_dtmf(this: &RtcdtmfSender, tones: &str);

    # [ wasm_bindgen ( method , structural , js_class = "RTCDTMFSender" , js_name = insertDTMF ) ]
    ///The `insertDTMF()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)
    ///
    ///*This API requires the following crate features to be activated: `RtcdtmfSender`*
    pub fn insert_dtmf_with_duration(this: &RtcdtmfSender, tones: &str, duration: u32);

    # [ wasm_bindgen ( method , structural , js_class = "RTCDTMFSender" , js_name = insertDTMF ) ]
    ///The `insertDTMF()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)
    ///
    ///*This API requires the following crate features to be activated: `RtcdtmfSender`*
    pub fn insert_dtmf_with_duration_and_inter_tone_gap(
        this: &RtcdtmfSender,
        tones: &str,
        duration: u32,
        inter_tone_gap: u32,
    );

}
