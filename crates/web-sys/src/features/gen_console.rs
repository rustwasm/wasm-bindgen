pub mod console {

    use super::super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]

    extern "C" {

        # [ wasm_bindgen ( js_namespace = console , js_name = assert ) ]
        ///The `console.assert()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn assert();

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = assert ) ]
        ///The `console.assert()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn assert_with_condition_and_data(condition: bool, data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = assert ) ]
        ///The `console.assert()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn assert_with_condition_and_data_0(condition: bool);

        # [ wasm_bindgen ( js_namespace = console , js_name = assert ) ]
        ///The `console.assert()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn assert_with_condition_and_data_1(condition: bool, data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = assert ) ]
        ///The `console.assert()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn assert_with_condition_and_data_2(
            condition: bool,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = assert ) ]
        ///The `console.assert()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn assert_with_condition_and_data_3(
            condition: bool,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = assert ) ]
        ///The `console.assert()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn assert_with_condition_and_data_4(
            condition: bool,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = assert ) ]
        ///The `console.assert()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn assert_with_condition_and_data_5(
            condition: bool,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = assert ) ]
        ///The `console.assert()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn assert_with_condition_and_data_6(
            condition: bool,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = assert ) ]
        ///The `console.assert()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn assert_with_condition_and_data_7(
            condition: bool,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = clear ) ]
        ///The `console.clear()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/clear)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn clear();

        # [ wasm_bindgen ( js_namespace = console , js_name = count ) ]
        ///The `console.count()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/count)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn count();

        # [ wasm_bindgen ( js_namespace = console , js_name = count ) ]
        ///The `console.count()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/count)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn count_with_label(label: &str);

        # [ wasm_bindgen ( js_namespace = console , js_name = countReset ) ]
        ///The `console.countReset()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/countReset)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn count_reset();

        # [ wasm_bindgen ( js_namespace = console , js_name = countReset ) ]
        ///The `console.countReset()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/countReset)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn count_reset_with_label(label: &str);

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = debug ) ]
        ///The `console.debug()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn debug(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = debug ) ]
        ///The `console.debug()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn debug_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = debug ) ]
        ///The `console.debug()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn debug_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = debug ) ]
        ///The `console.debug()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn debug_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = debug ) ]
        ///The `console.debug()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn debug_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = debug ) ]
        ///The `console.debug()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn debug_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = debug ) ]
        ///The `console.debug()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn debug_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = debug ) ]
        ///The `console.debug()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn debug_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = debug ) ]
        ///The `console.debug()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn debug_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = dir ) ]
        ///The `console.dir()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dir(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = dir ) ]
        ///The `console.dir()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dir_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = dir ) ]
        ///The `console.dir()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dir_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = dir ) ]
        ///The `console.dir()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dir_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = dir ) ]
        ///The `console.dir()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dir_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = dir ) ]
        ///The `console.dir()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dir_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = dir ) ]
        ///The `console.dir()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dir_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = dir ) ]
        ///The `console.dir()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dir_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = dir ) ]
        ///The `console.dir()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dir_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = dirxml ) ]
        ///The `console.dirxml()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dirxml(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = dirxml ) ]
        ///The `console.dirxml()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dirxml_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = dirxml ) ]
        ///The `console.dirxml()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dirxml_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = dirxml ) ]
        ///The `console.dirxml()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dirxml_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = dirxml ) ]
        ///The `console.dirxml()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dirxml_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = dirxml ) ]
        ///The `console.dirxml()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dirxml_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = dirxml ) ]
        ///The `console.dirxml()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dirxml_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = dirxml ) ]
        ///The `console.dirxml()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dirxml_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = dirxml ) ]
        ///The `console.dirxml()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn dirxml_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = error ) ]
        ///The `console.error()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn error(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = error ) ]
        ///The `console.error()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn error_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = error ) ]
        ///The `console.error()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn error_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = error ) ]
        ///The `console.error()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn error_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = error ) ]
        ///The `console.error()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn error_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = error ) ]
        ///The `console.error()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn error_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = error ) ]
        ///The `console.error()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn error_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = error ) ]
        ///The `console.error()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn error_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = error ) ]
        ///The `console.error()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn error_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = exception ) ]
        ///The `console.exception()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn exception(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = exception ) ]
        ///The `console.exception()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn exception_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = exception ) ]
        ///The `console.exception()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn exception_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = exception ) ]
        ///The `console.exception()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn exception_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = exception ) ]
        ///The `console.exception()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn exception_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = exception ) ]
        ///The `console.exception()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn exception_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = exception ) ]
        ///The `console.exception()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn exception_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = exception ) ]
        ///The `console.exception()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn exception_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = exception ) ]
        ///The `console.exception()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn exception_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = group ) ]
        ///The `console.group()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = group ) ]
        ///The `console.group()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = group ) ]
        ///The `console.group()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = group ) ]
        ///The `console.group()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = group ) ]
        ///The `console.group()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = group ) ]
        ///The `console.group()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = group ) ]
        ///The `console.group()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = group ) ]
        ///The `console.group()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = group ) ]
        ///The `console.group()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = groupCollapsed ) ]
        ///The `console.groupCollapsed()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_collapsed(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = groupCollapsed ) ]
        ///The `console.groupCollapsed()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_collapsed_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = groupCollapsed ) ]
        ///The `console.groupCollapsed()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_collapsed_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = groupCollapsed ) ]
        ///The `console.groupCollapsed()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_collapsed_2(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = groupCollapsed ) ]
        ///The `console.groupCollapsed()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_collapsed_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = groupCollapsed ) ]
        ///The `console.groupCollapsed()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_collapsed_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = groupCollapsed ) ]
        ///The `console.groupCollapsed()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_collapsed_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = groupCollapsed ) ]
        ///The `console.groupCollapsed()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_collapsed_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = groupCollapsed ) ]
        ///The `console.groupCollapsed()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_collapsed_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = groupEnd ) ]
        ///The `console.groupEnd()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupEnd)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn group_end();

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = info ) ]
        ///The `console.info()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn info(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = info ) ]
        ///The `console.info()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn info_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = info ) ]
        ///The `console.info()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn info_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = info ) ]
        ///The `console.info()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn info_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = info ) ]
        ///The `console.info()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn info_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = info ) ]
        ///The `console.info()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn info_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = info ) ]
        ///The `console.info()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn info_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = info ) ]
        ///The `console.info()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn info_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = info ) ]
        ///The `console.info()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn info_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = log ) ]
        ///The `console.log()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn log(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = log ) ]
        ///The `console.log()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn log_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = log ) ]
        ///The `console.log()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn log_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = log ) ]
        ///The `console.log()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn log_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = log ) ]
        ///The `console.log()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn log_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = log ) ]
        ///The `console.log()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn log_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = log ) ]
        ///The `console.log()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn log_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = log ) ]
        ///The `console.log()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn log_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = log ) ]
        ///The `console.log()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn log_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = profile ) ]
        ///The `console.profile()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = profile ) ]
        ///The `console.profile()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = profile ) ]
        ///The `console.profile()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = profile ) ]
        ///The `console.profile()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = profile ) ]
        ///The `console.profile()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = profile ) ]
        ///The `console.profile()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = profile ) ]
        ///The `console.profile()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = profile ) ]
        ///The `console.profile()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = profile ) ]
        ///The `console.profile()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = profileEnd ) ]
        ///The `console.profileEnd()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_end(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = profileEnd ) ]
        ///The `console.profileEnd()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_end_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = profileEnd ) ]
        ///The `console.profileEnd()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_end_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = profileEnd ) ]
        ///The `console.profileEnd()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_end_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = profileEnd ) ]
        ///The `console.profileEnd()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_end_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = profileEnd ) ]
        ///The `console.profileEnd()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_end_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = profileEnd ) ]
        ///The `console.profileEnd()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_end_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = profileEnd ) ]
        ///The `console.profileEnd()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_end_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = profileEnd ) ]
        ///The `console.profileEnd()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn profile_end_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = table ) ]
        ///The `console.table()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn table(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = table ) ]
        ///The `console.table()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn table_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = table ) ]
        ///The `console.table()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn table_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = table ) ]
        ///The `console.table()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn table_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = table ) ]
        ///The `console.table()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn table_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = table ) ]
        ///The `console.table()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn table_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = table ) ]
        ///The `console.table()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn table_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = table ) ]
        ///The `console.table()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn table_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = table ) ]
        ///The `console.table()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn table_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = time ) ]
        ///The `console.time()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/time)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time();

        # [ wasm_bindgen ( js_namespace = console , js_name = time ) ]
        ///The `console.time()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/time)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_with_label(label: &str);

        # [ wasm_bindgen ( js_namespace = console , js_name = timeEnd ) ]
        ///The `console.timeEnd()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeEnd)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_end();

        # [ wasm_bindgen ( js_namespace = console , js_name = timeEnd ) ]
        ///The `console.timeEnd()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeEnd)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_end_with_label(label: &str);

        # [ wasm_bindgen ( js_namespace = console , js_name = timeLog ) ]
        ///The `console.timeLog()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_log();

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = timeLog ) ]
        ///The `console.timeLog()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_log_with_label_and_data(label: &str, data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = timeLog ) ]
        ///The `console.timeLog()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_log_with_label_and_data_0(label: &str);

        # [ wasm_bindgen ( js_namespace = console , js_name = timeLog ) ]
        ///The `console.timeLog()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_log_with_label_and_data_1(label: &str, data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = timeLog ) ]
        ///The `console.timeLog()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_log_with_label_and_data_2(
            label: &str,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = timeLog ) ]
        ///The `console.timeLog()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_log_with_label_and_data_3(
            label: &str,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = timeLog ) ]
        ///The `console.timeLog()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_log_with_label_and_data_4(
            label: &str,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = timeLog ) ]
        ///The `console.timeLog()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_log_with_label_and_data_5(
            label: &str,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = timeLog ) ]
        ///The `console.timeLog()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_log_with_label_and_data_6(
            label: &str,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = timeLog ) ]
        ///The `console.timeLog()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_log_with_label_and_data_7(
            label: &str,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = timeStamp ) ]
        ///The `console.timeStamp()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeStamp)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_stamp();

        # [ wasm_bindgen ( js_namespace = console , js_name = timeStamp ) ]
        ///The `console.timeStamp()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeStamp)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn time_stamp_with_data(data: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = trace ) ]
        ///The `console.trace()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn trace(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = trace ) ]
        ///The `console.trace()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn trace_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = trace ) ]
        ///The `console.trace()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn trace_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = trace ) ]
        ///The `console.trace()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn trace_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = trace ) ]
        ///The `console.trace()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn trace_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = trace ) ]
        ///The `console.trace()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn trace_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = trace ) ]
        ///The `console.trace()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn trace_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = trace ) ]
        ///The `console.trace()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn trace_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = trace ) ]
        ///The `console.trace()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn trace_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( variadic , js_namespace = console , js_name = warn ) ]
        ///The `console.warn()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn warn(data: &::js_sys::Array);

        # [ wasm_bindgen ( js_namespace = console , js_name = warn ) ]
        ///The `console.warn()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn warn_0();

        # [ wasm_bindgen ( js_namespace = console , js_name = warn ) ]
        ///The `console.warn()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn warn_1(data_1: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = warn ) ]
        ///The `console.warn()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn warn_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);

        # [ wasm_bindgen ( js_namespace = console , js_name = warn ) ]
        ///The `console.warn()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn warn_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = warn ) ]
        ///The `console.warn()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn warn_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = warn ) ]
        ///The `console.warn()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn warn_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = warn ) ]
        ///The `console.warn()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn warn_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );

        # [ wasm_bindgen ( js_namespace = console , js_name = warn ) ]
        ///The `console.warn()` function.
        ///
        ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)
        ///
        ///*This API requires the following crate features to be activated: `console`*
        pub fn warn_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );

    }
}
