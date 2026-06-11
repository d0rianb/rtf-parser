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
git commit -m "Release $version"
git tag -a "$version" -m "Release $version"
git push origin master "$version"

echo "Publish to NPM"
wasm-pack publish

echo "Publish to Cargo"
cargo publish
