BIN_NAME = html_parser

.PHONY: all
all: build

.PHONY: build
build:
	cargo build --release

.PHONY: run
run: build
	cargo run -- $(ARGS)

.PHONY: test
test:
	cargo test

.PHONY: format
format:
	cargo fmt -- --check

.PHONY: clippy
clippy:
	cargo clippy -- -D warnings

.PHONY: clean
clean:
	cargo clean

.PHONY: precommit
precommit: format clippy test

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build      - Build the HTML parser project"
	@echo "  run        - Run the HTML parser program"
	@echo "  test       - Run tests"
	@echo "  format     - Check code formatting"
	@echo "  clippy     - Run clippy lint checks"
	@echo "  clean      - Clean build artifacts"
	@echo "  precommit  - Run tests, format, and clippy before committing"
	@echo "  help       - Show this help message"