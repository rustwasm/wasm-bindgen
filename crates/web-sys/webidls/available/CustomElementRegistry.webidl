/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://html.spec.whatwg.org/#dom-window-customelements
[Func="CustomElementRegistry::IsCustomElementEnabled"]
interface CustomElementRegistry {
  [CEReactions, Throws]
  void define(DOMString name, Function functionConstructor,
              optional ElementDefinitionOptions options);
  [ChromeOnly, Throws]
  void setElementCreationCallback(DOMString name, CustomElementCreationCallback callback);
  any get(DOMString name);
  [Throws]
  Promise<void> whenDefined(DOMString name);
  [CEReactions] void upgrade(Node root);
};

dictionary ElementDefinitionOptions {
  DOMString extends;
};

callback CustomElementCreationCallback = void (DOMString name);
