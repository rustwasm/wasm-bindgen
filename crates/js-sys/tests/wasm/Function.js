// Used for `Function.rs` tests
exports.get_function_to_bind = function() {
  return function() { return this.x || 1; }
};
exports.get_value_to_bind_to = function() {
  return { x: 2 };
};
exports.call_function = function(f) {
  return f();
};
