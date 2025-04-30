TARGET = riscv64gc-unknown-none-elf
BUILD_DIR = target/$(TARGET)/release
ELF = $(BUILD_DIR)/yield_a
BIN = $(BUILD_DIR)/yield_a.bin

QEMU = qemu-riscv64
OBJCOPY = riscv64-unknown-elf-objcopy
OBJDUMP = riscv64-unknown-elf-objdump

all: run

build:
	cargo build --release

run: build
	$(QEMU) $(ELF)

clean:
	cargo clean

bin: build
	$(OBJCOPY) --strip-all -O binary $(ELF) $(BIN)

dump: build
	$(OBJDUMP) -d $(ELF)

.PHONY: all build run clean bin dump