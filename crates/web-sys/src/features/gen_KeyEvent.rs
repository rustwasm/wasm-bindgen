use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = KeyEvent , typescript_name = KeyEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `KeyEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent)
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*
    pub type KeyEvent;

    # [ wasm_bindgen ( method , structural , js_class = "KeyEvent" , js_name = initKeyEvent ) ]
    ///The `initKeyEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*
    pub fn init_key_event(this: &KeyEvent, type_: &str);

    # [ wasm_bindgen ( method , structural , js_class = "KeyEvent" , js_name = initKeyEvent ) ]
    ///The `initKeyEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*
    pub fn init_key_event_with_can_bubble(this: &KeyEvent, type_: &str, can_bubble: bool);

    # [ wasm_bindgen ( method , structural , js_class = "KeyEvent" , js_name = initKeyEvent ) ]
    ///The `initKeyEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*
    pub fn init_key_event_with_can_bubble_and_cancelable(
        this: &KeyEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "KeyEvent" , js_name = initKeyEvent ) ]
    ///The `initKeyEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`, `Window`*
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view(
        this: &KeyEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "KeyEvent" , js_name = initKeyEvent ) ]
    ///The `initKeyEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`, `Window`*
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key(
        this: &KeyEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        ctrl_key: bool,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "KeyEvent" , js_name = initKeyEvent ) ]
    ///The `initKeyEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`, `Window`*
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key(
        this: &KeyEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        ctrl_key: bool,
        alt_key: bool,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "KeyEvent" , js_name = initKeyEvent ) ]
    ///The `initKeyEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`, `Window`*
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key(
        this: &KeyEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "KeyEvent" , js_name = initKeyEvent ) ]
    ///The `initKeyEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`, `Window`*
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key(
        this: &KeyEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "KeyEvent" , js_name = initKeyEvent ) ]
    ///The `initKeyEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`, `Window`*
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code(
        this: &KeyEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        key_code: u32,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "KeyEvent" , js_name = initKeyEvent ) ]
    ///The `initKeyEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyEvent/initKeyEvent)
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`, `Window`*
    pub fn init_key_event_with_can_bubble_and_cancelable_and_view_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_key_code_and_char_code(
        this: &KeyEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        key_code: u32,
        char_code: u32,
    );

}

impl KeyEvent {
    ///The `KeyEvent.DOM_VK_CANCEL` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_CANCEL: u32 = 3u64 as u32;

    ///The `KeyEvent.DOM_VK_HELP` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_HELP: u32 = 6u64 as u32;

    ///The `KeyEvent.DOM_VK_BACK_SPACE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_BACK_SPACE: u32 = 8u64 as u32;

    ///The `KeyEvent.DOM_VK_TAB` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_TAB: u32 = 9u64 as u32;

    ///The `KeyEvent.DOM_VK_CLEAR` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_CLEAR: u32 = 12u64 as u32;

    ///The `KeyEvent.DOM_VK_RETURN` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_RETURN: u32 = 13u64 as u32;

    ///The `KeyEvent.DOM_VK_SHIFT` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_SHIFT: u32 = 16u64 as u32;

    ///The `KeyEvent.DOM_VK_CONTROL` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_CONTROL: u32 = 17u64 as u32;

    ///The `KeyEvent.DOM_VK_ALT` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_ALT: u32 = 18u64 as u32;

    ///The `KeyEvent.DOM_VK_PAUSE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_PAUSE: u32 = 19u64 as u32;

    ///The `KeyEvent.DOM_VK_CAPS_LOCK` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_CAPS_LOCK: u32 = 20u64 as u32;

    ///The `KeyEvent.DOM_VK_KANA` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_KANA: u32 = 21u64 as u32;

    ///The `KeyEvent.DOM_VK_HANGUL` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_HANGUL: u32 = 21u64 as u32;

    ///The `KeyEvent.DOM_VK_EISU` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_EISU: u32 = 22u64 as u32;

    ///The `KeyEvent.DOM_VK_JUNJA` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_JUNJA: u32 = 23u64 as u32;

    ///The `KeyEvent.DOM_VK_FINAL` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_FINAL: u32 = 24u64 as u32;

    ///The `KeyEvent.DOM_VK_HANJA` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_HANJA: u32 = 25u64 as u32;

    ///The `KeyEvent.DOM_VK_KANJI` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_KANJI: u32 = 25u64 as u32;

    ///The `KeyEvent.DOM_VK_ESCAPE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_ESCAPE: u32 = 27u64 as u32;

    ///The `KeyEvent.DOM_VK_CONVERT` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_CONVERT: u32 = 28u64 as u32;

    ///The `KeyEvent.DOM_VK_NONCONVERT` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_NONCONVERT: u32 = 29u64 as u32;

    ///The `KeyEvent.DOM_VK_ACCEPT` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_ACCEPT: u32 = 30u64 as u32;

    ///The `KeyEvent.DOM_VK_MODECHANGE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_MODECHANGE: u32 = 31u64 as u32;

    ///The `KeyEvent.DOM_VK_SPACE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_SPACE: u32 = 32u64 as u32;

    ///The `KeyEvent.DOM_VK_PAGE_UP` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_PAGE_UP: u32 = 33u64 as u32;

    ///The `KeyEvent.DOM_VK_PAGE_DOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_PAGE_DOWN: u32 = 34u64 as u32;

    ///The `KeyEvent.DOM_VK_END` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_END: u32 = 35u64 as u32;

    ///The `KeyEvent.DOM_VK_HOME` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_HOME: u32 = 36u64 as u32;

    ///The `KeyEvent.DOM_VK_LEFT` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_LEFT: u32 = 37u64 as u32;

    ///The `KeyEvent.DOM_VK_UP` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_UP: u32 = 38u64 as u32;

    ///The `KeyEvent.DOM_VK_RIGHT` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_RIGHT: u32 = 39u64 as u32;

    ///The `KeyEvent.DOM_VK_DOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_DOWN: u32 = 40u64 as u32;

    ///The `KeyEvent.DOM_VK_SELECT` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_SELECT: u32 = 41u64 as u32;

    ///The `KeyEvent.DOM_VK_PRINT` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_PRINT: u32 = 42u64 as u32;

    ///The `KeyEvent.DOM_VK_EXECUTE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_EXECUTE: u32 = 43u64 as u32;

    ///The `KeyEvent.DOM_VK_PRINTSCREEN` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_PRINTSCREEN: u32 = 44u64 as u32;

    ///The `KeyEvent.DOM_VK_INSERT` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_INSERT: u32 = 45u64 as u32;

    ///The `KeyEvent.DOM_VK_DELETE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_DELETE: u32 = 46u64 as u32;

    ///The `KeyEvent.DOM_VK_0` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_0: u32 = 48u64 as u32;

    ///The `KeyEvent.DOM_VK_1` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_1: u32 = 49u64 as u32;

    ///The `KeyEvent.DOM_VK_2` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_2: u32 = 50u64 as u32;

    ///The `KeyEvent.DOM_VK_3` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_3: u32 = 51u64 as u32;

    ///The `KeyEvent.DOM_VK_4` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_4: u32 = 52u64 as u32;

    ///The `KeyEvent.DOM_VK_5` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_5: u32 = 53u64 as u32;

    ///The `KeyEvent.DOM_VK_6` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_6: u32 = 54u64 as u32;

    ///The `KeyEvent.DOM_VK_7` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_7: u32 = 55u64 as u32;

    ///The `KeyEvent.DOM_VK_8` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_8: u32 = 56u64 as u32;

    ///The `KeyEvent.DOM_VK_9` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_9: u32 = 57u64 as u32;

    ///The `KeyEvent.DOM_VK_COLON` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_COLON: u32 = 58u64 as u32;

    ///The `KeyEvent.DOM_VK_SEMICOLON` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_SEMICOLON: u32 = 59u64 as u32;

    ///The `KeyEvent.DOM_VK_LESS_THAN` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_LESS_THAN: u32 = 60u64 as u32;

    ///The `KeyEvent.DOM_VK_EQUALS` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_EQUALS: u32 = 61u64 as u32;

    ///The `KeyEvent.DOM_VK_GREATER_THAN` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_GREATER_THAN: u32 = 62u64 as u32;

    ///The `KeyEvent.DOM_VK_QUESTION_MARK` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_QUESTION_MARK: u32 = 63u64 as u32;

    ///The `KeyEvent.DOM_VK_AT` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_AT: u32 = 64u64 as u32;

    ///The `KeyEvent.DOM_VK_A` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_A: u32 = 65u64 as u32;

    ///The `KeyEvent.DOM_VK_B` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_B: u32 = 66u64 as u32;

    ///The `KeyEvent.DOM_VK_C` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_C: u32 = 67u64 as u32;

    ///The `KeyEvent.DOM_VK_D` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_D: u32 = 68u64 as u32;

    ///The `KeyEvent.DOM_VK_E` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_E: u32 = 69u64 as u32;

    ///The `KeyEvent.DOM_VK_F` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F: u32 = 70u64 as u32;

    ///The `KeyEvent.DOM_VK_G` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_G: u32 = 71u64 as u32;

    ///The `KeyEvent.DOM_VK_H` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_H: u32 = 72u64 as u32;

    ///The `KeyEvent.DOM_VK_I` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_I: u32 = 73u64 as u32;

    ///The `KeyEvent.DOM_VK_J` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_J: u32 = 74u64 as u32;

    ///The `KeyEvent.DOM_VK_K` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_K: u32 = 75u64 as u32;

    ///The `KeyEvent.DOM_VK_L` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_L: u32 = 76u64 as u32;

    ///The `KeyEvent.DOM_VK_M` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_M: u32 = 77u64 as u32;

    ///The `KeyEvent.DOM_VK_N` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_N: u32 = 78u64 as u32;

    ///The `KeyEvent.DOM_VK_O` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_O: u32 = 79u64 as u32;

    ///The `KeyEvent.DOM_VK_P` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_P: u32 = 80u64 as u32;

    ///The `KeyEvent.DOM_VK_Q` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_Q: u32 = 81u64 as u32;

    ///The `KeyEvent.DOM_VK_R` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_R: u32 = 82u64 as u32;

    ///The `KeyEvent.DOM_VK_S` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_S: u32 = 83u64 as u32;

    ///The `KeyEvent.DOM_VK_T` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_T: u32 = 84u64 as u32;

    ///The `KeyEvent.DOM_VK_U` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_U: u32 = 85u64 as u32;

    ///The `KeyEvent.DOM_VK_V` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_V: u32 = 86u64 as u32;

    ///The `KeyEvent.DOM_VK_W` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_W: u32 = 87u64 as u32;

    ///The `KeyEvent.DOM_VK_X` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_X: u32 = 88u64 as u32;

    ///The `KeyEvent.DOM_VK_Y` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_Y: u32 = 89u64 as u32;

    ///The `KeyEvent.DOM_VK_Z` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_Z: u32 = 90u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN: u32 = 91u64 as u32;

    ///The `KeyEvent.DOM_VK_CONTEXT_MENU` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_CONTEXT_MENU: u32 = 93u64 as u32;

    ///The `KeyEvent.DOM_VK_SLEEP` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_SLEEP: u32 = 95u64 as u32;

    ///The `KeyEvent.DOM_VK_NUMPAD0` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_NUMPAD0: u32 = 96u64 as u32;

    ///The `KeyEvent.DOM_VK_NUMPAD1` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_NUMPAD1: u32 = 97u64 as u32;

    ///The `KeyEvent.DOM_VK_NUMPAD2` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_NUMPAD2: u32 = 98u64 as u32;

    ///The `KeyEvent.DOM_VK_NUMPAD3` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_NUMPAD3: u32 = 99u64 as u32;

    ///The `KeyEvent.DOM_VK_NUMPAD4` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_NUMPAD4: u32 = 100u64 as u32;

    ///The `KeyEvent.DOM_VK_NUMPAD5` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_NUMPAD5: u32 = 101u64 as u32;

    ///The `KeyEvent.DOM_VK_NUMPAD6` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_NUMPAD6: u32 = 102u64 as u32;

    ///The `KeyEvent.DOM_VK_NUMPAD7` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_NUMPAD7: u32 = 103u64 as u32;

    ///The `KeyEvent.DOM_VK_NUMPAD8` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_NUMPAD8: u32 = 104u64 as u32;

    ///The `KeyEvent.DOM_VK_NUMPAD9` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_NUMPAD9: u32 = 105u64 as u32;

    ///The `KeyEvent.DOM_VK_MULTIPLY` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_MULTIPLY: u32 = 106u64 as u32;

    ///The `KeyEvent.DOM_VK_ADD` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_ADD: u32 = 107u64 as u32;

    ///The `KeyEvent.DOM_VK_SEPARATOR` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_SEPARATOR: u32 = 108u64 as u32;

    ///The `KeyEvent.DOM_VK_SUBTRACT` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_SUBTRACT: u32 = 109u64 as u32;

    ///The `KeyEvent.DOM_VK_DECIMAL` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_DECIMAL: u32 = 110u64 as u32;

    ///The `KeyEvent.DOM_VK_DIVIDE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_DIVIDE: u32 = 111u64 as u32;

    ///The `KeyEvent.DOM_VK_F1` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F1: u32 = 112u64 as u32;

    ///The `KeyEvent.DOM_VK_F2` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F2: u32 = 113u64 as u32;

    ///The `KeyEvent.DOM_VK_F3` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F3: u32 = 114u64 as u32;

    ///The `KeyEvent.DOM_VK_F4` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F4: u32 = 115u64 as u32;

    ///The `KeyEvent.DOM_VK_F5` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F5: u32 = 116u64 as u32;

    ///The `KeyEvent.DOM_VK_F6` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F6: u32 = 117u64 as u32;

    ///The `KeyEvent.DOM_VK_F7` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F7: u32 = 118u64 as u32;

    ///The `KeyEvent.DOM_VK_F8` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F8: u32 = 119u64 as u32;

    ///The `KeyEvent.DOM_VK_F9` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F9: u32 = 120u64 as u32;

    ///The `KeyEvent.DOM_VK_F10` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F10: u32 = 121u64 as u32;

    ///The `KeyEvent.DOM_VK_F11` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F11: u32 = 122u64 as u32;

    ///The `KeyEvent.DOM_VK_F12` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F12: u32 = 123u64 as u32;

    ///The `KeyEvent.DOM_VK_F13` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F13: u32 = 124u64 as u32;

    ///The `KeyEvent.DOM_VK_F14` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F14: u32 = 125u64 as u32;

    ///The `KeyEvent.DOM_VK_F15` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F15: u32 = 126u64 as u32;

    ///The `KeyEvent.DOM_VK_F16` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F16: u32 = 127u64 as u32;

    ///The `KeyEvent.DOM_VK_F17` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F17: u32 = 128u64 as u32;

    ///The `KeyEvent.DOM_VK_F18` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F18: u32 = 129u64 as u32;

    ///The `KeyEvent.DOM_VK_F19` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F19: u32 = 130u64 as u32;

    ///The `KeyEvent.DOM_VK_F20` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F20: u32 = 131u64 as u32;

    ///The `KeyEvent.DOM_VK_F21` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F21: u32 = 132u64 as u32;

    ///The `KeyEvent.DOM_VK_F22` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F22: u32 = 133u64 as u32;

    ///The `KeyEvent.DOM_VK_F23` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F23: u32 = 134u64 as u32;

    ///The `KeyEvent.DOM_VK_F24` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_F24: u32 = 135u64 as u32;

    ///The `KeyEvent.DOM_VK_NUM_LOCK` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_NUM_LOCK: u32 = 144u64 as u32;

    ///The `KeyEvent.DOM_VK_SCROLL_LOCK` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_SCROLL_LOCK: u32 = 145u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_FJ_JISHO` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_FJ_JISHO: u32 = 146u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_FJ_MASSHOU` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_FJ_MASSHOU: u32 = 147u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_FJ_TOUROKU` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_FJ_TOUROKU: u32 = 148u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_FJ_LOYA` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_FJ_LOYA: u32 = 149u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_FJ_ROYA` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_FJ_ROYA: u32 = 150u64 as u32;

    ///The `KeyEvent.DOM_VK_CIRCUMFLEX` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_CIRCUMFLEX: u32 = 160u64 as u32;

    ///The `KeyEvent.DOM_VK_EXCLAMATION` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_EXCLAMATION: u32 = 161u64 as u32;

    ///The `KeyEvent.DOM_VK_DOUBLE_QUOTE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_DOUBLE_QUOTE: u32 = 162u64 as u32;

    ///The `KeyEvent.DOM_VK_HASH` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_HASH: u32 = 163u64 as u32;

    ///The `KeyEvent.DOM_VK_DOLLAR` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_DOLLAR: u32 = 164u64 as u32;

    ///The `KeyEvent.DOM_VK_PERCENT` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_PERCENT: u32 = 165u64 as u32;

    ///The `KeyEvent.DOM_VK_AMPERSAND` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_AMPERSAND: u32 = 166u64 as u32;

    ///The `KeyEvent.DOM_VK_UNDERSCORE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_UNDERSCORE: u32 = 167u64 as u32;

    ///The `KeyEvent.DOM_VK_OPEN_PAREN` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_OPEN_PAREN: u32 = 168u64 as u32;

    ///The `KeyEvent.DOM_VK_CLOSE_PAREN` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_CLOSE_PAREN: u32 = 169u64 as u32;

    ///The `KeyEvent.DOM_VK_ASTERISK` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_ASTERISK: u32 = 170u64 as u32;

    ///The `KeyEvent.DOM_VK_PLUS` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_PLUS: u32 = 171u64 as u32;

    ///The `KeyEvent.DOM_VK_PIPE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_PIPE: u32 = 172u64 as u32;

    ///The `KeyEvent.DOM_VK_HYPHEN_MINUS` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_HYPHEN_MINUS: u32 = 173u64 as u32;

    ///The `KeyEvent.DOM_VK_OPEN_CURLY_BRACKET` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_OPEN_CURLY_BRACKET: u32 = 174u64 as u32;

    ///The `KeyEvent.DOM_VK_CLOSE_CURLY_BRACKET` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_CLOSE_CURLY_BRACKET: u32 = 175u64 as u32;

    ///The `KeyEvent.DOM_VK_TILDE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_TILDE: u32 = 176u64 as u32;

    ///The `KeyEvent.DOM_VK_VOLUME_MUTE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_VOLUME_MUTE: u32 = 181u64 as u32;

    ///The `KeyEvent.DOM_VK_VOLUME_DOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_VOLUME_DOWN: u32 = 182u64 as u32;

    ///The `KeyEvent.DOM_VK_VOLUME_UP` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_VOLUME_UP: u32 = 183u64 as u32;

    ///The `KeyEvent.DOM_VK_COMMA` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_COMMA: u32 = 188u64 as u32;

    ///The `KeyEvent.DOM_VK_PERIOD` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_PERIOD: u32 = 190u64 as u32;

    ///The `KeyEvent.DOM_VK_SLASH` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_SLASH: u32 = 191u64 as u32;

    ///The `KeyEvent.DOM_VK_BACK_QUOTE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_BACK_QUOTE: u32 = 192u64 as u32;

    ///The `KeyEvent.DOM_VK_OPEN_BRACKET` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_OPEN_BRACKET: u32 = 219u64 as u32;

    ///The `KeyEvent.DOM_VK_BACK_SLASH` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_BACK_SLASH: u32 = 220u64 as u32;

    ///The `KeyEvent.DOM_VK_CLOSE_BRACKET` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_CLOSE_BRACKET: u32 = 221u64 as u32;

    ///The `KeyEvent.DOM_VK_QUOTE` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_QUOTE: u32 = 222u64 as u32;

    ///The `KeyEvent.DOM_VK_META` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_META: u32 = 224u64 as u32;

    ///The `KeyEvent.DOM_VK_ALTGR` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_ALTGR: u32 = 225u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_ICO_HELP` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_ICO_HELP: u32 = 227u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_ICO_00` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_ICO_00: u32 = 228u64 as u32;

    ///The `KeyEvent.DOM_VK_PROCESSKEY` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_PROCESSKEY: u32 = 229u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_ICO_CLEAR` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_ICO_CLEAR: u32 = 230u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_RESET` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_RESET: u32 = 233u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_JUMP` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_JUMP: u32 = 234u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_PA1` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_PA1: u32 = 235u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_PA2` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_PA2: u32 = 236u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_PA3` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_PA3: u32 = 237u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_WSCTRL` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_WSCTRL: u32 = 238u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_CUSEL` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_CUSEL: u32 = 239u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_ATTN` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_ATTN: u32 = 240u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_FINISH` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_FINISH: u32 = 241u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_COPY` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_COPY: u32 = 242u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_AUTO` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_AUTO: u32 = 243u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_ENLW` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_ENLW: u32 = 244u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_BACKTAB` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_BACKTAB: u32 = 245u64 as u32;

    ///The `KeyEvent.DOM_VK_ATTN` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_ATTN: u32 = 246u64 as u32;

    ///The `KeyEvent.DOM_VK_CRSEL` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_CRSEL: u32 = 247u64 as u32;

    ///The `KeyEvent.DOM_VK_EXSEL` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_EXSEL: u32 = 248u64 as u32;

    ///The `KeyEvent.DOM_VK_EREOF` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_EREOF: u32 = 249u64 as u32;

    ///The `KeyEvent.DOM_VK_PLAY` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_PLAY: u32 = 250u64 as u32;

    ///The `KeyEvent.DOM_VK_ZOOM` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_ZOOM: u32 = 251u64 as u32;

    ///The `KeyEvent.DOM_VK_PA1` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_PA1: u32 = 253u64 as u32;

    ///The `KeyEvent.DOM_VK_WIN_OEM_CLEAR` const.
    ///
    ///*This API requires the following crate features to be activated: `KeyEvent`*

    pub const DOM_VK_WIN_OEM_CLEAR: u32 = 254u64 as u32;
}
