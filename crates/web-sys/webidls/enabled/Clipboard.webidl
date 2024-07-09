/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/*
 * Clipboard API and events
 * Editor’s Draft, 4 June 2024
 *
 * The origin of this IDL file is:
 * https://w3c.github.io/clipboard-apis/#idl-index
 */

dictionary ClipboardEventInit : EventInit {
  DataTransfer? clipboardData = null;
};

[Exposed=Window]
interface ClipboardEvent : Event {
  constructor(DOMString type, optional ClipboardEventInit eventInitDict = {});
  readonly attribute DataTransfer? clipboardData;
};

partial interface Navigator {
  [SecureContext, SameObject] readonly attribute Clipboard clipboard;
};

typedef Promise<(DOMString or Blob)> ClipboardItemData;

[SecureContext, Exposed=Window]
interface ClipboardItem {
  [Throws]
  constructor(record<DOMString, ClipboardItemData> items,
              optional ClipboardItemOptions options = {});

  readonly attribute PresentationStyle presentationStyle;
  readonly attribute FrozenArray<DOMString> types;

  Promise<Blob> getType(DOMString type);

  static boolean supports(DOMString type);
};

enum PresentationStyle { "unspecified", "inline", "attachment" };

dictionary ClipboardItemOptions {
  PresentationStyle presentationStyle = "unspecified";
};

typedef sequence<ClipboardItem> ClipboardItems;

[SecureContext, Exposed=Window]
interface Clipboard : EventTarget {
  Promise<ClipboardItems> read(optional ClipboardUnsanitizedFormats formats = {});
  Promise<DOMString> readText();
  Promise<undefined> write(ClipboardItems data);
  Promise<undefined> writeText(DOMString data);
};
