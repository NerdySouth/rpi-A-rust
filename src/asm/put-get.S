@ keep put32/get32 here so that they are in the same location in every binary.
@ if you want to override them easily, can re-define the name in rpi-asm.h
@
@ we write them as assembly routines to make absolutely sure the compiler
@ cannot optimize them (reorder, eliminate, cache, etc.)

@
@
@ store value <val> (passed in second argument register r1) 
@ into memory address <addr> (passed in first argument 
@ register, r0).
@


.syntax unified

.globl put32
.globl PUT32
put32:
PUT32:
    str r1,[r0]     @ store r1 into address held in r0
    bx lr           @ return

.globl PUT8
PUT8:
    strb r1,[r0]
    bx lr

@ get32(addr)
@
@
@ load value of <addr> passed in first argument register (r0).
@
.globl get32
.globl GET32
get32:
GET32:
    ldr r0,[r0]     @ load address held in r0 into r0
    bx lr           @ return



@ do nothing: put the call here so the compiler cannot optimize.
.globl _nop
_nop:
    bx lr
	
@ jump to the address in r0.  we don't mess w/ link register
@ so called routine will return to our caller.
.globl BRANCHTO
BRANCHTO:
    bx r0

@ #ifdef RPI_FP_ENABLED

@ .globl m4add
@ m4add:
@     vmov s0,r0
@     vmov s1,r1
@     vmov s2,r2
@     vmov s3,r3
@     vadd.f32 s4,s0,s1
@     vadd.f32 s5,s2,s3
@     vmov r0,s5
@     bx lr

@ .globl m4add2
@ m4add2:
@     vmov s0,r0
@     vmov s1,r1
@     vmov s2,r2
@     vmov s3,r3
@     vadd.f32 s4,s0,s1
@     vadd.f32 s5,s2,s3
@     vmov r0,s4
@     bx lr

@ .globl m4vmov
@ m4vmov:
@     vmov s4,r0
@     vmov s5,r0
@     bx lr

@ #endif

