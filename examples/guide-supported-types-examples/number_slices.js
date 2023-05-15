import {
  take_number_slice_by_shared_ref,
  take_number_slice_by_exclusive_ref,
} from './guide_supported_types_examples';

take_number_slice_by_shared_ref(new Float64Array(100));
take_number_slice_by_exclusive_ref(new Uint8Array(100));
