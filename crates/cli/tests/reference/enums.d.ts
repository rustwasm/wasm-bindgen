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
*/
export enum Color {
  Green = 0,
  Yellow = 1,
  Red = 2,
}
