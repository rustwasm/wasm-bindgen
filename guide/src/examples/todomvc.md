# TODO MVC using wasm-bingen and web-sys

[View full source code][code] or [view the compiled example online][online]

[online]: https://rustwasm.github.io/wasm-bindgen/exbuild/todomvc/
[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/todomvc

[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) and [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) coded [TODO MVC](https://todomvc.com/)

The code was rewritten from the [ES6 version](http://todomvc.com/examples/vanilla-es6/).

The core differences are:
- Having an [Element wrapper](/src/element.rs) that takes care of dyn and into refs in web-sys,
- A [Scheduler](/src/scheduler.rs) that allows Controller and View to communicate to each other by emulating something similar to the JS event loop.


## Size

The size of the project hasn't undergone much work to make it optimised yet.

- ~96kb release build
- ~76kb optimised with binaryen
- ~28kb brotli compressed
