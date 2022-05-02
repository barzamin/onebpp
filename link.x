__load_addr = 0x80000;

SECTIONS {
    . = __load_addr;

    .text :
    {
        KEEP(*(text._strap))

    }
}
