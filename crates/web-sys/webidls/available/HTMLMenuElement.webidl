/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://www.whatwg.org/specs/web-apps/current-work/#the-menu-element
 * http://www.whatwg.org/specs/web-apps/current-work/#other-elements,-attributes-and-apis
 *
 * © Copyright 2004-2011 Apple Computer, Inc., Mozilla Foundation, and
 * Opera Software ASA. You are granted a license to use, reproduce
 * and create derivative works of this document.
 */

interface MenuBuilder;

// http://www.whatwg.org/specs/web-apps/current-work/#the-menu-element
[HTMLConstructor]
interface HTMLMenuElement : HTMLElement {
           [CEReactions, SetterThrows]
           attribute DOMString type;
           [CEReactions, SetterThrows]
           attribute DOMString label;
};

// http://www.whatwg.org/specs/web-apps/current-work/#other-elements,-attributes-and-apis
partial interface HTMLMenuElement {
           [CEReactions, SetterThrows]
           attribute boolean compact;
};

// Mozilla specific stuff
partial interface HTMLMenuElement {
  /**
   * Creates and dispatches a trusted event named "show".
   * The event is not cancelable and does not bubble.
   * See http://www.whatwg.org/specs/web-apps/current-work/multipage/interactive-elements.html#context-menus
   */
  [ChromeOnly]
  void sendShowEvent();

  /**
   * Creates a native menu builder. The builder type is dependent on menu type.
   * Currently, it returns the @mozilla.org/content/html-menu-builder;1
   * component. Toolbar menus are not yet supported (the method returns null).
   */
  [ChromeOnly]
  MenuBuilder? createBuilder();

  /*
   * Builds a menu by iterating over menu children.
   * See http://www.whatwg.org/specs/web-apps/current-work/multipage/interactive-elements.html#building-menus-and-toolbars
   * The caller can use a native builder by calling createBuilder() or provide
   * a custom builder that implements the nsIMenuBuilder interface.
   * A custom builder can be used for example to build native context menus
   * that are not defined using <menupopup>.
   */
  [ChromeOnly]
  void build(MenuBuilder aBuilder);
};
