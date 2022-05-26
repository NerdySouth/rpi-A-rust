@ call to sync across different GPIO devices.
.globl dev_barrier_c
dev_barrier_c:
    b dsb_c

/* Data memory barrier */
.globl dmb_c
dmb_c:
    mov r0, #0
    mcr p15, 0, r0, c7, c10, 5
    bx lr

/*
 * used to be called "drain write buffer"
 * includes all cache operations.  is a superset (>) DMB
 */
.globl dsb_c
dsb_c:
        mcr p15, 0, r0, c7, c10, 4
        bx lr
