import {
  ExportedRustType,
  exported_type_by_value,
  exported_type_by_shared_ref,
  exported_type_by_exclusive_ref,
  return_exported_type,
} from './guide_supported_types_examples';

let rustThing = return_exported_type();
console.log(rustThing instanceof ExportedRustType); // true

exported_type_by_value(rustThing);
exported_type_by_shared_ref(rustThing);
exported_type_by_exclusive_ref(rustThing);
