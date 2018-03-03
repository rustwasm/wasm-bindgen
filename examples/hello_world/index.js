// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './hello_world';`
// will work here one day as well!
const js = import("./hello_world");

js.then(js => {
  js.greet("World!");
});
