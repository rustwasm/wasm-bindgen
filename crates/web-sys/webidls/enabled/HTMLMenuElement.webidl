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

// https://developer.mozilla.org/en-US/docs/Web/HTML/Element/menu
// > The <menu> and <ul> elements both represent an unordered list of items.
// > The key difference is that <ul> primarily contains items for display,
// > while <menu> was intended for interactive items.
[HTMLConstructor]
interface HTMLMenuElement : HTMLElement {
};

// > In early versions of the HTML specification, the <menu> element had
// > an additional use case as a context menu. This functionality is considered
// > obsolete and is not in the specification.
// Define this as deprecated partial interface to prevent breaking changes
// in wasm-bindgen.
[RustDeprecated="Absent in all major browsers"]
partial interface HTMLMenuElement {
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
