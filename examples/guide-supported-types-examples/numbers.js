import {
  take_number_by_value,
  return_number,
  take_option_number,
  return_option_number,
} from './guide_supported_types_examples';

take_number_by_value(42);

let x = return_number();
console.log(typeof x); // "number"

take_option_number(null);
take_option_number(undefined);
take_option_number(13);

let y = return_option_number();
if (y == null) {
  // ...
} else {
  console.log(typeof y); // "number"
}
