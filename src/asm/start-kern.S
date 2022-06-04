@ linker hack so this will be linked to at start of  .text
.section ".text.boot"

@ this is where we start.  if you look in any '.list' file, you 
@ better see code at the first addresses (in our case: starting
@ at 0x8000).

@based on dwelch's start for bootloader.
@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@ FOR KERNEL @@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@
.globl _start
_start:
 	b skip
.space 0x200000-0x8004,0
skip:
 	mov sp,#0x08000000
 	bl _bootmain
