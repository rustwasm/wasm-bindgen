enum AudioSinkType {
    "none"
};

dictionary AudioSinkOptions {
    required AudioSinkType type;
};

partial dictionary AudioContextOptions {
    (DOMString or AudioSinkOptions) sinkId;
};

partial interface AudioContext {
  [SecureContext] readonly attribute (DOMString or AudioSinkInfo) sinkId;

  [SecureContext] Promise<undefined> setSinkId ((DOMString or AudioSinkOptions) sinkId);
  attribute EventHandler onsinkchange;
};
