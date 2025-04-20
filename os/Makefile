all: build run

build:
	cargo build --release && \
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin

run:
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios default \
		-kernel target/riscv64gc-unknown-none-elf/release/os.bin
