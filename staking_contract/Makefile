prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd staking_contract && cargo build --release --target wasm32-unknown-unknown
	wasm-strip staking_contract/target/wasm32-unknown-unknown/release/staking_contract.wasm 2>/dev/null | true

test-only:
	cd staking_contract_tests && cargo test

copy-wasm-file-to-test:
	cp staking_contract/target/wasm32-unknown-unknown/release/*.wasm staking_contract_tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf staking_contract_tests/wasm/*.wasm
