#![no_std]

use core::arch::asm;

use cortex_a::registers::{
    CNTHCTL_EL2, CNTVOFF_EL2, ELR_EL2, HCR_EL2, MPIDR_EL1, SPSR_EL2, SP_EL1,
};
use tock_registers::interfaces::{Readable, Writeable};

const CORE_MASK: u64 = 0b11;

#[inline(always)]
unsafe fn el2_to_el1(jumpto: fn() -> !, stack: u64) {
    // enable timer counters for EL1 (since this reg is hypervisor-only)
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);
    CNTVOFF_EL2.set(0); // no counter-timer offset

    // hypervisor configuration: EL1 should run in aarch64
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    // imagine a type of exception and then get ERET at him
    SPSR_EL2.write(
        // mask debug, SError, IRQ, FIQ.
        SPSR_EL2::D::Masked
			+ SPSR_EL2::A::Masked
			+ SPSR_EL2::I::Masked
			+ SPSR_EL2::F::Masked
		// "returning" to EL1 [handler]
			+ SPSR_EL2::M::EL1h,
    );

    // point link register to our EL1 code and let the core know where the EL1 stack is
    ELR_EL2.set(jumpto as u64);
    SP_EL1.set(stack);

    // return from our imaginary guy
    asm!("eret");
}

extern "C" {
	fn main();
}

fn el1_bootcode() -> ! {
    unsafe {
		// initialize BSS given __sbss and __ebss from linker script
		asm!(
			"ldr x0, =__sbss",
			"ldr x1, =__ebss",
			"2:",
			"cmp x0, x1",
			"b.eq 3f",
			// zero two words at a time
			"stp xzr, xzr, [x0], #0x10",
			"b 2b",
			"3:",
		);

		main();

		loop {
			asm!("wfe");
		}
	}
}

extern crate bootshim_macros as macros;
pub use macros::entry;

#[link_section = ".text.bootshim"]
#[no_mangle]
pub unsafe extern "C" fn _boot_core() -> ! {
    extern "C" {
        static __bcore_stack_start: u64;
    }

    // boot core0
    if MPIDR_EL1.get() & CORE_MASK == 0 {
        el2_to_el1(el1_bootcode, &__bcore_stack_start as *const _ as u64);
    }

    // park everything else wheee
    loop {
        asm!("wfe");
    }
}
