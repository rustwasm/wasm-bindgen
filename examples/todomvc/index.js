// For more comments about what's going on here, check out the `hello_world`
// example
const rust = import('./pkg');

rust
  .then(m => m.run())
  .catch(console.error);
