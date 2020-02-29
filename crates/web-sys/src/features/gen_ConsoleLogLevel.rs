use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `ConsoleLogLevel` enum.
///
///*This API requires the following crate features to be activated: `ConsoleLogLevel`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ConsoleLogLevel {
    All = "All",
    Debug = "Debug",
    Log = "Log",
    Info = "Info",
    Clear = "Clear",
    Trace = "Trace",
    TimeLog = "TimeLog",
    TimeEnd = "TimeEnd",
    Time = "Time",
    Group = "Group",
    GroupEnd = "GroupEnd",
    Profile = "Profile",
    ProfileEnd = "ProfileEnd",
    Dir = "Dir",
    Dirxml = "Dirxml",
    Warn = "Warn",
    Error = "Error",
    Off = "Off",
}
