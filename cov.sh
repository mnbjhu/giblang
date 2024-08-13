#!/bin/bash

cargo clean && cargo test

grcov . \
  -s . \
  --binary-path ./target/debug/ \
  -t html \
  --branch --ignore-not-existing \
  -o ./target/debug/coverage/ \
  --ignore "src/cli/*" \
  --ignore "src/main.rs" \
  --ignore "src/check/err/*"
