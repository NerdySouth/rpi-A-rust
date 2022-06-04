/*  this script tells the linker how to layout our pi executable's.  */
SECTIONS
{
    /*
     * cs107e uses this:
     *      .text 0x8000 :  { start.o(.text*)  *(.text*) } 
     * which makes linking in start.o awkward if you don't copy it into
     * each dir.
     *
     * XXX: I cannot make text.boot work if we link start into libpi.a 
     */
    .text 0x8000 :  { 
        __code_start__ = .;
        KEEP(*(.text.boot))  
        *(.text .text*) 
        __code_end__ = .;
        . = ALIGN(4);
    } > .
    . = ALIGN(4);
    __rd = .;
    .rodata __rd : ALIGN(4) {  
        KEEP(*(.rodata .rodata.*));
        . = ALIGN(4);
        } > .
    /DISCARD/ : {
        *(.ARM.exidx.*);
        }
}
