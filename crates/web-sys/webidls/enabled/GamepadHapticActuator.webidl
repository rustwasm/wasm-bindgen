/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://w3c.github.io/gamepad/
 * (Deprecated) https://w3c.github.io/gamepad/extensions.html#gamepadhapticactuator-interface
 */

[RustDeprecated]
enum GamepadHapticActuatorType {
  "vibration"
};

enum GamepadHapticEffectType {
  "dual-rumble",
  "trigger-rumble"
};

enum GamepadHapticsResult {
  "complete",
  "preempted"
};

dictionary GamepadEffectParameters {
  unsigned long long duration = 0;
  unsigned long long startDelay = 0;
  double strongMagnitude = 0.0;
  double weakMagnitude = 0.0;
  double leftTrigger = 0.0;
  double rightTrigger = 0.0;
};

[Pref="dom.gamepad.extensions.enabled",
  HeaderFile="mozilla/dom/GamepadHapticActuator.h"]
interface GamepadHapticActuator
{
  [RustDeprecated]
  readonly attribute GamepadHapticActuatorType type;
  [Throws, NewObject, RustDeprecated]
  Promise<boolean> pulse(double value, double duration);

	readonly attribute FrozenArray<GamepadHapticEffectType> effects;
  Promise<GamepadHapticsResult> playEffect(
      GamepadHapticEffectType type,
      optional GamepadEffectParameters params = {}
  );
  Promise<GamepadHapticsResult> reset();
};
