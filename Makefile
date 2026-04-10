.PHONY: build clean test guests all release
.PHONY: build-linux build-mac build-mac-arm build-windows

GUESTS := echo to help list
WASMTGT := wasm32-wasip1

all: build guests

build:
	cargo build

release:
	cargo build --release

build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu

build-mac:
	cargo build --release --target x86_64-apple-darwin

build-mac-arm:
	cargo build --release --target aarch64-apple-darwin

build-windows:
	cargo build --release --target x86_64-pc-windows-gnu

guests:
	@mkdir -p .hither
	@for g in $(GUESTS); do \
		cd guests/$$g && cargo build --release --target $(WASMTGT) && cd ../..; \
		cp guests/$$g/target/$(WASMTGT)/release/hither-guest-$$g.wasm .hither/$$g.wasm; \
	done

test:
	cargo test

clean:
	cargo clean
	rm -rf .hither
	@for g in $(GUESTS); do cd guests/$$g && cargo clean && cd ../..; done
