OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS {
    . = 0x80200000;

    .text : ALIGN(4K) {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }

    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4K) {
        *(.data .data.*)
    }

    .bss (NOLOAD) : ALIGN(4K) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    .boot_stack (NOLOAD) : ALIGN(4K) {
        __boot_stack_start = .;
        . += 4096;
        __boot_stack_top = .;
    }

    .kernel_stack (NOLOAD) : ALIGN(4K) {
        __kernel_stack_start = .;
        . += 4096;
        __kernel_stack_top = .;
    }

    .user_stack (NOLOAD) : ALIGN(4K) {
        __user_stack_start = .;
        . += 4096;
        __user_stack_top = .;
    }

    /DISCARD/ : {
        *(.eh_frame)
    }
}