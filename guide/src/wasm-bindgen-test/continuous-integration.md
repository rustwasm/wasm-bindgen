# Setting Up Continuous Integration with `wasm-bindgen-test`

This page contains example configurations for running `wasm-bindgen-test`-based
tests in various CI services.

Is your favorite CI service missing? [Send us a pull
request!](https://github.com/rustwasm/wasm-bindgen)

## Travis CI

```yaml
language: rust
rust: nightly

addons:
  firefox: latest
  chrome: stable

install:
  - rustup target add wasm32-unknown-unknown
  - cargo install wasm-bindgen-cli
  # Install node.js with nvm.
  - curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.8/install.sh | bash
  - source ~/.nvm/nvm.sh
  - nvm install v10.5
  # Install chromedriver.
  - curl --retry 5 -LO https://chromedriver.storage.googleapis.com/2.41/chromedriver_linux64.zip
  - unzip chromedriver_linux64.zip
  # Install geckodriver.
  - curl --retry 5 -LO https://github.com/mozilla/geckodriver/releases/download/v0.21.0/geckodriver-v0.21.0-linux64.tar.gz
  - tar xf geckodriver-v0.21.0-linux64.tar.gz

script:
  # Test in Chrome.
  - CHROMEDRIVER=$(pwd)/chromedriver cargo test --target wasm32-unknown-unknown
  # Test in Firefox.
  - GECKODRIVER=$(pwd)/geckodriver cargo test --target wasm32-unknown-unknown
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
