#!/bin/bash

# Check if Cargo.toml exist in this directory
if [ ! -f Cargo.toml ]; then
  echo "Cargo.toml doesn't exist in this directory"
  exit 1
fi

version=$(grep '^version =' Cargo.toml | cut -d '"' -f 2)

echo "Publishing rtf-parser v$version"
echo "Build WASM released module"
./build_wasm.sh

#echo "Commiting the last changes"
git add .
git commit -m "Publishing v$version"
git push origin master

echo "Publish to NPM"
# Rename from 'rtf-parser' to 'rtf-parser-wasm' for NPM
sed -i '' 's/"name": "[^"]*"/"name": "rtf-parser-wasm"/' pkg/package.json
wasm-pack publish

echo "Publish to Cargo"
cargo publish
