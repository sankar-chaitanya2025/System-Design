.PHONY: all setup wasm dev build test clean

all: build

setup:
	rustup target add wasm32-unknown-unknown
	npm install

wasm:
	wasm-pack build crates/sim-wasm --target web --out-dir ../../apps/web/src/wasm/pkg

dev:
	npm run dev

build: wasm
	npm run build

test:
	cargo test --workspace

clean:
	cargo clean
	rm -rf apps/web/dist apps/web/src/wasm/pkg node_modules
