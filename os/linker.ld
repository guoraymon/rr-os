OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS {
    . = 0x80200000;

    __kernel_start = .;
    .text : ALIGN(4096) {
        __text_start = .;
        KEEP(*(.text.boot))
        *(.text .text.*)
        __text_end = .;
    }

    .rodata : ALIGN(4096) {
        __rodata_start = .;
        *(.rodata .rodata.*)
        __rodata_end = .;
    }

    .data : ALIGN(4096) {
        __data_start = .;
        *(.data .data.*)
        __data_end = .;
    }

    .bss (NOLOAD) : ALIGN(4096) {
        __bss_start = .;

        . = ALIGN(4096);
        __boot_stack_start = .;
        . += 4096;
        __boot_stack_top = .;

        . = ALIGN(4096);
        __kernel_stack_start = .;
        . += 4096;
        __kernel_stack_top = .;

        . = ALIGN(4096);
        __user_stack_start = .;
        . += 4096;
        __user_stack_top = .;

        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }
    __kernel_end = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}