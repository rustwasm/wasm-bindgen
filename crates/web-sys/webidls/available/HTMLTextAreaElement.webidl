/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://www.whatwg.org/specs/web-apps/current-work/#the-textarea-element
 * http://www.whatwg.org/specs/web-apps/current-work/#other-elements,-attributes-and-apis
 * © Copyright 2004-2011 Apple Computer, Inc., Mozilla Foundation, and
 * Opera Software ASA. You are granted a license to use, reproduce
 * and create derivative works of this document.
 */

interface nsIEditor;
interface XULControllers;

[HTMLConstructor]
interface HTMLTextAreaElement : HTMLElement {
  [CEReactions, SetterThrows, Pure]
           attribute DOMString autocomplete;
  [CEReactions, SetterThrows, Pure]
           attribute boolean autofocus;
  [CEReactions, SetterThrows, Pure]
           attribute unsigned long cols;
           // attribute DOMString dirName;
  [CEReactions, SetterThrows, Pure]
           attribute boolean disabled;
  [Pure]
  readonly attribute HTMLFormElement? form;
           // attribute DOMString inputMode;
  [CEReactions, SetterThrows, Pure]
           attribute long maxLength;
  [CEReactions, SetterThrows, Pure]
           attribute long minLength;
  [CEReactions, SetterThrows, Pure]
           attribute DOMString name;
  [CEReactions, SetterThrows, Pure]
           attribute DOMString placeholder;
  [CEReactions, SetterThrows, Pure]
           attribute boolean readOnly;
  [CEReactions, SetterThrows, Pure]
           attribute boolean required;
  [CEReactions, SetterThrows, Pure]
           attribute unsigned long rows;
  [CEReactions, SetterThrows, Pure]
           attribute DOMString wrap;

  [Constant]
  readonly attribute DOMString type;
  [CEReactions, Throws, Pure]
           attribute DOMString defaultValue;
  [CEReactions, SetterThrows, TreatNullAs=EmptyString] attribute DOMString value;
  readonly attribute unsigned long textLength;

  readonly attribute boolean willValidate;
  readonly attribute ValidityState validity;
  [Throws]
  readonly attribute DOMString validationMessage;
  boolean checkValidity();
  boolean reportValidity();
  void setCustomValidity(DOMString error);

  readonly attribute NodeList labels;

  void select();
  [Throws]
           attribute unsigned long? selectionStart;
  [Throws]
           attribute unsigned long? selectionEnd;
  [Throws]
           attribute DOMString? selectionDirection;
  [Throws]
  void setRangeText(DOMString replacement);
  [Throws]
  void setRangeText(DOMString replacement, unsigned long start,
    unsigned long end, optional SelectionMode selectionMode = "preserve");
  [Throws]
  void setSelectionRange(unsigned long start, unsigned long end, optional DOMString direction);
};

partial interface HTMLTextAreaElement {
  // Chrome-only Mozilla extensions

  [Throws, ChromeOnly]
  readonly attribute XULControllers controllers;
};

HTMLTextAreaElement implements MozEditableElement;

partial interface HTMLTextAreaElement {
  [ChromeOnly]
  attribute DOMString previewValue;
};
