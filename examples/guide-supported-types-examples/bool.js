import {
  take_char_by_value,
  return_char,
  take_option_bool,
  return_option_bool,
} from './guide_supported_types_examples';

take_bool_by_value(true);

let b = return_bool();
console.log(typeof b); // "boolean"

take_option_bool(null);
take_option_bool(undefined);
take_option_bool(true);

let c = return_option_bool();
if (c == null) {
  // ...
} else {
  console.log(typeof c); // "boolean"
}
