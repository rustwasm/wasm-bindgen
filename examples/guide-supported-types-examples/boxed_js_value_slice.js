import {
  take_boxed_js_value_slice_by_value,
  return_boxed_js_value_slice,
  take_option_boxed_js_value_slice,
  return_option_boxed_js_value_slice,
} from './guide_supported_types_examples';

take_boxed_js_value_slice_by_value([null, true, 2, {}, []]);

let values = return_boxed_js_value_slice();
console.log(values instanceof Array); // true

take_option_boxed_js_value_slice(null);
take_option_boxed_js_value_slice(undefined);
take_option_boxed_js_value_slice([1, 2, 3]);

let maybeValues = return_option_boxed_js_value_slice();
if (maybeValues == null) {
  // ...
} else {
  console.log(maybeValues instanceof Array); // true
}
