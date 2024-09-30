cargo build --target wasm32-unknown-unknown --release
wasm-bindgen ../target/release/m.wasm --out-dir build --target web
