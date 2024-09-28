// https://www.w3.org/TR/gamepad

[Exposed=Window]
interface Gamepad {
  readonly attribute DOMString id;
  // TODO: remove `unsigned`
  readonly attribute unsigned long index;
  readonly attribute boolean connected;
  readonly attribute DOMHighResTimeStamp timestamp;
  readonly attribute GamepadMappingType mapping;
  readonly attribute FrozenArray<double> axes;
  readonly attribute FrozenArray<GamepadButton> buttons;
  [RustDeprecated]
  readonly attribute unsigned long displayId;
};

[Exposed=Window]
interface GamepadButton {
  readonly attribute boolean pressed;
  readonly attribute boolean touched;
  readonly attribute double value;
};

enum GamepadMappingType {
  "",
  "standard",
};

interface GamepadHapticActuator {
  [RustDeprecated]
  readonly attribute GamepadHapticActuatorType type;
};

[RustDeprecated]
enum GamepadHapticActuatorType {
  "vibration"
};

[Exposed=Window]
partial interface Navigator {
  [Throws] sequence<Gamepad?> getGamepads();
};

[Exposed=Window]

interface GamepadEvent: Event {
  // TODO: remove `optional` modifier on `eventInitDict`
  constructor(DOMString type, optional GamepadEventInit eventInitDict);
  // TODO: remove `?` modifier
  [SameObject] readonly attribute Gamepad? gamepad;
};

dictionary GamepadEventInit : EventInit {
  // TODO: add `required`
  // TODO: remove `?` modifier
  Gamepad? gamepad;
};
