// https://www.w3.org/TR/gamepad

partial interface Gamepad {
  [SameObject] readonly attribute GamepadHapticActuator vibrationActuator;
};

/* TODO: requires partial or extend support.
enum GamepadMappingType {
  "xr-standard",
};
*/

partial interface GamepadHapticActuator {
  [SameObject] readonly attribute FrozenArray<GamepadHapticEffectType> effects;
  Promise<GamepadHapticsResult> playEffect(
      GamepadHapticEffectType type,
      optional GamepadEffectParameters params = {}
  );
  Promise<GamepadHapticsResult> reset();
};

enum GamepadHapticsResult {
  "complete",
  "preempted"
};

enum GamepadHapticEffectType {
  "dual-rumble",
  "trigger-rumble"
};

dictionary GamepadEffectParameters {
    unsigned long long duration = 0;
    unsigned long long startDelay = 0;
    double strongMagnitude = 0.0;
    double weakMagnitude = 0.0;
    double leftTrigger = 0.0;
    double rightTrigger = 0.0;
};

partial interface mixin WindowEventHandlers {
  attribute EventHandler ongamepadconnected;
  attribute EventHandler ongamepaddisconnected;
};
