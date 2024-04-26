#!/bin/bash

cargo build --profile profiling --example bench
samply record -r 10000 target/profiling/examples/bench