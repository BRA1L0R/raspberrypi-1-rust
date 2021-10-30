.section .text._start

_start:
    mov sp, #0x8000
    b _kernel_init
exited: b exited

.globl data_barrier
data_barrier:
     mcr p15, 0, ip, c7, c5, 0       @ invalidate I cache
     mcr p15, 0, ip, c7, c5, 6       @ invalidate BTB
     mcr p15, 0, ip, c7, c10, 4      @ drain write buffer
     mcr p15, 0, ip, c7, c5, 4       @ prefetch flush
     mov pc, lr

.size	_start, . - _start
.type	_start, function
.global	_start
