import {
  take_number_slice_by_value,
  return_number_slice,
  take_option_number_slice,
  return_option_number_slice,
} from './guide_supported_types_examples';

take_number_slice_by_shared_ref(new Float64Array(100));
take_number_slice_by_exclusive_ref(new Uint8Array(100));
