PYTHON_SRC := python/wooden-puzzle-solver.py
GO_SRC := go/wooden-puzzle-solver.go
GO_BIN := go/wooden-puzzle-solver
RUST_MANIFEST := rust/wooden-puzzle-solver/Cargo.toml

.PHONY: all python go rust wasm clean test test-python test-go test-rust

all: python go rust wasm

test: test-python test-go test-rust

python:
	python3 -m py_compile $(PYTHON_SRC)

go:
	go build -o $(GO_BIN) $(GO_SRC)

rust:
	cargo build --manifest-path $(RUST_MANIFEST)
	cargo build --manifest-path $(RUST_MANIFEST) --release

wasm:
	./scripts/build-static-wasm.sh

clean:
	rm -rf python/__pycache__ python/tests/__pycache__
	rm -f $(GO_BIN)
	cargo clean --manifest-path $(RUST_MANIFEST)
	rm -rf site/wasm
test-python:
	python3 -m unittest discover -s python/tests -p 'test_*.py'

test-go:
	cd go && GO111MODULE=off go test

test-rust:
	cargo test --manifest-path $(RUST_MANIFEST)
