partial interface Document {
  [Throws]
  ViewTransition startViewTransition(optional UpdateCallback? updateCallback = null);
};

callback UpdateCallback = Promise<any> ();

[Exposed=Window]
interface ViewTransition {
  readonly attribute Promise<undefined> updateCallbackDone;
  readonly attribute Promise<undefined> ready;
  readonly attribute Promise<undefined> finished;
  [Throws]
  undefined skipTransition();
};
