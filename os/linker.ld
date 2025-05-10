OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS {
    . = 0x80200000;

    __kernel_start = .;
    .text : ALIGN(4K) {
        __text_start = .;
        KEEP(*(.text.boot))
        *(.text .text.*)
        __text_end = .;
    }

    .rodata : ALIGN(4K) {
        __rodata_start = .;
        *(.rodata .rodata.*)
        __rodata_end = .;
    }

    .data : ALIGN(4K) {
        __data_start = .;
        *(.data .data.*)
        __data_end = .;
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
    __kernel_end = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}