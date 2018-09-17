const map = {
  global_no_args: () => 3,
  global_with_args: (a, b) => a + b,
  global_attribute: 'x',
};

global.get_global = () => map;

