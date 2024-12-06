/* tslint:disable */
/* eslint-disable */
export function enum_echo(color: Color): Color;
export function option_enum_echo(color?: Color): Color | undefined;
export function get_name(color: Color): ColorName;
export function option_string_enum_echo(color?: ColorName): ColorName | undefined;
export function use_used(arg0: NoExportButUsedStringEnum, arg1: NoExportButUsedEnum): void;
export function option_order(order?: Ordering): Ordering | undefined;
/**
 * A color.
 */
export enum Color {
  /**
   * Green as a leaf.
   */
  Green = 0,
  /**
   * Yellow as the sun.
   */
  Yellow = 1,
  /**
   * Red as a rose.
   */
  Red = 2,
}
export enum ImplicitDiscriminant {
  A = 0,
  B = 1,
  C = 42,
  D = 43,
}
enum NoExportButUsedEnum {
  Foo = 0,
  Bar = 1,
}
/**
 * A C-style enum with negative discriminants.
 */
export enum Ordering {
  Less = -1,
  Equal = 0,
  Greater = 1,
}
export enum PrivateEnum {
  Foo = 0,
  Bar = 1,
}
/**
 * The name of a color.
 */
export type ColorName = "green" | "yellow" | "red";
/**
 * An unused string enum.
 */
export type FooBar = "foo" | "bar";
type NoExportButUsedStringEnum = "foo" | "bar";
export type PrivateStringEnum = "foo" | "bar";
