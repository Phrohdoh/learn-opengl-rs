.PHONY: help check build test run
.DEFAULT_GOAL := help

help:
	@echo ""
	@echo "> make help"
	@echo "  Display this help"
	@echo ""
	@echo "> make check"
	@echo "  Check the source for compilation errors."
	@echo "  This is much faster than a full build."
	@echo ""
	@echo "> make build"
	@echo "  Build the project in the release configuration."
	@echo ""
	@echo "> make test"
	@echo "  Run unit and integration tests."
	@echo ""
	@echo "> make run"
	@echo "  Build (if necessary) then run in the release configuration."
	@echo ""

check:
	cargo check --release

build:
	cargo build --release

test:
	cargo test --release

run:
	cargo run --release
