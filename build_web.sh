#!/bin/bash

set -eux -o pipefail


wasm-pack build --target web --out-dir web/pkg --release
