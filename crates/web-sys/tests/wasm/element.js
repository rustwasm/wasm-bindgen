export function new_div() {
  return document.createElement("div");
}

export function new_a() {
  return document.createElement("a");
}

export function new_body() {
  return document.createElement("body");
}

export function new_br() {
  return document.createElement("br");
}

export function new_button() {
  return document.createElement("button");
}

export function new_form() {
  return document.createElement("form");
}

export function new_head() {
  return document.createElement("head");
}

export function new_hr() {
    return document.createElement("hr");
}

export function new_html() {
  return document.createElement("html");
}

export function new_script() {
  return document.createElement("script");
}

export function new_span() {
  return document.createElement("span");
}

export function new_style() {
  return document.createElement("style");
}

export function new_input() {
  return document.createElement("input");
}

export function new_title() {
  return document.createElement("title");
}

export function new_heading() {
  return document.createElement("h1");
}

export function new_xpath_result() {
    let xmlDoc = new DOMParser().parseFromString("<root><value>tomato</value></root>", "application/xml");
    let xpathResult = xmlDoc.evaluate("/root//value", xmlDoc, null, XPathResult.ANY_TYPE, null);
    return xpathResult;
}
