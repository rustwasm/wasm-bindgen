// For more comments about what's going on here, check out the `hello_world`
// example
const rust = import("./add");
rust.then(m => alert('1 + 2 = ' + m.add(1, 2)));
