/* tslint:disable */
/* eslint-disable */
export function enum_echo(color: Color): Color;
export function option_enum_echo(color?: Color): Color | undefined;
export function get_name(color: Color): ColorName;
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
type ColorName = "green" | "yellow" | "red";
