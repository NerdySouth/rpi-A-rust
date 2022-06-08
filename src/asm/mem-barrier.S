@ call to sync across different GPIO devices.
.globl _dev_barrier
_dev_barrier:
    b _dsb

/* Data memory barrier */
.globl _dmb
_dmb:
    mov r0, #0
    mcr p15, 0, r0, c7, c10, 5
    bx lr

/*
 * used to be called "drain write buffer"
 * includes all cache operations.  is a superset (>) DMB
 */
.globl _dsb
_dsb:
	mcr p15, 0, r0, c7, c10, 4
	bx lr
