.section .text
.global __trap
__trap:
    csrrw sp, sscratch, sp // now sp is kernel stack pointer
    addi sp, sp, -12*8
    csrr t0, sstatus
    csrr t1, sepc
    csrr t2, sscratch // t2 is user stack pointer
    sd t0, 0(sp)
    sd t1, 1*8(sp)
    sd ra, 2*8(sp)
    sd t2, 3*8(sp)
    sd a0, 4*8(sp)
    sd a1, 5*8(sp)
    sd a2, 6*8(sp)
    sd a3, 7*8(sp)
    sd a4, 8*8(sp)
    sd a5, 9*8(sp)
    sd a6, 10*8(sp)
    sd a7, 11*8(sp)
    mv a0, sp //  a0 is TrapContext
    call trap_handle

.global __restore
__restore:
    ld t0, 0(sp)
    ld t1, 1*8(sp)
    ld ra, 2*8(sp)
    ld t2, 3*8(sp) // t2 is user stack pointer
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2 // now sscratch is user stack pointer
    ld a0, 4*8(sp)
    ld a1, 5*8(sp)
    ld a2, 6*8(sp)
    ld a3, 7*8(sp)
    ld a4, 8*8(sp)
    ld a5, 9*8(sp)
    ld a6, 10*8(sp)
    ld a7, 11*8(sp)
    addi sp, sp, 12*8
    csrrw sp, sscratch, sp // now sp is user stack pointer
    sret