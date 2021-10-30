__pi_1_load = 0x8000;  

ENTRY(__pi_1_load)

SECTIONS
{
    . =  __pi_1_load;

    .text :
    {
        KEEP(*(.text._start));
        *(.text*);
    }

    .rodata : { *(.rodata*); }
    .got    : { *(.got);     }
    .data   : { *(.data*); }

    .bss    :
    { 
        __bss_start = .;
        *(.bss*)
        __bss_end = .;      
    }
}