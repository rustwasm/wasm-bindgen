import * as wbg from "../pkg/typescript_tests";
import { expect, jest, test } from "@jest/globals";

test("member function (void) -> void", () => {
  const a = new wbg.A();
  wbg.A.other();
  a.foo();
  a.free();
  expect(() => {
    a.ret_bool();
  }).toThrow(/null pointer passed to rust/);
});

test("function with parameters", () => {
  const a = new wbg.A();
  const b: boolean = a.ret_bool();
  expect(b).toStrictEqual(true);
  a.take_bool(b);
  a.take_many(b, 1, 2);
});
