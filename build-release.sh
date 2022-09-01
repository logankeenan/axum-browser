cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/axum_browser.wasm --out-dir ./out --target web
