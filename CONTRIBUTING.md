# Contributing

This document contains instructions on how to get this project up and running.
For more information on the architecture, design, and goals of this project
please checkout [`DESIGN.md`](DESIGN.md).

## Prerequisites

1. Rust Nightly. [Install Rust]. Once Rust is installed, run

    ```
    rustup default nightly
    ```

[install Rust]: https://www.rust-lang.org/en-US/install.html

2. The tests for this project use Node. Make sure you have node >= 8 installed,
   as that is when WebAssembly support was introduced. [Install Node].

[Install Node]: https://nodejs.org/en/

3. The tests for this project also use yarn, a package manager for Node. To install yarn, run:

    ```
    npm install yarn -g
    ```

   ... or follow other platform-specific instructions [here](https://yarnpkg.com/en/docs/install).

   Once `yarn` is installed, run it in the top level directory:

   ```
   yarn
   ```

   Finally, you can run the tests with `cargo`:

   ```
   cargo test
   ```
