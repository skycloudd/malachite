#!/bin/bash
rustup update &&
cd malachite-base &&
cargo fmt &&
cargo check --all-targets &&
cd ../malachite-base-test-util &&
cargo fmt &&
cargo check --all-targets &&
cd ../malachite-nz &&
cargo fmt &&
cargo check --all-targets --features 32_bit_limbs --features serde &&
cargo check --all-targets --features serde &&
cd ../malachite-nz-test-util &&
cargo fmt &&
cargo check --all-targets &&
cd ../malachite-bench &&
cargo fmt &&
cd ../malachite-test &&
cargo fmt &&
cargo check --all-targets --features 32_bit_limbs &&
cargo check --all-targets &&
cd .. &&
python3 additional-lints.py &&
cd malachite-base &&
cargo update &&
cargo fmt &&
cargo clippy --tests &&
cargo test --release &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps &&
cargo build --release --target wasm32-unknown-unknown &&
cd ../malachite-base-test-util &&
cargo update &&
cargo fmt &&
cargo clippy &&
cd ../malachite-nz &&
cargo update &&
cargo fmt &&
cargo clippy --tests --features 32_bit_limbs --features serde &&
cargo test --release --features 32_bit_limbs --features fail_on_untested_path --features serde &&
cargo test --release --test lib --features fail_on_untested_path --features serde && # Skip doctests when in 64-bit mode
python extra-tests.py &&
cargo clippy --tests --features serde &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps &&
cargo build --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
cargo build --release --target wasm32-unknown-unknown &&
cd ../malachite-nz-test-util &&
cargo update &&
cargo fmt &&
cargo clippy &&
cd ../malachite-bench &&
cargo update &&
cargo fmt &&
cd ../malachite-test &&
cargo update &&
cargo fmt &&
cargo test --release --features 32_bit_limbs --features fail_on_untested_path &&
cargo test --release --features fail_on_untested_path &&
cd .. &&
python3 additional-lints.py
