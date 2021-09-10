build-wasm:
	cargo build --target wasm32-unknown-unknown --release
	mv target/wasm32-unknown-unknown/release/testgame.wasm docs/