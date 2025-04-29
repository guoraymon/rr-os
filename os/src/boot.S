.section .text.boot
.global _start
_start:
    # 设置栈顶地址
    la sp, __boot_stack_top

    # 清 BSS 段
    la a0, __bss_start
    la a1, __bss_end
1:
    beq a0, a1, 2f         # 如果 a0 == a1，清空完成
    sb zero, 0(a0)         # *a0 = 0 （按字节写入）
    addi a0, a0, 1         # a0 += 1
    j 1b
2:
    call rust_main
3:
    wfi                    # 等待中断
    j 3b