/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 */

interface nsISupports;
interface IID;

[NoInterfaceObject,
 // Need Exposed here, because this is a mixin onto things like Event
 // that are exposed in workers.
 Exposed=(Window,Worker,System)]
interface LegacyQueryInterface {
  // Legacy QueryInterface, only exposed to chrome code on the main thread.
  [Exposed=(Window,System), ChromeOnly]
  nsISupports QueryInterface(IID iid);
};

BoxObject implements LegacyQueryInterface;
DOMParser implements LegacyQueryInterface;
Document implements LegacyQueryInterface;
DocumentFragment implements LegacyQueryInterface;
Element implements LegacyQueryInterface;
Event implements LegacyQueryInterface;
Selection implements LegacyQueryInterface;
TreeColumns implements LegacyQueryInterface;
TreeContentView implements LegacyQueryInterface;
Window implements LegacyQueryInterface;
XMLHttpRequest implements LegacyQueryInterface;
