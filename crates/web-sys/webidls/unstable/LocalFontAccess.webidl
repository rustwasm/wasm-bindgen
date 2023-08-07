[SecureContext]
partial interface Window {
  [Throws]
  Promise<sequence<FontData>> queryLocalFonts(optional QueryOptions options = {});
};

dictionary QueryOptions {
  sequence<DOMString> postscriptNames;
};

[Exposed=Window]
interface FontData {
  Promise<Blob> blob();

  // Names
  readonly attribute USVString postscriptName;
  readonly attribute USVString fullName;
  readonly attribute USVString family;
  readonly attribute USVString style;
};
