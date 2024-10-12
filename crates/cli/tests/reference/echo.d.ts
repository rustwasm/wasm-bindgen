/* tslint:disable */
/* eslint-disable */
/**
 * @param {number} a
 * @returns {number}
 */
export function echo_u8(a: number): number;
/**
 * @param {number} a
 * @returns {number}
 */
export function echo_i8(a: number): number;
/**
 * @param {number} a
 * @returns {number}
 */
export function echo_u16(a: number): number;
/**
 * @param {number} a
 * @returns {number}
 */
export function echo_i16(a: number): number;
/**
 * @param {number} a
 * @returns {number}
 */
export function echo_u32(a: number): number;
/**
 * @param {number} a
 * @returns {number}
 */
export function echo_i32(a: number): number;
/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_u64(a: bigint): bigint;
/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_i64(a: bigint): bigint;
/**
 * @param {number} a
 * @returns {number}
 */
export function echo_usize(a: number): number;
/**
 * @param {number} a
 * @returns {number}
 */
export function echo_isize(a: number): number;
/**
 * @param {number} a
 * @returns {number}
 */
export function echo_f32(a: number): number;
/**
 * @param {number} a
 * @returns {number}
 */
export function echo_f64(a: number): number;
/**
 * @param {boolean} a
 * @returns {boolean}
 */
export function echo_bool(a: boolean): boolean;
/**
 * @param {string} a
 * @returns {string}
 */
export function echo_char(a: string): string;
/**
 * @param {string} a
 * @returns {string}
 */
export function echo_string(a: string): string;
/**
 * @param {Uint8Array} a
 * @returns {Uint8Array}
 */
export function echo_vec_u8(a: Uint8Array): Uint8Array;
/**
 * @param {Foo} a
 * @returns {Foo}
 */
export function echo_struct(a: Foo): Foo;
/**
 * @param {(Foo)[]} a
 * @returns {(Foo)[]}
 */
export function echo_vec_struct(a: (Foo)[]): (Foo)[];
/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_u8(a?: number): number | undefined;
/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_i8(a?: number): number | undefined;
/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_u16(a?: number): number | undefined;
/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_i16(a?: number): number | undefined;
/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_u32(a?: number): number | undefined;
/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_i32(a?: number): number | undefined;
/**
 * @param {bigint | undefined | null} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_u64(a?: bigint): bigint | undefined;
/**
 * @param {bigint | undefined | null} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_i64(a?: bigint): bigint | undefined;
/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_usize(a?: number): number | undefined;
/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_isize(a?: number): number | undefined;
/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_f32(a?: number): number | undefined;
/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_f64(a?: number): number | undefined;
/**
 * @param {boolean | undefined | null} [a]
 * @returns {boolean | undefined}
 */
export function echo_option_bool(a?: boolean): boolean | undefined;
/**
 * @param {string | undefined | null} [a]
 * @returns {string | undefined}
 */
export function echo_option_char(a?: string): string | undefined;
/**
 * @param {string | undefined | null} [a]
 * @returns {string | undefined}
 */
export function echo_option_string(a?: string): string | undefined;
/**
 * @param {Uint8Array | undefined | null} [a]
 * @returns {Uint8Array | undefined}
 */
export function echo_option_vec_u8(a?: Uint8Array): Uint8Array | undefined;
/**
 * @param {Foo | undefined | null} [a]
 * @returns {Foo | undefined}
 */
export function echo_option_struct(a?: Foo): Foo | undefined;
/**
 * @param {(Foo)[] | undefined | null} [a]
 * @returns {(Foo)[] | undefined}
 */
export function echo_option_vec_struct(a?: (Foo)[]): (Foo)[] | undefined;
export class Foo {
  free(): void;
}
