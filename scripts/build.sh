set -e
cargo build --target wasm32-unknown-unknown --package canister-a --release
ic-cdk-optimizer target/wasm32-unknown-unknown/release/canister-a.wasm -o target/wasm32-unknown-unknown/release/canister-a-opt.wasm
cargo build --target wasm32-unknown-unknown --package canister-b --release
ic-cdk-optimizer target/wasm32-unknown-unknown/release/canister-b.wasm -o target/wasm32-unknown-unknown/release/canister-b-opt.wasm
cargo run -p canister-a > candid/canister-a.did
cargo run -p canister-b > candid/canister-b.did
