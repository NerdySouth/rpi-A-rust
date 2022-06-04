@ linker hack so this will be linked to at start of  .text
.section ".text.boot"

@ this is where we start.  if you look in any '.list' file, you 
@ better see code at the first addresses (in our case: starting
@ at 0x8000).
.globl _start
_start:
    @ force the mode to be SUPER.
    mov r1,  #19 @super mode
    orr r1,r1, #(1<<7)    @ disable interrupts.
    msr cpsr, r1
    mov r0, #0
    mcr p15, 0, r0, c15, c12, 0 // init cycle counter
    mcr p15, 0, r2, c7, c5, 4   // Prefetch flush

    mov sp, 0x8000000			@ initialize stack pointer
    mov fp, #0          @ clear frame pointer reg.  don't think needed.
	bl _rmain
