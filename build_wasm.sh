#!/bin/bash

#cargo build --target web -- --features serde
wasm-pack build --release --target web -- --features serde
