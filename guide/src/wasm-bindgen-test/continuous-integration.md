# Setting Up Continuous Integration with `wasm-bindgen-test`

This page contains example configurations for running `wasm-bindgen-test`-based
tests in various CI services.

Is your favorite CI service missing? [Send us a pull
request!](https://github.com/rustwasm/wasm-bindgen)

## Travis CI

```yaml
language: rust
rust    : nightly

addons:
  firefox: latest
  chrome : stable

install:
  - curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

script:

  # this will test the non wasm targets if your crate has those, otherwise remove this line.
  #
  - cargo test

  - wasm-pack test --firefox --headless
  - wasm-pack test --chrome  --headless
```

Note that the wasm-pack installer will ask for confirmation before overwriting the wasm-pack binary. So if you use caching you can make sure it's not caching wasm-pack by adding this to your `travis.yml`:
```yml
# Need to cache the whole `.cargo` directory to keep .crates.toml for cargo-update to work
#
cache:
  directories:
    - /home/travis/.cargo

# But don't cache the cargo registry
# and remove wasm-pack binary to avoid the installer asking confirmation for overwriting it.
#
before_cache:
  - rm -rf /home/travis/.cargo/registry
  - rm -rf /home/travis/.cargo/bin/wasm-pack
```

## AppVeyor

```yaml
install:
  - ps: Install-Product node 10
  - appveyor-retry appveyor DownloadFile https://win.rustup.rs/ -FileName rustup-init.exe
  - rustup-init.exe -y --default-host x86_64-pc-windows-msvc --default-toolchain nightly
  - set PATH=%PATH%;C:\Users\appveyor\.cargo\bin
  - rustc -V
  - cargo -V
  - rustup target add wasm32-unknown-unknown
  - cargo install wasm-bindgen-cli

build: false

test_script:
  # Test in Chrome. chromedriver is installed by default in appveyor.
  - set CHROMEDRIVER=C:\Tools\WebDriver\chromedriver.exe
  - cargo test --target wasm32-unknown-unknown
  - set CHROMEDRIVER=
  # Test in Firefox. geckodriver is also installed by default.
  - set GECKODRIVER=C:\Tools\WebDriver\geckodriver.exe
  - cargo test --target wasm32-unknown-unknown
```
