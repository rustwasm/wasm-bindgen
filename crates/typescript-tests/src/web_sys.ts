import * as wbg from "../pkg/typescript_tests";

const doc: Document = document;

const docStruct = wbg.DocStruct.new(doc);

const el: HTMLElement = document.createElement("a");

docStruct.append_element(el);
docStruct.append_many(el, el, el);

const newDoc = docStruct.get_doc();

// This test ensures that the correct Typescript types are
// used. If "newDoc" is "any", then "event" will cause the
// compilation to fail because of noImplicitAny.
newDoc.addEventListener("load", event => {
  console.log(event);
});

// Same as above, but testing that the param is a document.
const listener: Parameters<
  Parameters<typeof wbg["DocStruct"]["new"]>[0]["addEventListener"]
>[1] = event => console.log(event);

newDoc.addEventListener("load", listener);
