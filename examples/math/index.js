// For more comments about what's going on here, check out the `hello_world`
// example
const rust = import("./math");

rust.then(m => {
  m.run();
  console.log('and in JS the answers look like:');
  console.log(`Math.log2(10.0) = ${Math.log2(10.0)}`);
  console.log(`Math.sin(1.2) = ${Math.sin(1.2)}`);
});
