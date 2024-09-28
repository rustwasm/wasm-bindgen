// https://w3c.github.io/gamepad/extensions.html

[Exposed=Window, SecureContext]
interface GamepadTouch {
  readonly attribute unsigned long touchId;
  readonly attribute octet surfaceId;
  readonly attribute Float32Array position;
  readonly attribute Uint32Array? surfaceDimensions;
};

partial interface Gamepad {
  readonly attribute FrozenArray<GamepadTouch>? touchEvents;
};
