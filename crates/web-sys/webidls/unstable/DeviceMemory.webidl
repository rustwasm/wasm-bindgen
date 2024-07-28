// https://www.w3.org/TR/2022/WD-device-memory-1-20220722

[
    SecureContext,
    Exposed=(Window,Worker)
] interface mixin NavigatorDeviceMemory {
    readonly attribute double deviceMemory;
};

Navigator includes NavigatorDeviceMemory;
WorkerNavigator includes NavigatorDeviceMemory;
