import {
  take_js_value_by_value,
  take_js_value_by_shared_ref,
  return_js_value,
} from './guide_supported_types_examples';

take_js_value_by_value(42);
take_js_value_by_shared_ref('hello');

let v = return_js_value();
