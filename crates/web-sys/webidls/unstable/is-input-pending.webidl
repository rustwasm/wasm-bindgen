dictionary IsInputPendingOptions {
  boolean includeContinuous = false;
};

[Exposed=Window] interface Scheduling {
   boolean isInputPending(optional IsInputPendingOptions isInputPendingOptions = {});
};

partial interface Navigator {
  readonly attribute Scheduling scheduling;
};
