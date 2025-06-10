cp ../target/debug/ntk-enclave .
cp -r ../src/enclave/service .
docker build . -t hello