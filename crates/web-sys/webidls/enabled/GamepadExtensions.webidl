// https://w3c.github.io/gamepad/extensions.html

enum GamepadHand {
  "",  /* unknown, both hands, or not applicable */
  "left",
  "right"
};

[Exposed=Window]
interface GamepadPose {
  readonly attribute boolean hasOrientation;
  readonly attribute boolean hasPosition;

  [Throws] // TODO: remove
  readonly attribute Float32Array? position;
  [Throws] // TODO: remove
  readonly attribute Float32Array? linearVelocity;
  [Throws] // TODO: remove
  readonly attribute Float32Array? linearAcceleration;
  [Throws] // TODO: remove
  readonly attribute Float32Array? orientation;
  [Throws] // TODO: remove
  readonly attribute Float32Array? angularVelocity;
  [Throws] // TODO: remove
  readonly attribute Float32Array? angularAcceleration;
};

partial interface Gamepad {
  readonly attribute GamepadHand hand;
  readonly attribute FrozenArray<GamepadHapticActuator> hapticActuators;
  readonly attribute GamepadPose? pose;
};

[Exposed=Window]
partial interface GamepadHapticActuator {
  [Throws] // TODO: remove
  Promise<boolean> pulse(double value, double duration);
};
