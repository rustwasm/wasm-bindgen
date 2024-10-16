/* tslint:disable */
/* eslint-disable */
export class Foo {
  free(): void;
  x: number;
  y?: number;
  z?: number;
  readonly lone_getter: number | undefined;
  set lone_setter(value: number | undefined);
  /**
   * You will only read numbers.
   */
  get weird(): number;
  /**
   * But you must write strings.
   *
   * Yes, this is totally fine in JS.
   */
  set weird(value: string | undefined);
  /**
   * There can be static getters and setters too, and they can even have the
   * same name as instance getters and setters.
   */
  static x?: boolean;
}
