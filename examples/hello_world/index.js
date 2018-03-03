const js = import("./hello_world");

js.then(js => {
  js.greet("World!");
});
