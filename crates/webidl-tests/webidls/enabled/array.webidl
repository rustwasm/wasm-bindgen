[Constructor()]
interface TestArrays {
  DOMString strings(DOMString arg1);
  ByteString byteStrings(ByteString arg1);
  USVString usvStrings(USVString arg1);

  Float32Array f32(Float32Array a);
  Float64Array f64(Float64Array a);
  Int8Array i8(Int8Array a);
  Int16Array i16(Int16Array a);
  Int32Array i32(Int32Array a);
  Uint8Array u8(Uint8Array a);
  Uint8ClampedArray u8Clamped(Uint8ClampedArray a);
  Uint16Array u16(Uint16Array a);
  Uint32Array u32(Uint32Array a);
};
