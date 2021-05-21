[
    SecureContext,
    Exposed=(Window,Worker)
] interface mixin NavigatorDeviceMemory {
    readonly attribute double deviceMemory;
};

Navigator includes NavigatorDeviceMemory;
WorkerNavigator includes NavigatorDeviceMemory;
