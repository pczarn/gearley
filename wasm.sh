#!/usr/bin/env bash

date > gearley-wasm/public/date.txt
wasm-pack build --target web -d public/pkg gearley-wasm/
