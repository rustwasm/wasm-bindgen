/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/*
 * Clipboard API and events
 * Editor’s Draft, 21 November 2023
 *
 * The origin of this IDL file is:
 * https://w3c.github.io/clipboard-apis/#clipboard-event-interfaces
 */

dictionary ClipboardEventInit : EventInit {
  DataTransfer? clipboardData = null;
};

[Exposed=Window]
interface ClipboardEvent : Event {
  constructor(DOMString type, optional ClipboardEventInit eventInitDict = {});
  readonly attribute DataTransfer? clipboardData;
};
