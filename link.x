ENTRY(_boot_core);

__load_addr = 0x80000;
stack_size  = 0x80000; /* 512KiB */

SECTIONS {
    . = __load_addr;
    .text :
    {
        KEEP(*(.text.bootshim)) /* needs to start at exactly __load_addr */

        *(.text*)
    }

    .rodata ALIGN(16) :
    {
        *(.rodata .rodata.*)
    }

    /* we don't actually support GOTs b/c dynarel in the kernel would be insane.
       just making sure they dont trample stuff. */
    .got (NOLOAD) :
    {
        KEEP(*(.got .got.*));
    }

    .data ALIGN(16) :
    {
        *(.data .data.*)
    }

    /* align to 16 bytes b/c we zero w/ paired stores */
    .bss ALIGN(16) (NOLOAD) :
    {
        __sbss = .;
        *(.bss .bss.*)
        *(COMMON)
        __ebss = .;
    }

    .stack ALIGN(16) (NOLOAD) :
    {
        __bcore_stack_end = .;
        . += stack_size;
        __bcore_stack_start = .;
    }

}
