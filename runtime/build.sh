cargo build --release
cp ../runtime/target/i686-pc-windows-gnu/release/amodus_runtime.dll ../injector/
cd ../injector
cargo run
cd ../runtime
echo "Done!"
