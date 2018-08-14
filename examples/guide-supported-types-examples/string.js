import {
  take_string_by_value,
  return_string,
  take_option_string,
  return_option_string,
} from './guide_supported_types_examples';

take_string_by_value('hello');

let s = return_string();
console.log(typeof s); // "string"

take_option_string(null);
take_option_string(undefined);
take_option_string('hello');

let t = return_option_string();
if (t == null) {
  // ...
} else {
  console.log(typeof s); // "string"
}
