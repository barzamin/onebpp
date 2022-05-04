`bootshim` boots core0 (bcore) and parks everything else.
switches from EL2 to EL1, jumps to `el1_bootcode` w/ stack `SP_EL1 <- __bcore_stack_start`,
`.bss` gets zeroed and jump to `kmain` in `kernel/` (`onebpp`).
