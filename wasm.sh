#!/usr/bin/env bash

date > gearley-wasm/public/date.txt
RUST_BACKTRACE=1 wasm-pack build --target web --dev -d vite/src/assets/pkg/ gearley-wasm/
