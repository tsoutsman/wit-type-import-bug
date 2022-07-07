cargo b --release --manifest-path wasm_component/Cargo.toml --target wasm32-unknown-unknown
wit-component target/wasm32-unknown-unknown/release/wasm_component.wasm -o wit_component.wasm --interface export.wit
cargo r --release --manifest-path wasm_runner/Cargo.toml