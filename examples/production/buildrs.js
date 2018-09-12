const fs = require('fs');
const path = require('path');
const exec = require('child_process').execSync;

const dirname = __dirname.split(path.sep).pop().replace(/-/g, '_');
const wasmOutPath = path.join(__dirname, 'src/wasm');
if (!fs.existsSync(wasmOutPath)) {
  fs.mkdirSync('./src/wasm');
}

const isRelease = process.argv.includes('release');

const buildCmd = `cargo +nightly build --target=wasm32-unknown-unknown${isRelease ? ' --release' : ''}`;
exec(buildCmd);
const wasmBindgenCmd = `cargo +nightly run --manifest-path ../../crates/cli/Cargo.toml \
  --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/${isRelease ? 'release' : 'debug'}/${dirname}.wasm --out-dir ./src/wasm
`;
/*
 * Here we're using the version of the CLI in this repository, but for external
 * usage you'd use the commented out version below
 */
// const wasmBindgenCmd = `wasm-bindgen target/wasm32-unknown-unknown/${isRelease ? 'release' : 'debug'}/${dirname}.wasm --out-dir ./src/wasm`;
exec(wasmBindgenCmd);
