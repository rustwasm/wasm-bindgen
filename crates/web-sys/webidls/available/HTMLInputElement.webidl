/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://www.whatwg.org/specs/web-apps/current-work/#the-input-element
 * http://www.whatwg.org/specs/web-apps/current-work/#other-elements,-attributes-and-apis
 *
 * © Copyright 2004-2011 Apple Computer, Inc., Mozilla Foundation, and
 * Opera Software ASA. You are granted a license to use, reproduce
 * and create derivative works of this document.
 */

enum SelectionMode {
  "select",
  "start",
  "end",
  "preserve",
};

interface XULControllers;

[HTMLConstructor]
interface HTMLInputElement : HTMLElement {
  [CEReactions, Pure, SetterThrows]
           attribute DOMString accept;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString alt;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString autocomplete;
  [CEReactions, Pure, SetterThrows]
           attribute boolean autofocus;
  [CEReactions, Pure, SetterThrows]
           attribute boolean defaultChecked;
  [Pure]
           attribute boolean checked;
           // Bug 850337 - attribute DOMString dirName;
  [CEReactions, Pure, SetterThrows]
           attribute boolean disabled;
  readonly attribute HTMLFormElement? form;
  [Pure]
           attribute FileList? files;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString formAction;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString formEnctype;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString formMethod;
  [CEReactions, Pure, SetterThrows]
           attribute boolean formNoValidate;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString formTarget;
  [CEReactions, Pure, SetterThrows]
           attribute unsigned long height;
  [Pure]
           attribute boolean indeterminate;
  [CEReactions, Pure, SetterThrows, Pref="dom.forms.inputmode"]
           attribute DOMString inputMode;
  [Pure]
  readonly attribute HTMLElement? list;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString max;
  [CEReactions, Pure, SetterThrows]
           attribute long maxLength;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString min;
  [CEReactions, Pure, SetterThrows]
           attribute long minLength;
  [CEReactions, Pure, SetterThrows]
           attribute boolean multiple;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString name;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString pattern;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString placeholder;
  [CEReactions, Pure, SetterThrows]
           attribute boolean readOnly;
  [CEReactions, Pure, SetterThrows]
           attribute boolean required;
  [CEReactions, Pure, SetterThrows]
           attribute unsigned long size;
  [CEReactions, Pure, SetterNeedsSubjectPrincipal=NonSystem, SetterThrows]
           attribute DOMString src;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString step;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString type;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString defaultValue;
  [CEReactions, Pure, TreatNullAs=EmptyString, SetterThrows, NeedsCallerType]
           attribute DOMString value;
  [Throws, Func="HTMLInputElement::ValueAsDateEnabled"]
           attribute Date? valueAsDate;
  [Pure, SetterThrows]
           attribute unrestricted double valueAsNumber;
  [CEReactions, SetterThrows]
           attribute unsigned long width;

  [Throws]
  void stepUp(optional long n = 1);
  [Throws]
  void stepDown(optional long n = 1);

  [Pure]
  readonly attribute boolean willValidate;
  [Pure]
  readonly attribute ValidityState validity;
  [Throws]
  readonly attribute DOMString validationMessage;
  boolean checkValidity();
  boolean reportValidity();
  void setCustomValidity(DOMString error);

  readonly attribute NodeList? labels;

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

  // also has obsolete members
};

partial interface HTMLInputElement {
  [CEReactions, Pure, SetterThrows]
           attribute DOMString align;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString useMap;
};

// Mozilla extensions

partial interface HTMLInputElement {
  [GetterThrows, ChromeOnly]
  readonly attribute XULControllers        controllers;
  // Binaryname because we have a FragmentOrElement function named "TextLength()".
  [NeedsCallerType, BinaryName="inputTextLength"]
  readonly attribute long                  textLength;

  [Throws, ChromeOnly]
  sequence<DOMString> mozGetFileNameArray();

  [ChromeOnly, Throws]
  void mozSetFileNameArray(sequence<DOMString> fileNames);

  [ChromeOnly]
  void mozSetFileArray(sequence<File> files);

  // This method is meant to use for testing only.
  [ChromeOnly, Throws]
  void mozSetDirectory(DOMString directoryPath);

  // This method is meant to use for testing only.
  [ChromeOnly]
  void mozSetDndFilesAndDirectories(sequence<(File or Directory)> list);

  // Number controls (<input type=number>) have an anonymous text control
  // (<input type=text>) in the anonymous shadow tree that they contain. On
  // such an anonymous text control this property provides access to the
  // number control that owns the text control. This is useful, for example,
  // in code that looks at the currently focused element to make decisions
  // about which IME to bring up. Such code needs to be able to check for any
  // owning number control since it probably wants to bring up a number pad
  // instead of the standard keyboard, even when the anonymous text control has
  // focus.
  [ChromeOnly]
  readonly attribute HTMLInputElement? ownerNumberControl;

  boolean mozIsTextField(boolean aExcludePassword);

  [ChromeOnly]
  // This function will return null if @autocomplete is not defined for the
  // current @type
  AutocompleteInfo? getAutocompleteInfo();
};

[NoInterfaceObject]
interface MozEditableElement {
  [Pure, ChromeOnly]
  readonly attribute nsIEditor? editor;

  // This is similar to set .value on nsIDOMInput/TextAreaElements, but handling
  // of the value change is closer to the normal user input, so 'change' event
  // for example will be dispatched when focusing out the element.
  [Func="IsChromeOrXBL", NeedsSubjectPrincipal]
  void setUserInput(DOMString input);
};

HTMLInputElement implements MozEditableElement;

partial interface HTMLInputElement {
  [Pref="dom.input.dirpicker", SetterThrows]
  attribute boolean allowdirs;

  [Pref="dom.input.dirpicker"]
  readonly attribute boolean isFilesAndDirectoriesSupported;

  [Throws, Pref="dom.input.dirpicker"]
  Promise<sequence<(File or Directory)>> getFilesAndDirectories();

  [Throws, Pref="dom.input.dirpicker"]
  Promise<sequence<File>> getFiles(optional boolean recursiveFlag = false);

  [Throws, Pref="dom.input.dirpicker"]
  void chooseDirectory();
};

HTMLInputElement implements MozImageLoadingContent;

// Webkit/Blink
partial interface HTMLInputElement {
  [Pref="dom.webkitBlink.filesystem.enabled", Frozen, Cached, Pure]
  readonly attribute sequence<FileSystemEntry> webkitEntries;

  [Pref="dom.webkitBlink.dirPicker.enabled", BinaryName="WebkitDirectoryAttr", SetterThrows]
          attribute boolean webkitdirectory;
};

dictionary DateTimeValue {
  long hour;
  long minute;
  long year;
  long month;
  long day;
};

partial interface HTMLInputElement {
  [Pref="dom.forms.datetime", ChromeOnly]
  DateTimeValue getDateTimeInputBoxValue();

  [Pref="dom.forms.datetime", ChromeOnly]
  void updateDateTimeInputBox(optional DateTimeValue value);

  [Pref="dom.forms.datetime", ChromeOnly]
  void setDateTimePickerState(boolean open);

  [Pref="dom.forms.datetime", ChromeOnly,
   BinaryName="getMinimumAsDouble"]
  double getMinimum();

  [Pref="dom.forms.datetime", ChromeOnly,
   BinaryName="getMaximumAsDouble"]
  double getMaximum();

  [Pref="dom.forms.datetime", Func="IsChromeOrXBL"]
  void openDateTimePicker(optional DateTimeValue initialValue);

  [Pref="dom.forms.datetime", Func="IsChromeOrXBL"]
  void updateDateTimePicker(optional DateTimeValue value);

  [Pref="dom.forms.datetime", Func="IsChromeOrXBL"]
  void closeDateTimePicker();

  [Pref="dom.forms.datetime", Func="IsChromeOrXBL"]
  void setFocusState(boolean aIsFocused);

  [Pref="dom.forms.datetime", Func="IsChromeOrXBL"]
  void updateValidityState();

  [Pref="dom.forms.datetime", Func="IsChromeOrXBL",
   BinaryName="getStepAsDouble"]
  double getStep();

  [Pref="dom.forms.datetime", Func="IsChromeOrXBL",
   BinaryName="getStepBaseAsDouble"]
  double getStepBase();
};

partial interface HTMLInputElement {
  [ChromeOnly]
  attribute DOMString previewValue;
};
