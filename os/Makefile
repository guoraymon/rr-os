TARGET = riscv64gc-unknown-none-elf
BUILD_DIR = target/$(TARGET)/release
ELF = $(BUILD_DIR)/os

QEMU = qemu-system-riscv64
GDB = riscv64-unknown-elf-gdb
OBJDUMP = riscv64-unknown-elf-objdump

all: run

build:
	cargo build --release

run: build
	$(QEMU) \
		-machine virt \
		-nographic \
		-bios default \
		-kernel $(ELF) \
		-d in_asm -D qemu.log

clean:
	cargo clean

debug: build
	$(QEMU) \
		-machine virt \
		-nographic \
		-bios default \
		-kernel $(ELF) \
		-s -S

gdb:
	$(GDB) \
		-ex 'file $(ELF)' \
		-ex 'set arch riscv:rv64' \
		-ex 'target remote localhost:1234' \
		-x debug.gdb
		
dump: build
	$(OBJDUMP) -d $(ELF)

.PHONY: all build run clean debug gdb dump