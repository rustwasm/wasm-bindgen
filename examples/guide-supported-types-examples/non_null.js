import {
  take_pointer_by_value,
  return_pointer,
} from './guide_supported_types_examples';
import { memory } from './guide_supported_types_examples_bg';

let ptr = return_pointer();
let buf = new Uint8Array(memory.buffer);
let value = buf[ptr];
console.log(`The byte at the ${ptr} address is ${value}`);

take_pointer_by_value(ptr);
