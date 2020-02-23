use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ConsoleLogLevel {
    All = 0,
    Debug = 1,
    Log = 2,
    Info = 3,
    Clear = 4,
    Trace = 5,
    TimeLog = 6,
    TimeEnd = 7,
    Time = 8,
    Group = 9,
    GroupEnd = 10,
    Profile = 11,
    ProfileEnd = 12,
    Dir = 13,
    Dirxml = 14,
    Warn = 15,
    Error = 16,
    Off = 17,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ConsoleLogLevel {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<ConsoleLogLevel> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "All" => Some(ConsoleLogLevel::All),
            "Debug" => Some(ConsoleLogLevel::Debug),
            "Log" => Some(ConsoleLogLevel::Log),
            "Info" => Some(ConsoleLogLevel::Info),
            "Clear" => Some(ConsoleLogLevel::Clear),
            "Trace" => Some(ConsoleLogLevel::Trace),
            "TimeLog" => Some(ConsoleLogLevel::TimeLog),
            "TimeEnd" => Some(ConsoleLogLevel::TimeEnd),
            "Time" => Some(ConsoleLogLevel::Time),
            "Group" => Some(ConsoleLogLevel::Group),
            "GroupEnd" => Some(ConsoleLogLevel::GroupEnd),
            "Profile" => Some(ConsoleLogLevel::Profile),
            "ProfileEnd" => Some(ConsoleLogLevel::ProfileEnd),
            "Dir" => Some(ConsoleLogLevel::Dir),
            "Dirxml" => Some(ConsoleLogLevel::Dirxml),
            "Warn" => Some(ConsoleLogLevel::Warn),
            "Error" => Some(ConsoleLogLevel::Error),
            "Off" => Some(ConsoleLogLevel::Off),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ConsoleLogLevel {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ConsoleLogLevel {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ConsoleLogLevel {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ConsoleLogLevel::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ConsoleLogLevel::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ConsoleLogLevel {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ConsoleLogLevel {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ConsoleLogLevel> for wasm_bindgen::JsValue {
    fn from(obj: ConsoleLogLevel) -> wasm_bindgen::JsValue {
        match obj {
            ConsoleLogLevel::All => wasm_bindgen::JsValue::from_str("All"),
            ConsoleLogLevel::Debug => wasm_bindgen::JsValue::from_str("Debug"),
            ConsoleLogLevel::Log => wasm_bindgen::JsValue::from_str("Log"),
            ConsoleLogLevel::Info => wasm_bindgen::JsValue::from_str("Info"),
            ConsoleLogLevel::Clear => wasm_bindgen::JsValue::from_str("Clear"),
            ConsoleLogLevel::Trace => wasm_bindgen::JsValue::from_str("Trace"),
            ConsoleLogLevel::TimeLog => wasm_bindgen::JsValue::from_str("TimeLog"),
            ConsoleLogLevel::TimeEnd => wasm_bindgen::JsValue::from_str("TimeEnd"),
            ConsoleLogLevel::Time => wasm_bindgen::JsValue::from_str("Time"),
            ConsoleLogLevel::Group => wasm_bindgen::JsValue::from_str("Group"),
            ConsoleLogLevel::GroupEnd => wasm_bindgen::JsValue::from_str("GroupEnd"),
            ConsoleLogLevel::Profile => wasm_bindgen::JsValue::from_str("Profile"),
            ConsoleLogLevel::ProfileEnd => wasm_bindgen::JsValue::from_str("ProfileEnd"),
            ConsoleLogLevel::Dir => wasm_bindgen::JsValue::from_str("Dir"),
            ConsoleLogLevel::Dirxml => wasm_bindgen::JsValue::from_str("Dirxml"),
            ConsoleLogLevel::Warn => wasm_bindgen::JsValue::from_str("Warn"),
            ConsoleLogLevel::Error => wasm_bindgen::JsValue::from_str("Error"),
            ConsoleLogLevel::Off => wasm_bindgen::JsValue::from_str("Off"),
            ConsoleLogLevel::__Nonexhaustive => {
                panic!("attempted to convert invalid ConsoleLogLevel into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e0ba284b72c1c979: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
