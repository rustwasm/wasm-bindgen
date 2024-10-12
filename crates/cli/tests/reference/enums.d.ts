/* tslint:disable */
/* eslint-disable */
/**
 * @param {Color} color
 * @returns {Color}
 */
export function enum_echo(color: Color): Color;
/**
 * @param {Color | undefined | null} [color]
 * @returns {Color | undefined}
 */
export function option_enum_echo(color?: Color | null): Color | undefined;
/**
 * @param {Color} color
 * @returns {ColorName}
 */
export function get_name(color: Color): ColorName;
/**
 * @param {any | undefined | null} [color]
 * @returns {any | undefined}
 */
export function option_string_enum_echo(color?: any | null): any | undefined;
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
