export function test_string_roundtrip(f) {
  const test = expected => {
    const actual = f(expected);
    if (actual === expected)
      return;
    throw new Error(`string roundtrip "${actual}" != "${expected}"`);
  };

  test('');
  test('a');
  test('💖');

  test('a longer string');
  test('a longer 💖 string');
}
