partial interface Navigator {
  [SecureContext] Promise<undefined> share(optional ShareData data = {});
};

dictionary ShareData {
  sequence<File> files;
  USVString title;
  USVString text;
  USVString url;
};