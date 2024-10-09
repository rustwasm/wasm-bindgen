[Constructor()]
interface TestArrays {
  DOMString strings(DOMString arg1);
  ByteString byteStrings(ByteString arg1);
  USVString usvStrings(USVString arg1);

  Float32Array f32([AllowShared] Float32Array a);
  Float64Array f64([AllowShared] Float64Array a);
  Int8Array i8([AllowShared] Int8Array a);
  Int16Array i16([AllowShared] Int16Array a);
  Int32Array i32([AllowShared] Int32Array a);
  Uint8Array u8([AllowShared] Uint8Array a);
  Uint8ClampedArray u8Clamped([AllowShared] Uint8ClampedArray a);
  Uint16Array u16([AllowShared] Uint16Array a);
  Uint32Array u32(Uint32Array a);
  
  readonly attribute FrozenArray<octet> octetArray;
  readonly attribute sequence<octet> octetSequence;
};
