TARGET := armv7a-none-eabi

default:
	cargo build --target $(TARGET)

clippy:
	cargo clippy --target $(TARGET)

check:
	cargo check --target $(TARGET)

fmt:
	cargo fmt

ready: clippy fmt
	git pull
	cargo package --allow-dirty

clean:
	cargo clean
