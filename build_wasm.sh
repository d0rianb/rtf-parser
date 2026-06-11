#!/bin/bash

#cargo build --target web -- --features serde
wasm-pack build --release --target web

# Use a distinct package name because "rtf-parser" is already taken on NPM
sed -i '' 's/"name": "[^"]*"/"name": "rtf-parser-wasm"/' pkg/package.json
