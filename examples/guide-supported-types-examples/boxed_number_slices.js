import {
  take_boxed_number_slice_by_value,
  return_boxed_number_slice,
  take_option_boxed_number_slice,
  return_option_boxed_number_slice,
} from './guide_supported_types_examples';

take_boxed_number_slice_by_value(new Uint8Array(100));

let x = return_boxed_number_slice();
console.log(x instanceof Uint32Array); // true

take_option_boxed_number_slice(null);
take_option_boxed_number_slice(undefined);
take_option_boxed_number_slice(new Int16Array(256));

let y = return_option_boxed_number_slice();
if (y == null) {
  // ...
} else {
  console.log(x instanceof Int32Array); // true
}
