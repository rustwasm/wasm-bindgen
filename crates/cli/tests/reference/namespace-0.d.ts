/* tslint:disable */
/* eslint-disable */
/**
 * C-style enum
 */
export enum ImageFormat {
  PNG = 0,
  JPEG = 1,
  GIF = 2,
}
/**
 * String enum
 */
type Status = "success" | "failure";
export namespace ImageFormat {
  export function from_str(s: string): ImageFormat;
}
export namespace Status {
  /**
   * I have documentation.
   */
  export function from_bool(success: boolean): Status;
}
