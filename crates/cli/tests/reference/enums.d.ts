/* tslint:disable */
/* eslint-disable */
/**
 * @param {Color} color
 * @returns {Color}
 */
export function enum_echo(color: Color): Color;
/**
 * @param {Color | undefined} [color]
 * @returns {Color | undefined}
 */
export function option_enum_echo(color?: Color): Color | undefined;
/**
 * @param {Color} color
 * @returns {ColorName}
 */
export function get_name(color: Color): ColorName;
/**
 * @param {ColorName | undefined} [color]
 * @returns {ColorName | undefined}
 */
export function option_string_enum_echo(color?: ColorName): ColorName | undefined;
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
/**
 * The name of a color.
 */
export type ColorName = "green" | "yellow" | "red";
/**
 * An unused string enum.
 */
export type FooBar = "foo" | "bar";
type PrivateStringEnum = "foo" | "bar";
