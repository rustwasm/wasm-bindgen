import {
  take_char_by_value,
  return_char,
} from './guide_supported_types_examples';

take_char_by_value('a');

let c = return_char();
console.log(typeof c); // "string"
