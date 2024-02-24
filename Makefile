bind:
	@echo "Building project artifacts."
	forge bind --bindings-path kit/src/bindings --contracts src/ --skip-cargo-toml --module