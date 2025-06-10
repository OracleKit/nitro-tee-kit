rm -rf ./build
mkdir ./build

cp ../target/debug/ntk-enclave ./build
docker build . -t hello