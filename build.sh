cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/axum_browser.wasm --out-dir ./out --target web
