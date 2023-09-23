cargo build --release --features runtime-benchmarks

./target/release/node-template benchmark pallet \
--chain dev --pallet pallet_dao \
--extrinsic "*" --steps=50 --repeat=20 \
--wasm-execution=compiled \
--output pallets/dao/src/weights.rs \
--template .maintain/frame-weight-template.hbs
