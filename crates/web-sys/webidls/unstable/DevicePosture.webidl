[SecureContext, Exposed=(Window)]
partial interface Navigator {
  [SameObject] readonly attribute DevicePosture devicePosture;
};

[SecureContext, Exposed=(Window)]
interface DevicePosture : EventTarget {
  readonly attribute DevicePostureType type;
  attribute EventHandler onchange;
};

enum DevicePostureType {
  "no-fold",
  "laptop",
  "flat",
  "tent",
  "tablet",
  "book"
};