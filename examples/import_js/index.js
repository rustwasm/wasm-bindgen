// For more comments about what's going on here, check out the `hello_world`
// example
const rust = import('./import_js');

rust.then(m => m.run());
