#!/bin/bash
(
    cd frontend
    wasm-pack build --target web --out-name wasm --out-dir ../static
)

(
    cd backend
    cargo run
)
