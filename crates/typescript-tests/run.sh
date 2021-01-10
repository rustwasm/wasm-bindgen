#!/bin/sh

set -ex

cargo build --target wasm32-unknown-unknown

rm -rf pkg
mkdir pkg
cargo run -p wasm-bindgen-cli --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/debug/typescript_tests.wasm \
  --out-dir pkg \
  --typescript

mkdir pkg/web
cargo run -p wasm-bindgen-cli --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/debug/typescript_tests.wasm \
  --out-dir pkg/web \
  --target web \
  --typescript

if [ ! -d node_modules ]; then
  npm install
fi

npm run tsc

# Build using no-modules, and make sure can minimally be read by the TypeScript compiler.
mkdir pkg/no_modules
cargo run -p wasm-bindgen-cli --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/debug/typescript_tests.wasm \
  --out-dir pkg/no_modules \
  --target no-modules \
  --typescript

rm -rf src_no_modules
mkdir src_no_modules
cd src
for filename in *.ts; do
  # Create TypeScript file copies that don't use the module system.
  if grep -q "as wasm" "$filename"
  then
    echo "Ignoring $filename as it uses the *.wasm.d.ts file."
  else
    path="../src_no_modules/$filename"
    # Copy over everything but the import statements.
    grep -v -w "import" "$filename" > "$path"
    # Then replace "wbg."/"wbg "/"wbg[" with "wasm_bindgen" (the namespace the .d.ts file uses).
    sed -i "s/wbg\./wasm_bindgen./g" "$path"
    sed -i "s/wbg /wasm_bindgen /g" "$path"
    sed -i "s/wbg\[/wasm_bindgen[/g" "$path"
  fi
done
cd ..

npm run tsc -- -p no_modules.tsconfig.json
