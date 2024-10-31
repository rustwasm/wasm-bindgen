/* tslint:disable */
/* eslint-disable */
export function echo_u8(a: number): number;
export function echo_i8(a: number): number;
export function echo_u16(a: number): number;
export function echo_i16(a: number): number;
export function echo_u32(a: number): number;
export function echo_i32(a: number): number;
export function echo_u64(a: bigint): bigint;
export function echo_i64(a: bigint): bigint;
export function echo_usize(a: number): number;
export function echo_isize(a: number): number;
export function echo_f32(a: number): number;
export function echo_f64(a: number): number;
export function echo_bool(a: boolean): boolean;
export function echo_char(a: string): string;
export function echo_string(a: string): string;
export function echo_vec_u8(a: Uint8Array): Uint8Array;
export function echo_struct(a: Foo): Foo;
export function echo_vec_struct(a: (Foo)[]): (Foo)[];
export function echo_option_u8(a?: number | null): number | undefined;
export function echo_option_i8(a?: number | null): number | undefined;
export function echo_option_u16(a?: number | null): number | undefined;
export function echo_option_i16(a?: number | null): number | undefined;
export function echo_option_u32(a?: number | null): number | undefined;
export function echo_option_i32(a?: number | null): number | undefined;
export function echo_option_u64(a?: bigint | null): bigint | undefined;
export function echo_option_i64(a?: bigint | null): bigint | undefined;
export function echo_option_usize(a?: number | null): number | undefined;
export function echo_option_isize(a?: number | null): number | undefined;
export function echo_option_f32(a?: number | null): number | undefined;
export function echo_option_f64(a?: number | null): number | undefined;
export function echo_option_bool(a?: boolean | null): boolean | undefined;
export function echo_option_char(a?: string | null): string | undefined;
export function echo_option_string(a?: string | null): string | undefined;
export function echo_option_vec_u8(a?: Uint8Array | null): Uint8Array | undefined;
export function echo_option_struct(a?: Foo | null): Foo | undefined;
export function echo_option_vec_struct(a?: (Foo)[] | null): (Foo)[] | undefined;
export class Foo {
  free(): void;
}