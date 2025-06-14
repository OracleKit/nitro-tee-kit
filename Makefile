.PHONY: build
build: bin build/enclave-image build/install build/ntk-host.service
	@echo "Build successful!"

build/enclave-image: build/ntk-enclave $(wildcard src/enclave/docker/*)
	rm -rf build/tmp
	mkdir build/tmp
	cp build/ntk-enclave build/tmp
	cp -r src/enclave/docker/* build/tmp
	docker build -t ntk-ubuntu build/tmp
	docker image prune -f
	rm -rf build/tmp
	docker images --digests | grep -m 1 "ntk-ubuntu" | awk '{ print $$3 }' > build/enclave-image

.PHONY: bin
bin: build/ntk-host build/ntk-enclave

build/ntk-host: $(wildcard src/host/src/*) src/host/Cargo.toml build/libntk_common.rlib | build-dirs
	cargo build --package ntk-host
	cp target/debug/ntk-host build/

build/ntk-enclave: $(wildcard src/enclave/src/*) src/enclave/Cargo.toml build/libntk_common.rlib | build-dirs
	cargo build --package ntk-enclave
	cp target/debug/ntk-enclave build/

build/libntk_common.rlib: $(wildcard src/common/src/*) src/common/Cargo.toml | build-dirs
	cargo build --package ntk-common
	cp target/debug/libntk_common.rlib build/

build/install: src/host/system/install | build-dirs
	cp src/host/system/install build/install
	chmod +x build/install

build/ntk-host.service: src/host/system/ntk-host.service | build-dirs
	cp src/host/system/ntk-host.service build/ntk-host.service

build-dirs:
	@mkdir -p build