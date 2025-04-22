all: build binary run
build:
	cargo build --release

binary:
	riscv64-unknown-elf-objcopy -O binary target/riscv64gc-unknown-none-elf/release/sample target/riscv64gc-unknown-none-elf/release/sample.bin

run:
	qemu-riscv64 target/riscv64gc-unknown-none-elf/release/sample