cd proc
cargo build --release
cd ..

cd sdk
cargo build --release
cd ..

cd cli
cargo build --release
cd ..

cd runtime
./build.sh
cd ..

mkdir -p bin

cp ./runtime/target/x86_64-pc-windows-gnu/debug/amodus_runtime.dll bin

./cli/target/debug/amodus_cli new temp
cd temp
../cli/target/debug/amodus_cli build

cd ..

rm -rf temp
