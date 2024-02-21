bind:
	@echo "Building project artifacts."
	forge bind --via-ir --bindings-path kit/src/bindings --contracts src/ --skip-cargo-toml --module