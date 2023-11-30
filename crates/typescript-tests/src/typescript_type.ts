import * as wbg from "../pkg/typescript_tests";
import { expect, jest, test } from "@jest/globals";

test("constructor", () => {
  const style: wbg.TextStyle = new wbg.TextStyle({
    bold: true,
    italic: false,
    size: 42,
  });

  expect(style.bold).toStrictEqual(true);
  expect(style.italic).toStrictEqual(false);
  expect(style.size).toStrictEqual(42);
});

test("optional parameter constructor", () => {
  const default_constructed: wbg.TextStyle = wbg.TextStyle.optional_new();
  expect(default_constructed.bold).toStrictEqual(false);
  expect(default_constructed.italic).toStrictEqual(false);
  expect(default_constructed.size).toStrictEqual(0);

  const optional_style: wbg.TextStyle = wbg.TextStyle.optional_new({
    italic: true,
    bold: false,
    size: 0,
  });
  expect(optional_style.bold).toStrictEqual(false);
  expect(optional_style.italic).toStrictEqual(true);
  expect(optional_style.size).toStrictEqual(0);
});
