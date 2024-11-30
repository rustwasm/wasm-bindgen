/* tslint:disable */
/* eslint-disable */
/**
 * Manually documented function
 *
 * @param {number} arg - This is my arg. It is mine.
 * @returns to whence I came
 */
export function docme(arg: number): number;
/**
 * Manually documented function
 *
 * @param {number} arg - This is my arg. It is mine.
 * @returns to whence I came
 */
export function docme_skip(arg: number): number;
/**
 * Regular documentation.
 */
export function i_has_docs(arg: number): number;
/**
 * Regular documentation.
 *
 * @param [b=0] Description of `arg`.
 * @param d Another description.
 * @returns
 */
export function add(a: number, b?: number, c?: number, d?: number): number;
/**
 * ```js
 * function foo() {
 *     return 1;
 * }
 * ```
 */
export function indent_test1(arg: number): void;
/**
 * ```js
 * function foo() {
 *     return 1;
 * }
 * ```
 */
export function indent_test2(arg: number): void;
