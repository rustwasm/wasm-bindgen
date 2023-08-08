// For more comments about what's going on here, check out the `hello_world`
// example
const rust = import('./pkg');
const css = import('./index.css');

rust
  .then(m => m.run())
  .catch(console.error);
