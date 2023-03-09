import {
  imported_type_by_value,
  imported_type_by_shared_ref,
  return_imported_type,
  take_option_imported_type,
  return_option_imported_type,
} from './guide_supported_types_examples';

imported_type_by_value(new SomeJsType());
imported_type_by_shared_ref(new SomeJsType());

let x = return_imported_type();
console.log(x instanceof SomeJsType); // true

take_option_imported_type(null);
take_option_imported_type(undefined);
take_option_imported_type(new SomeJsType());

let y = return_option_imported_type();
if (y == null) {
  // ...
} else {
  console.log(y instanceof SomeJsType); // true
}
